//! The clear text of ISO 10303-21: a header of records, then numbered
//! instances, each one record or, for a complex instance, several.

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Reference(usize),
    Number(f64),
    Text(String),
    Enumeration(String),
    List(Vec<Value>),
    Typed(String, Vec<Value>),
    Unset,
    Derived,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    pub values: Vec<Value>,
}

pub struct Exchange {
    pub header: Vec<Record>,
    pub instances: BTreeMap<usize, Vec<Record>>,
}

pub fn parse(text: &str) -> Result<Exchange, String> {
    let statements = statements(text)?;
    let mut statements = statements.iter().map(|statement| statement.trim());
    let mut expect = |keyword: &str| match statements.next() {
        Some(found) if found == keyword => Ok(()),
        found => Err(format!(
            "the file should go on with {keyword}, it holds {found:?}"
        )),
    };
    expect("ISO-10303-21")?;
    expect("HEADER")?;
    let mut statements = statements.peekable();
    let mut header = Vec::new();
    while let Some(statement) = statements.next_if(|statement| *statement != "ENDSEC") {
        let mut cursor = Cursor::of(statement);
        header.push(cursor.record()?);
        cursor.end()?;
    }
    for keyword in ["ENDSEC", "DATA"] {
        match statements.next() {
            Some(found) if found == keyword => {}
            found => {
                return Err(format!(
                    "the header should close on {keyword}, not {found:?}"
                ));
            }
        }
    }
    let mut instances = BTreeMap::new();
    while let Some(statement) = statements.next_if(|statement| *statement != "ENDSEC") {
        let (number, records) = Cursor::of(statement).instance()?;
        if instances.insert(number, records).is_some() {
            return Err(format!("#{number} is defined twice"));
        }
    }
    for keyword in ["ENDSEC", "END-ISO-10303-21"] {
        match statements.next() {
            Some(found) if found == keyword => {}
            found => return Err(format!("the data should close on {keyword}, not {found:?}")),
        }
    }
    match statements.next() {
        None => Ok(Exchange { header, instances }),
        Some(more) => Err(format!("{more:?} stands past the end of the file")),
    }
}

