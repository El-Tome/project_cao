//! A case run through the kernel, and held to every rule at every step.

use cao_solid::Body;
use cao_solid::soundness::{
    Flaw, Lines, NEAR, Silence, Spans, Triangle, closed, enclosed, reach, repeatable, uncrossed,
};
use glam::DVec3;

use super::{Case, Leaf, Mode};

/// How many lines of measure cross a case each way. Enough that a piece of
/// matter the size of one cell cannot go missing unseen, few enough that the
/// measure costs less than the boolean it checks.
const LINES: usize = 48;

/// How far a prism may land from its area times its height, as a fraction of
/// that volume: rounding, and nothing else.
const PRISM: f64 = 1e-9;

/// Whether a case keeps every rule: every solid it raises, and the part after
/// every one of its steps.
///
/// A case with a leaf that is no solid — a shrunk case can round a rectangle
/// flat — holds nothing and breaks nothing. A leaf that is one and that the
/// kernel declines to raise is a kernel giving no answer.
pub fn check(case: &Case) -> Result<(), Flaw> {
    if !case.leaves().all(Leaf::is_solid) {
        return Ok(());
    }
    let Some(leaves) = raised(case) else {
        return Err(Flaw::NoAnswer(Silence::Refused));
    };
    for (leaf, solid) in case.leaves().zip(&leaves) {
        let triangles = solid.triangles();
        sound(&triangles)?;
        kept_its_promise(leaf, &triangles)?;
    }

    let region = span(&leaves);
    let lines = Lines::across(region.0, region.1, LINES);
    let mut body = leaves[0].clone();
    let mut promised = lines.inside(&body.triangles());
    for (step, tool) in case.steps.iter().zip(&leaves[1..]) {
        let reach = lines.inside(&tool.triangles());
        body = combined(&body, tool, step.mode);
        promised = promised
            .iter()
            .zip(&reach)
            .map(|(before, tool)| match step.mode {
                Mode::Add => before.union(tool),
                Mode::Cut => before.without(tool),
            })
            .collect::<Vec<Spans>>();

        let triangles = body.triangles();
        sound(&triangles)?;
        lines.compare(&promised, &lines.inside(&triangles))?;
        within_reach(region, &lines, &promised, &triangles)?;
    }

    let again = raised(case).map(|leaves| replayed(case, &leaves));
    repeatable(&body.triangles(), &again.unwrap_or_default().triangles())
}

/// Checks a case and ends the test on the first rule it breaks, with the case
/// printed beside the flaw.
pub fn holds(case: &Case) {
    if let Err(flaw) = check(case) {
        panic!("{:?} broke the rule: {flaw:?}\n{case}", flaw.rule());
    }
}

fn raised(case: &Case) -> Option<Vec<Body>> {
    case.leaves().map(Leaf::solid).collect()
}

fn combined(body: &Body, tool: &Body, mode: Mode) -> Body {
    match mode {
        Mode::Add => body.union(tool),
        Mode::Cut => body.difference(tool),
    }
}

fn replayed(case: &Case, leaves: &[Body]) -> Body {
    case.steps
        .iter()
        .zip(&leaves[1..])
        .fold(leaves[0].clone(), |body, (step, tool)| {
            combined(&body, tool, step.mode)
        })
}

fn sound(triangles: &[Triangle]) -> Result<(), Flaw> {
    closed(triangles)?;
    uncrossed(triangles)
}

/// Whether a solid raised on its own encloses what arithmetic promised for
/// its leaf.
pub fn kept_its_promise(leaf: &Leaf, triangles: &[Triangle]) -> Result<(), Flaw> {
    let (promised, loss) = leaf.promise();
    let got = enclosed(triangles);
    let rounding = PRISM * promised.max(1.0);
    if got < promised - loss - rounding || got > promised + rounding {
        return Err(Flaw::Volume {
            promised,
            enclosed: got,
            worst: None,
        });
    }
    Ok(())
}

/// Whether a result stays inside the box its inputs span: matter found outside
/// it was promised nowhere, and no line of measure passes there to see it.
pub fn within_reach(
    (low, high): (DVec3, DVec3),
    lines: &Lines,
    promised: &[Spans],
    triangles: &[Triangle],
) -> Result<(), Flaw> {
    let room = NEAR * reach(triangles);
    let outside = triangles
        .iter()
        .flatten()
        .any(|corner| corner.cmplt(low - room).any() || corner.cmpgt(high + room).any());
    if outside {
        return Err(Flaw::Volume {
            promised: lines.volume(promised),
            enclosed: enclosed(triangles),
            worst: None,
        });
    }
    Ok(())
}

fn span(leaves: &[Body]) -> (DVec3, DVec3) {
    leaves
        .iter()
        .filter_map(Body::bounds)
        .reduce(|(low, high), (other_low, other_high)| (low.min(other_low), high.max(other_high)))
        .unwrap_or((DVec3::ZERO, DVec3::ONE))
}
