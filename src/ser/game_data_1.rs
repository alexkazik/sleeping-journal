use crate::data::encounter_type::EncounterType;
use crate::data::quest::QuestState;
use crate::data::vis::Vis;
use crate::global::data::Data;
use crate::ser::game_data_3::{SerdeEncounter3, SerdeGameData3, SerdeLocation3, SerdeQuest3};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
struct SerdeQuest<'a>(QuestState, BTreeMap<usize, SerdeEncounter>, Cow<'a, str>);

#[derive(Serialize, Deserialize)]
struct SerdeLocation<'a>(Cow<'a, str>);

#[derive(Serialize, Deserialize)]
struct SerdeEncounter(BTreeMap<EncounterType, (Option<usize>, Vis)>);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SerdeGameData1<'a> {
    version_1: (),
    quests: BTreeMap<usize, SerdeQuest<'a>>,
    locations: BTreeMap<usize, SerdeLocation<'a>>,
    game_language: String,
}

impl Data {
    pub(crate) fn load_game_data_1(&mut self, game_data: SerdeGameData1) {
        let SerdeGameData1 {
            quests, locations, ..
        } = game_data;

        let game_data = SerdeGameData3 {
            version_3: (),
            quests: quests
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SerdeQuest3(
                            v.0,
                            v.1.into_iter()
                                .map(|(k2, v2)| (k2, SerdeEncounter3(v2.0)))
                                .collect(),
                            v.2,
                            Vis::Visible,
                        ),
                    )
                })
                .collect(),
            locations: locations
                .into_iter()
                .map(|(a, b)| (a, SerdeLocation3(b.0)))
                .collect(),
        };

        self.load_game_data_3(game_data);
    }
}
