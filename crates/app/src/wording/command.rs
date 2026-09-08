use cao_prefs::{Command, CommandFamily};

/// The only place a `Command` is turned into a name.
///
/// The palette, the toolbar and the shortcut list all read from here, so a
/// command is called the same thing wherever the user meets it.
pub fn label(command: Command) -> &'static str {
    match command {
        Command::NewSketch => "Nouvelle esquisse",
        Command::FinishSketch => "Terminer",
        Command::RecenterOnSketch => "Recadrer",
        Command::Undo => "Annuler",
        Command::Redo => "Rétablir",
        Command::ToolSelect => "Sélection",
        Command::ToolLine => "Ligne",
        Command::ToolRectangle => "Rectangle",
        Command::ToolCircle => "Cercle",
        Command::ToolPoint => "Point",
        Command::ToolDimension => "Cote",
        Command::CircleCenter => "Centre et diamètre",
        Command::CircleTwoPoints => "Deux points du bord",
        Command::CircleThreePoints => "Deux points puis le centre",
        Command::CircleTwoTangents => "Tangent à deux droites",
        Command::CircleThreeTangents => "Tangent à trois droites",
        Command::DimensionAuto => "Cote intelligente",
        Command::DimensionPointToPoint => "Cote point à point",
        Command::DimensionLength => "Cote de trait",
        Command::DimensionAngle => "Cote d'angle",
        Command::DimensionRadius => "Cote de rayon",
        Command::RulePerpendicular => "Perpendiculaire",
        Command::RuleParallel => "Parallèle",
        Command::RuleEqual => "Égalité",
        Command::RuleCoincident => "Coïncidence",
        Command::RuleCollinear => "Colinéaire",
        Command::RuleTangent => "Tangence",
        Command::RuleMidpoint => "Milieu",
        Command::RuleFixed => "Fixe",
        Command::RuleConcentric => "Concentrique",
        Command::ExtrusionAdd => "Ajout de matière",
        Command::ExtrusionCut => "Enlèvement de matière",
        Command::ExtrusionStraight => "Extrusion droite",
        Command::ExtrusionRevolution => "Révolution",
        Command::ExtrusionApply => "Appliquer",
        Command::ExtrusionCancel => "Annuler l'extrusion",
        Command::ToggleHistory => "Panneau Historique",
        Command::ToggleToolbarDocked => "Ancrer / détacher la barre",
        Command::OpenSettings => "Préférences",
        Command::BackToMenu => "Accueil",
    }
}

