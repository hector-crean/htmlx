use maud::{html, PreEscaped};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type, Default)]
pub enum IconName {
    #[default]
    AvoidDistressingThoughts,
    Disassociation,
    Hypervigilance,
    PrologedPsychologicalStress,
    DistressingDreams,
    AvoidReminders,
    IrritbaleBehaviour,
    PsychologicalReactions,
    DistressingMemories,
    SleepDisturbance,
    ExageratedStartleResponse,
    Flashbacks,
}

impl IconName {
    fn svg(&self) -> &str {
        use IconName::*;
        match self {
            AvoidDistressingThoughts => include_str!("./icons/AvoidDistressingThoughts.svg"),
            Disassociation => include_str!("./icons/Disassociation.svg"),
            Hypervigilance => include_str!("./icons/Hypervigilance.svg"),
            PrologedPsychologicalStress => include_str!("./icons/PrologedPsychologicalStress.svg"),
            DistressingDreams => include_str!("./icons/DistressingDreams.svg"),
            AvoidReminders => include_str!("./icons/AvoidReminders.svg"),
            IrritbaleBehaviour => include_str!("./icons/IrritbaleBehaviour.svg"),
            PsychologicalReactions => include_str!("./icons/PsychologicalReactions.svg"),
            DistressingMemories => include_str!("./icons/DistressingMemories.svg"),
            SleepDisturbance => include_str!("./icons/SleepDisturbance.svg"),
            ExageratedStartleResponse => include_str!("./icons/ExageratedStartleResponse.svg"),
            Flashbacks => include_str!("./icons/Flashbacks.svg"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IconProps {
    pub name: IconName,
}

impl Default for IconProps {
    fn default() -> Self {
        IconProps {
            name: IconName::default(),
        }
    }
}

impl maud::Render for IconProps {
    fn render(&self) -> maud::Markup {
        html!((PreEscaped(self.name.svg())))
    }
}
