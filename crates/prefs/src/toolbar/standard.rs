//! The arrangement the toolbar has before anybody rearranges it.
//!
//! Data rather than behaviour, and the one place a new button has to be
//! listed for it to appear on a fresh installation.

use super::{Edge, Item, ToolbarLayout};
use crate::command::Command;

impl Default for ToolbarLayout {
    fn default() -> Self {
        use Command as C;
        Self {
            edge: Edge::Top,
            show_logo: false,
            logo_text: "CAO".to_string(),
            show_labels: true,
            items: vec![
                Item::group(
                    "sketch",
                    vec![
                        Item::Command(C::NewSketch),
                        Item::Separator,
                        Item::group(
                            "drawing",
                            vec![
                                Item::Command(C::ToolSelect),
                                Item::Command(C::ToolLine),
                                Item::Command(C::ToolLineSymmetric),
                                Item::Command(C::ToolRectangle),
                                Item::Command(C::ToolCircle),
                                Item::group(
                                    "circles",
                                    vec![
                                        Item::Command(C::CircleCenter),
                                        Item::Command(C::CircleTwoPoints),
                                        Item::Command(C::CircleThreePoints),
                                        Item::Command(C::CircleTwoTangents),
                                        Item::Command(C::CircleThreeTangents),
                                    ],
                                ),
                                Item::Command(C::ToolArc),
                                Item::group(
                                    "arcs",
                                    vec![
                                        Item::Command(C::ArcByCenter),
                                        Item::Command(C::ArcByEnds),
                                    ],
                                ),
                                Item::Command(C::ToolPoint),
                                Item::Command(C::ToolDimension),
                                Item::Command(C::ToolTrim),
                                Item::Command(C::ToolSplit),
                                Item::Command(C::ToolChamfer),
                                Item::group(
                                    "chamfers",
                                    vec![
                                        Item::Command(C::ChamferEqual),
                                        Item::Command(C::ChamferAngled),
                                        Item::Command(C::ChamferSided),
                                    ],
                                ),
                                Item::Command(C::ToggleConstruction),
                                // A third level opens as a menu rather than
                                // being spread out: nine rules laid on the bar
                                // would push everything else off it.
                                Item::group(
                                    "constraints",
                                    vec![
                                        Item::Command(C::RulePerpendicular),
                                        Item::Command(C::RuleParallel),
                                        Item::Command(C::RuleEqual),
                                        Item::Command(C::RuleCoincident),
                                        Item::Command(C::RuleCollinear),
                                        Item::Command(C::RuleTangent),
                                        Item::Command(C::RuleMidpoint),
                                        Item::Command(C::RuleFixed),
                                        Item::Command(C::RuleConcentric),
                                    ],
                                ),
                            ],
                        ),
                        Item::Command(C::RecenterOnSketch),
                        Item::Command(C::FinishSketch),
                        Item::Separator,
                        Item::group("edit", vec![Item::Command(C::Undo), Item::Command(C::Redo)]),
                    ],
                ),
                Item::group(
                    "extrusion",
                    vec![
                        Item::Command(C::ExtrusionAdd),
                        Item::Command(C::ExtrusionCut),
                        Item::Separator,
                        Item::Command(C::ExtrusionStraight),
                        Item::Command(C::ExtrusionRevolution),
                    ],
                ),
            ],
        }
    }
}