/// The text cut at every `;` that stands outside a string, comments dropped.
fn statements(text: &str) -> Result<Vec<String>, String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();
    while let Some(char) = chars.next() {
        match char {
            '\'' => {
                current.push(char);
                loop {
                    match chars.next() {
                        Some('\'') if chars.peek() == Some(&'\'') => {
                            chars.next();
                            current.push_str("''");
                        }
                        Some('\'') => {
                            current.push('\'');
                            break;
                        }
                        Some(other) => current.push(other),
                        None => return Err("a string runs to the end of the file".into()),
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut last = ' ';
                loop {
                    match chars.next() {
                        Some('/') if last == '*' => break,
                        Some(other) => last = other,
                        None => return Err("a comment runs to the end of the file".into()),
                    }
                }
            }
            ';' => statements.push(std::mem::take(&mut current)),
            _ => current.push(char),
        }
    }
    if current.trim().is_empty() {
        Ok(statements)
    } else {
        Err(format!("{:?} is not closed by a ';'", current.trim()))
    }
}

struct Cursor {
    chars: Vec<char>,
    at: usize,
}

impl Cursor {
    fn of(statement: &str) -> Cursor {
        Cursor {
            chars: statement.chars().collect(),
            at: 0,
        }
    }

    fn statement(&self) -> String {
        self.chars.iter().collect()
    }

    fn peek(&mut self) -> Option<char> {
        while self
            .chars
            .get(self.at)
            .is_some_and(|char| char.is_whitespace())
        {
            self.at += 1;
        }
        self.chars.get(self.at).copied()
    }

    fn take(&mut self, wanted: char) -> Result<(), String> {
        match self.peek() {
            Some(found) if found == wanted => {
                self.at += 1;
                Ok(())
            }
            found => Err(format!(
                "{:?}: expected {wanted:?} at {}, found {found:?}",
                self.statement(),
                self.at
            )),
        }
    }

    fn end(&mut self) -> Result<(), String> {
        match self.peek() {
            None => Ok(()),
            Some(found) => Err(format!("{:?}: {found:?} past its end", self.statement())),
        }
    }

    fn while_in(&mut self, keeps: impl Fn(char) -> bool) -> String {
        self.peek();
        let start = self.at;
        while self.chars.get(self.at).is_some_and(|char| keeps(*char)) {
            self.at += 1;
        }
        self.chars[start..self.at].iter().collect()
    }

    fn number(&mut self) -> Result<usize, String> {
        let digits = self.while_in(|char| char.is_ascii_digit());
        digits
            .parse()
            .map_err(|_| format!("{:?}: no instance number at {}", self.statement(), self.at))
    }

    fn keyword(&mut self) -> Result<String, String> {
        let keyword =
            self.while_in(|char| char.is_ascii_alphanumeric() || char == '_' || char == '-');
        if keyword.is_empty() {
            Err(format!("{:?}: no keyword at {}", self.statement(), self.at))
        } else {
            Ok(keyword.to_ascii_uppercase())
        }
    }

    fn instance(&mut self) -> Result<(usize, Vec<Record>), String> {
        self.take('#')?;
        let number = self.number()?;
        self.take('=')?;
        let mut records = Vec::new();
        if self.peek() == Some('(') {
            self.take('(')?;
            while self.peek() != Some(')') {
                records.push(self.record()?);
            }
            self.take(')')?;
        } else {
            records.push(self.record()?);
        }
        self.end()?;
        Ok((number, records))
    }

    fn record(&mut self) -> Result<Record, String> {
        let name = self.keyword()?;
        let values = self.values()?;
        Ok(Record { name, values })
    }

    fn values(&mut self) -> Result<Vec<Value>, String> {
        self.take('(')?;
        let mut values = Vec::new();
        if self.peek() == Some(')') {
            self.take(')')?;
            return Ok(values);
        }
        loop {
            values.push(self.value()?);
            match self.peek() {
                Some(',') => self.take(',')?,
                _ => break,
            }
        }
        self.take(')')?;
        Ok(values)
    }

    fn value(&mut self) -> Result<Value, String> {
        match self.peek() {
            Some('#') => {
                self.take('#')?;
                Ok(Value::Reference(self.number()?))
            }
            Some('\'') => {
                self.at += 1;
                let mut text = String::new();
                loop {
                    match self.chars.get(self.at) {
                        Some('\'') if self.chars.get(self.at + 1) == Some(&'\'') => {
                            text.push('\'');
                            self.at += 2;
                        }
                        Some('\'') => {
                            self.at += 1;
                            return Ok(Value::Text(text));
                        }
                        Some(char) => {
                            text.push(*char);
                            self.at += 1;
                        }
                        None => return Err(format!("{:?}: an open string", self.statement())),
                    }
                }
            }
            Some('.') => {
                self.take('.')?;
                let name = self.keyword()?;
                self.take('.')?;
                Ok(Value::Enumeration(name))
            }
            Some('$') => {
                self.take('$')?;
                Ok(Value::Unset)
            }
            Some('*') => {
                self.take('*')?;
                Ok(Value::Derived)
            }
            Some('(') => Ok(Value::List(self.values()?)),
            Some(char) if char.is_ascii_digit() || char == '-' || char == '+' => {
                let text = self.while_in(|char| {
                    char.is_ascii_digit() || matches!(char, '-' | '+' | '.' | 'E' | 'e')
                });
                text.parse()
                    .map(Value::Number)
                    .map_err(|_| format!("{:?}: {text:?} is no number", self.statement()))
            }
            Some(char) if char.is_ascii_alphabetic() => {
                let name = self.keyword()?;
                Ok(Value::Typed(name, self.values()?))
            }
            found => Err(format!(
                "{:?}: no value at {}, found {found:?}",
                self.statement(),
                self.at
            )),
        }
    }
}

/// Every instance number some value of `values` names.
pub fn references(values: &[Value], into: &mut Vec<usize>) {
    for value in values {
        match value {
            Value::Reference(number) => into.push(*number),
            Value::List(inner) | Value::Typed(_, inner) => references(inner, into),
            _ => {}
        }
    }
}
