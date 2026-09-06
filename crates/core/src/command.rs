use serde::{Deserialize, Serialize};

/// Everything the user can ask the application to do.
///
/// One list for both the toolbar and the keyboard: a button and a shortcut are
/// two ways of asking for the same thing, and keeping them apart would mean
/// adding every new action twice and letting them drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Command {
    NewSketch,
    FinishSketch,
    RecenterOnSketch,
    Undo,
    Redo,

    ToolSelect,
    ToolLine,
    ToolRectangle,
    ToolCircle,
    ToolPoint,
    ToolDimension,

    CircleCenter,
    CircleTwoPoints,
    CircleThreePoints,
    CircleTwoTangents,
    CircleThreeTangents,

    DimensionAuto,
    DimensionPointToPoint,
    DimensionLength,
    DimensionAngle,
    DimensionRadius,

    RulePerpendicular,
    RuleParallel,
    RuleEqual,
    RuleCoincident,
    RuleCollinear,
    RuleTangent,
    RuleMidpoint,
    RuleFixed,
    RuleConcentric,

    ExtrusionAdd,
    ExtrusionCut,
    ExtrusionStraight,
    ExtrusionRevolution,
    ExtrusionApply,
    ExtrusionCancel,

    ToggleHistory,
    ToggleToolbarDocked,
    OpenSettings,
    BackToMenu,
}