/// The one line of help shown when the pointer rests on the command.
pub fn hint(command: Command) -> &'static str {
    match command {
        Command::NewSketch => "Choisir un plan pour commencer un dessin",
        Command::FinishSketch => "Terminer l'esquisse et proposer de l'extruder",
        Command::RecenterOnSketch => "Se replacer face au plan de l'esquisse",
        Command::Undo => "Reculer d'une étape dans l'historique",
        Command::Redo => "Avancer d'une étape dans l'historique",
        Command::ToolSelect => "Cliquer-glisser un point pour le déplacer",
        Command::ToolLine => "Clics successifs, Échap pour terminer la chaîne",
        Command::ToolRectangle => "Deux clics : deux coins opposés",
        Command::ToolCircle => "Deux clics : centre puis rayon",
        Command::ToolPoint => "Un clic pose un point",
        Command::ToolDimension => "Cote intelligente : cliquer ce qu'on veut mesurer",
        Command::CircleCenter => "Le centre, puis un point du bord",
        Command::CircleTwoPoints => "Deux points opposés du bord",
        Command::CircleThreePoints => "Deux points du bord, puis le centre sur leur médiatrice",
        Command::CircleTwoTangents => "Deux droites, puis le centre sur leur bissectrice",
        Command::CircleThreeTangents => "Trois droites : le cercle inscrit entre elles",
        Command::DimensionAuto => "Mesure ce qui est sous le curseur",
        Command::DimensionPointToPoint => "Deux points, reliés ou non",
        Command::DimensionLength => "Un trait, mesuré sur toute sa longueur",
        Command::DimensionAngle => "Deux traits qui se touchent, ou un trait et un axe",
        Command::DimensionRadius => "Un cercle",
        Command::RulePerpendicular => "Deux traits, mis d'équerre",
        Command::RuleParallel => "Deux traits, gardant la même direction",
        Command::RuleEqual => "Deux traits de même longueur, ou deux cercles de même rayon",
        Command::RuleCoincident => "Un point posé sur un trait, ou deux points fondus en un",
        Command::RuleCollinear => "Deux traits couchés sur la même droite",
        Command::RuleTangent => "Un cercle et un trait qui l'effleure",
        Command::RuleMidpoint => "Un point tenu au milieu d'un trait",
        Command::RuleFixed => "Un point qui ne bouge plus de sa place",
        Command::RuleConcentric => "Deux cercles ramenés sur le même centre",
        Command::ExtrusionAdd => "Sélectionner des aires fermées, donner une hauteur",
        Command::ExtrusionCut => "Sélectionner des aires fermées, donner une profondeur",
        Command::ExtrusionStraight => "Pousser la matière perpendiculairement au plan",
        Command::ExtrusionRevolution => "Faire tourner l'aire autour d'un axe du plan",
        Command::ExtrusionApply => "Fabriquer le volume avec les aires choisies",
        Command::ExtrusionCancel => "Abandonner l'extrusion en cours",
        Command::ToggleHistory => "Afficher ou masquer l'arbre des opérations",
        Command::ToggleToolbarDocked => "Détacher la barre d'outils ou l'ancrer",
        Command::OpenSettings => "Régler l'application",
        Command::BackToMenu => "Revenir au menu de démarrage",
    }
}

/// The heading a run of the palette is listed under.
///
/// The settings screen starts a new run where the family changes, so a family
/// is written once, above the commands that belong to it.
pub fn family_heading(family: CommandFamily) -> &'static str {
    match family {
        CommandFamily::Sketch => "Esquisse",
        CommandFamily::Editing => "Édition",
        CommandFamily::DrawingTools => "Outils de dessin",
        CommandFamily::Circles => "Cercles",
        CommandFamily::Dimensions => "Cotes",
        CommandFamily::Constraints => "Contraintes",
        CommandFamily::Extrusion => "Extrusion",
        CommandFamily::Window => "Fenêtre",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn no_two_commands_of_the_palette_read_the_same() {
        let mut seen: BTreeMap<&str, Command> = BTreeMap::new();

        for command in Command::ALL {
            assert!(
                !label(command).is_empty(),
                "{command:?} has no name, so its button is a blank",
            );
            if let Some(taken) = seen.insert(label(command), command) {
                panic!(
                    "{taken:?} and {command:?} both read {:?}: the palette offers two \
                     entries a user cannot tell apart",
                    label(command),
                );
            }
        }
    }

    #[test]
    fn resting_on_a_command_says_more_than_its_button_already_shows() {
        for command in Command::ALL {
            assert!(
                !hint(command).is_empty(),
                "{command:?} has nothing to say on hover",
            );
            assert_ne!(
                hint(command),
                label(command),
                "{command:?} repeats its own name on hover instead of helping",
            );
        }
    }

    #[test]
    fn no_two_families_of_the_palette_are_headed_the_same() {
        let mut seen: BTreeMap<&str, CommandFamily> = BTreeMap::new();

        for command in Command::ALL {
            let family = command.family();
            assert!(
                !family_heading(family).is_empty(),
                "{family:?} heads its run of the palette with a blank",
            );
            if let Some(taken) = seen.insert(family_heading(family), family) {
                assert_eq!(
                    taken,
                    family,
                    "{taken:?} and {family:?} both head {:?}: the palette reads as \
                     one family cut in two",
                    family_heading(family),
                );
            }
        }
    }
}
