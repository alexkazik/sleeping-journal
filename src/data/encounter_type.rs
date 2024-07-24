use serde_repr::{Deserialize_repr, Serialize_repr};
use yew_bootstrap::icons::BI;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum EncounterType {
    Unless = 0,
    Gain = 1,
    When = 2,
    Complete = 3,
    Lose = 4,
}

impl EncounterType {
    pub(crate) fn raw(self) -> u8 {
        self as u8
    }

    pub(crate) fn icon_active(self) -> BI {
        match self {
            EncounterType::Unless => BI::EXCLAMATION_SQUARE_FILL,
            EncounterType::Gain => BI::PLUS_SQUARE_FILL,
            EncounterType::When => BI::QUESTION_SQUARE_FILL,
            EncounterType::Complete => BI::CHECK_SQUARE_FILL,
            EncounterType::Lose => BI::X_SQUARE_FILL,
        }
    }

    pub(crate) fn icon(self, active: bool) -> BI {
        if active {
            self.icon_active()
        } else {
            match self {
                EncounterType::Unless => BI::EXCLAMATION_SQUARE,
                EncounterType::Gain => BI::PLUS_SQUARE,
                EncounterType::When => BI::QUESTION_SQUARE,
                EncounterType::Complete => BI::CHECK_SQUARE,
                EncounterType::Lose => BI::X_SQUARE_FILL,
            }
        }
    }

    pub(crate) fn to_csv(self) -> &'static str {
        match self {
            EncounterType::Unless => "unless",
            EncounterType::Gain => "gain",
            EncounterType::When => "when",
            EncounterType::Complete => "complete",
            EncounterType::Lose => "lose",
        }
    }
    pub(crate) fn try_from_csv(input: &str) -> Option<Self> {
        match input {
            "unless" => Some(EncounterType::Unless),
            "gain" => Some(EncounterType::Gain),
            "when" => Some(EncounterType::When),
            "complete" => Some(EncounterType::Complete),
            "lose" => Some(EncounterType::Lose),
            _ => None,
        }
    }
}
