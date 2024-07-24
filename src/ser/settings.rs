use crate::data::sub_component::SubComponent;
use crate::game::{GameLanguage, MsgLanguage};
use crate::global::app::App;
use crate::pane::action::PaneAction;
use crate::pane::edit::PaneEdit;
use crate::pane::edit_quest::PaneEditQuest;
use crate::pane::map::PaneMap;
use crate::pane::map_location::PaneMapLocation;
use crate::pane::map_new_quest::PaneMapNewQuest;
use crate::pane::settings::PaneSettings;
use crate::pane::todo::PaneTodo;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct SerdeSettings {
    // global settings
    msg_language: MsgLanguage,
    game_language: GameLanguage,
    quests_is_map: bool,
    // panes
    pane_todo: <PaneTodo as SubComponent>::Ser,
    pane_map: <PaneMap as SubComponent>::Ser,
    #[serde(skip_serializing)]
    pane_map_location: <PaneMapLocation as SubComponent>::Ser,
    #[serde(skip_serializing)]
    pane_map_new_quest: <PaneMapNewQuest as SubComponent>::Ser,
    #[serde(skip_serializing)]
    pane_action: <PaneAction as SubComponent>::Ser,
    #[serde(skip_serializing)]
    pane_edit: <PaneEdit as SubComponent>::Ser,
    #[serde(skip_serializing)]
    pane_edit_quest: <PaneEditQuest as SubComponent>::Ser,
    pane_settings: <PaneSettings as SubComponent>::Ser,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct EmptySer {}

impl App {
    pub(crate) fn save_settings(&self) -> SerdeSettings {
        SerdeSettings {
            // global settings
            msg_language: self.data.msg,
            game_language: self.data.quest_locale.language(),
            quests_is_map: self.quests_is_map,
            // panes
            pane_todo: self.pane_todo.save(),
            pane_map: self.pane_map.save(),
            pane_map_location: self.pane_map_location.save(),
            pane_map_new_quest: self.pane_map_new_quest.save(),
            pane_action: self.pane_action.save(),
            pane_edit: self.pane_edit.save(),
            pane_edit_quest: self.pane_edit_quest.save(),
            pane_settings: self.pane_settings.save(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn load_settings(&mut self, settings: SerdeSettings) {
        // global settings
        self.data.msg = settings.msg_language;
        self.data.quest_locale.set_language(settings.game_language);
        self.quests_is_map = settings.quests_is_map;
        // panes
        self.pane_todo.load(settings.pane_todo);
        self.pane_map.load(settings.pane_map);
        self.pane_map_location.load(settings.pane_map_location);
        self.pane_map_new_quest.load(settings.pane_map_new_quest);
        self.pane_action.load(settings.pane_action);
        self.pane_edit.load(settings.pane_edit);
        self.pane_edit_quest.load(settings.pane_edit_quest);
        self.pane_settings.load(settings.pane_settings);
    }
}
