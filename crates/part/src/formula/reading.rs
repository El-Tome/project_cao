//! Reading a formula out of text: the pieces it is made of, then how they
//! group — products before sums, parentheses before either.

use crate::variables::VariableId;

use super::{Formula, Operator, Unreadable};

/// One piece of a formula as written.
#[derive(Clone, Debug, PartialEq)]
enum Piece {
    Number(f64),
    Variable(VariableId),
    Operator(Operator),
    Open,
    Close,
}

impl Piece {
    /// The character it was written with, to say where a formula went wrong.
    fn written(&self) -> Option<char> {
        match self {
            Self::Number(_) | Self::Variable(_) => None,
            Self::Operator(operator) => Some(operator.sign()),
            Self::Open => Some('('),
            Self::Close => Some(')'),
        }
    }
}

/// How a variable is named in what is being read.
pub(super) enum Naming<F: Fn(&str) -> Option<VariableId>> {
    /// By the name the user gave it.
    ByName(F),
    /// By its rank, `#0`, the way a part file writes it.
    ByRank,
}

pub(super) fn read<F: Fn(&str) -> Option<VariableId>>(
    text: &str,
    naming: Naming<F>,
) -> Result<Formula, Unreadable> {
    let text = text.trim();
    let text = text.strip_prefix('=').unwrap_or(text);
    let pieces = pieces(text, naming)?;
    if pieces.is_empty() {
        return Err(Unreadable::Empty);
    }
    let mut reader = Reader { pieces, at: 0 };
    let formula = reader.sum()?;
    match reader.next() {
        None => Ok(formula),
        Some(Piece::Close) => Err(Unreadable::Unopened),
        Some(_) => Err(Unreadable::MissingOperator),
    }
}

fn pieces<F: Fn(&str) -> Option<VariableId>>(
    text: &str,
    naming: Naming<F>,
) -> Result<Vec<Piece>, Unreadable> {
    let characters: Vec<char> = text.chars().collect();
    let mut pieces = Vec::new();
    let mut at = 0;
    while let Some(&character) = characters.get(at) {
        if character.is_whitespace() {
            at += 1;
            continue;
        }
        if let Some(operator) = Operator::signed(character) {
            pieces.push(Piece::Operator(operator));
            at += 1;
            continue;
        }
        match character {
            '(' => pieces.push(Piece::Open),
            ')' => pieces.push(Piece::Close),
            _ if starts_a_number(&characters[at..]) => {
                let (number, length) = number(&characters[at..]);
                pieces.push(Piece::Number(number));
                at += length;
                continue;
            }
            '#' if matches!(naming, Naming::ByRank) => {
                let digits: String = characters[at + 1..]
                    .iter()
                    .take_while(|character| character.is_ascii_digit())
                    .collect();
                let rank = digits
                    .parse()
                    .map_err(|_| Unreadable::StrayCharacter(character))?;
                pieces.push(Piece::Variable(VariableId(rank)));
                at += 1 + digits.len();
                continue;
            }
            _ if starts_a_name(character) => {
                let Naming::ByName(names) = &naming else {
                    return Err(Unreadable::StrayCharacter(character));
                };
                let name: String = characters[at..]
                    .iter()
                    .take_while(|character| goes_on_a_name(**character))
                    .collect();
                at += name.chars().count();
                let variable = names(&name).ok_or(Unreadable::UnknownName(name))?;
                pieces.push(Piece::Variable(variable));
                continue;
            }
            _ => return Err(Unreadable::StrayCharacter(character)),
        }
        at += 1;
    }
    Ok(pieces)
}

/// What a name may open on: a letter, or `_`. Never a digit, which would open
/// a number instead.
pub(crate) fn starts_a_name(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

/// What a name goes on with: letters, digits and `_`.
pub(crate) fn goes_on_a_name(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn is_decimal_separator(character: char) -> bool {
    character == '.' || character == ','
}

fn starts_a_number(characters: &[char]) -> bool {
    match characters {
        [first, ..] if first.is_ascii_digit() => true,
        [first, second, ..] => is_decimal_separator(*first) && second.is_ascii_digit(),
        _ => false,
    }
}

/// The number the characters open on, and how many of them it takes.
fn number(characters: &[char]) -> (f64, usize) {
    let mut written = String::new();
    let mut separated = false;
    for character in characters {
        if character.is_ascii_digit() {
            written.push(*character);
        } else if is_decimal_separator(*character) && !separated {
            separated = true;
            written.push('.');
        } else {
            break;
        }
    }
    let length = written.chars().count();
    // Only digits and one point went in, so this always reads.
    (written.parse().unwrap_or_default(), length)
}

struct Reader {
    pieces: Vec<Piece>,
    at: usize,
}

impl Reader {
    fn peek(&self) -> Option<&Piece> {
        self.pieces.get(self.at)
    }

    fn next(&mut self) -> Option<Piece> {
        let piece = self.pieces.get(self.at).cloned();
        self.at += 1;
        piece
    }

    fn sum(&mut self) -> Result<Formula, Unreadable> {
        let mut formula = self.product()?;
        while let Some(Piece::Operator(operator @ (Operator::Add | Operator::Subtract))) =
            self.peek().cloned()
        {
            self.at += 1;
            let right = self.product()?;
            formula = Formula::Combined(operator, Box::new(formula), Box::new(right));
        }
        Ok(formula)
    }

    fn product(&mut self) -> Result<Formula, Unreadable> {
        let mut formula = self.signed()?;
        while let Some(Piece::Operator(operator @ (Operator::Multiply | Operator::Divide))) =
            self.peek().cloned()
        {
            self.at += 1;
            let right = self.signed()?;
            formula = Formula::Combined(operator, Box::new(formula), Box::new(right));
        }
        Ok(formula)
    }

    fn signed(&mut self) -> Result<Formula, Unreadable> {
        match self.peek() {
            Some(Piece::Operator(Operator::Subtract)) => {
                self.at += 1;
                Ok(match self.signed()? {
                    Formula::Number(number) => Formula::Number(-number),
                    other => Formula::Negative(Box::new(other)),
                })
            }
            Some(Piece::Operator(Operator::Add)) => {
                self.at += 1;
                self.signed()
            }
            _ => self.single(),
        }
    }

    fn single(&mut self) -> Result<Formula, Unreadable> {
        match self.next() {
            Some(Piece::Number(number)) => Ok(Formula::Number(number)),
            Some(Piece::Variable(variable)) => Ok(Formula::Variable(variable)),
            Some(Piece::Open) => {
                let inside = self.sum()?;
                match self.next() {
                    Some(Piece::Close) => Ok(inside),
                    None => Err(Unreadable::Unclosed),
                    Some(_) => Err(Unreadable::MissingOperator),
                }
            }
            found => Err(Unreadable::MissingValue(
                found.as_ref().and_then(Piece::written),
            )),
        }
    }
}
