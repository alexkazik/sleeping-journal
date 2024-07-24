use crate::global::data::Data;
use serde_repr::{Deserialize_repr, Serialize_repr};
use yew::{classes, Classes};
use yew_bootstrap::util::Color;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(
    Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub(crate) enum Vis {
    #[default]
    Visible = 0,
    HiddenThisCampaign = 1,
    HiddenForever = 2,
}

impl Vis {
    pub(crate) fn to_csv(self) -> &'static str {
        match self {
            Vis::Visible => "",
            Vis::HiddenThisCampaign => "hidden-this-campaign",
            Vis::HiddenForever => "hidden-forever",
        }
    }

    pub(crate) fn try_from_csv(input: &str) -> Option<Self> {
        match input {
            "" => Some(Vis::Visible),
            "hidden-this-campaign" => Some(Vis::HiddenThisCampaign),
            "hidden-forever" => Some(Vis::HiddenForever),
            _ => None,
        }
    }

    pub(crate) const fn to_style(self) -> Color {
        match self {
            Vis::Visible => Color::Success,
            Vis::HiddenThisCampaign => Color::Warning,
            Vis::HiddenForever => Color::Danger,
        }
    }

    pub(crate) const fn to_style_primary(self) -> Color {
        match self {
            Vis::Visible => Color::Primary,
            Vis::HiddenThisCampaign => Color::Warning,
            Vis::HiddenForever => Color::Danger,
        }
    }

    pub(crate) fn class_text(self) -> Classes {
        match self {
            Vis::Visible => classes!(),
            Vis::HiddenThisCampaign => classes!("text-warning"),
            Vis::HiddenForever => classes!("text-danger"),
        }
    }

    pub(crate) fn class_btn_outline(self) -> Classes {
        match self {
            Vis::Visible => classes!("btn", "btn-outline-success"),
            Vis::HiddenThisCampaign => classes!("btn", "btn-outline-warning"),
            Vis::HiddenForever => classes!("btn", "btn-outline-danger"),
        }
    }

    pub(crate) fn text(self, data: &Data) -> &'static str {
        match self {
            Vis::Visible => data.msg.str_vis_visible(),
            Vis::HiddenThisCampaign => data.msg.str_vis_hidden_this_campaign(),
            Vis::HiddenForever => data.msg.str_vis_hidden_forever(),
        }
    }
}