impl Command {
    /// Every command, in the order the settings screen offers them.
    pub const ALL: [Self; 38] = [
        Self::NewSketch,
        Self::FinishSketch,
        Self::RecenterOnSketch,
        Self::Undo,
        Self::Redo,
        Self::ToolSelect,
        Self::ToolLine,
        Self::ToolRectangle,
        Self::ToolCircle,
        Self::ToolPoint,
        Self::ToolDimension,
        Self::CircleCenter,
        Self::CircleTwoPoints,
        Self::CircleThreePoints,
        Self::CircleTwoTangents,
        Self::CircleThreeTangents,
        Self::DimensionAuto,
        Self::DimensionPointToPoint,
        Self::DimensionLength,
        Self::DimensionAngle,
        Self::DimensionRadius,
        Self::RulePerpendicular,
        Self::RuleParallel,
        Self::RuleEqual,
        Self::RuleCoincident,
        Self::RuleCollinear,
        Self::RuleTangent,
        Self::RuleMidpoint,
        Self::RuleFixed,
        Self::RuleConcentric,
        Self::ExtrusionAdd,
        Self::ExtrusionCut,
        Self::ExtrusionStraight,
        Self::ExtrusionRevolution,
        Self::ExtrusionApply,
        Self::ExtrusionCancel,
        Self::ToggleHistory,
        Self::ToggleToolbarDocked,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::NewSketch => "Nouvelle esquisse",
            Self::FinishSketch => "Terminer",
            Self::RecenterOnSketch => "Recadrer",
            Self::Undo => "Annuler",
            Self::Redo => "Rétablir",
            Self::ToolSelect => "Sélection",
            Self::ToolLine => "Ligne",
            Self::ToolRectangle => "Rectangle",
            Self::ToolCircle => "Cercle",
            Self::ToolPoint => "Point",
            Self::ToolDimension => "Cote",
            Self::CircleCenter => "Centre et diamètre",
            Self::CircleTwoPoints => "Deux points du bord",
            Self::CircleThreePoints => "Deux points puis le centre",
            Self::CircleTwoTangents => "Tangent à deux droites",
            Self::CircleThreeTangents => "Tangent à trois droites",
            Self::DimensionAuto => "Cote intelligente",
            Self::DimensionPointToPoint => "Cote point à point",
            Self::DimensionLength => "Cote de trait",
            Self::DimensionAngle => "Cote d'angle",
            Self::DimensionRadius => "Cote de rayon",
            Self::RulePerpendicular => "Perpendiculaire",
            Self::RuleParallel => "Parallèle",
            Self::RuleEqual => "Égalité",
            Self::RuleCoincident => "Coïncidence",
            Self::RuleCollinear => "Colinéaire",
            Self::RuleTangent => "Tangence",
            Self::RuleMidpoint => "Milieu",
            Self::RuleFixed => "Fixe",
            Self::RuleConcentric => "Concentrique",
            Self::ExtrusionAdd => "Ajout de matière",
            Self::ExtrusionCut => "Enlèvement de matière",
            Self::ExtrusionStraight => "Extrusion droite",
            Self::ExtrusionRevolution => "Révolution",
            Self::ExtrusionApply => "Appliquer",
            Self::ExtrusionCancel => "Annuler l'extrusion",
            Self::ToggleHistory => "Panneau Historique",
            Self::ToggleToolbarDocked => "Ancrer / détacher la barre",
            Self::OpenSettings => "Préférences",
            Self::BackToMenu => "Accueil",
        }
    }

    /// The one line of help shown when the pointer rests on the command.
    pub fn hint(self) -> &'static str {
        match self {
            Self::NewSketch => "Choisir un plan pour commencer un dessin",
            Self::FinishSketch => "Terminer l'esquisse et proposer de l'extruder",
            Self::RecenterOnSketch => "Se replacer face au plan de l'esquisse",
            Self::Undo => "Reculer d'une étape dans l'historique",
            Self::Redo => "Avancer d'une étape dans l'historique",
            Self::ToolSelect => "Cliquer-glisser un point pour le déplacer",
            Self::ToolLine => "Clics successifs, Échap pour terminer la chaîne",
            Self::ToolRectangle => "Deux clics : deux coins opposés",
            Self::ToolCircle => "Deux clics : centre puis rayon",
            Self::ToolPoint => "Un clic pose un point",
            Self::ToolDimension => "Cote intelligente : cliquer ce qu'on veut mesurer",
            Self::CircleCenter => "Le centre, puis un point du bord",
            Self::CircleTwoPoints => "Deux points opposés du bord",
            Self::CircleThreePoints => "Deux points du bord, puis le centre sur leur médiatrice",
            Self::CircleTwoTangents => "Deux droites, puis le centre sur leur bissectrice",
            Self::CircleThreeTangents => "Trois droites : le cercle inscrit entre elles",
            Self::DimensionAuto => "Mesure ce qui est sous le curseur",
            Self::DimensionPointToPoint => "Deux points, reliés ou non",
            Self::DimensionLength => "Un trait, mesuré sur toute sa longueur",
            Self::DimensionAngle => "Deux traits qui se touchent, ou un trait et un axe",
            Self::DimensionRadius => "Un cercle",
            Self::RulePerpendicular => "Deux traits, mis d'équerre",
            Self::RuleParallel => "Deux traits, gardant la même direction",
            Self::RuleEqual => "Deux traits de même longueur, ou deux cercles de même rayon",
            Self::RuleCoincident => "Un point posé sur un trait, ou deux points fondus en un",
            Self::RuleCollinear => "Deux traits couchés sur la même droite",
            Self::RuleTangent => "Un cercle et un trait qui l'effleure",
            Self::RuleMidpoint => "Un point tenu au milieu d'un trait",
            Self::RuleFixed => "Un point qui ne bouge plus de sa place",
            Self::RuleConcentric => "Deux cercles ramenés sur le même centre",
            Self::ExtrusionAdd => "Sélectionner des aires fermées, donner une hauteur",
            Self::ExtrusionCut => "Sélectionner des aires fermées, donner une profondeur",
            Self::ExtrusionStraight => "Pousser la matière perpendiculairement au plan",
            Self::ExtrusionRevolution => "Faire tourner l'aire autour d'un axe du plan",
            Self::ExtrusionApply => "Fabriquer le volume avec les aires choisies",
            Self::ExtrusionCancel => "Abandonner l'extrusion en cours",
            Self::ToggleHistory => "Afficher ou masquer l'arbre des opérations",
            Self::ToggleToolbarDocked => "Détacher la barre d'outils ou l'ancrer",
            Self::OpenSettings => "Régler l'application",
            Self::BackToMenu => "Revenir au menu de démarrage",
        }
    }

    /// Which family the command belongs to, used to group the palette in the
    /// settings screen.
    pub fn family(self) -> &'static str {
        match self {
            Self::NewSketch | Self::FinishSketch | Self::RecenterOnSketch => "Esquisse",
            Self::Undo | Self::Redo => "Édition",
            Self::ToolSelect
            | Self::ToolLine
            | Self::ToolRectangle
            | Self::ToolCircle
            | Self::ToolPoint
            | Self::ToolDimension => "Outils de dessin",
            Self::CircleCenter
            | Self::CircleTwoPoints
            | Self::CircleThreePoints
            | Self::CircleTwoTangents
            | Self::CircleThreeTangents => "Cercles",
            Self::DimensionAuto
            | Self::DimensionPointToPoint
            | Self::DimensionLength
            | Self::DimensionAngle
            | Self::DimensionRadius => "Cotes",
            Self::RulePerpendicular
            | Self::RuleParallel
            | Self::RuleEqual
            | Self::RuleCoincident
            | Self::RuleCollinear
            | Self::RuleTangent
            | Self::RuleMidpoint
            | Self::RuleFixed
            | Self::RuleConcentric => "Contraintes",
            Self::ExtrusionAdd
            | Self::ExtrusionCut
            | Self::ExtrusionStraight
            | Self::ExtrusionRevolution
            | Self::ExtrusionApply
            | Self::ExtrusionCancel => "Extrusion",
            Self::ToggleHistory
            | Self::ToggleToolbarDocked
            | Self::OpenSettings
            | Self::BackToMenu => "Fenêtre",
        }
    }
}
