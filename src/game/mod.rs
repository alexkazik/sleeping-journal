pub(crate) use crate::game::game_language::GameLanguage;
pub(crate) use crate::game::location_id::LocationId;
pub(crate) use crate::game::map::{MAP, MAP_DEFAULT_POSITION};
pub(crate) use crate::game::msg::MsgLanguage;
pub(crate) use crate::game::quest_id::QuestId;
pub(crate) use crate::game::quest_locale::QuestLocale;

mod game_language;
mod generated;
mod location_id;
mod map;
mod msg;
mod quest_id;
mod quest_locale;
