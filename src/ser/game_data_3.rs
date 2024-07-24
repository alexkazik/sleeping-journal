use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState};
use crate::data::quest_location::QuestLocation;
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId};
use crate::global::data::Data;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeQuest3<'a>(
    pub(crate) QuestState,
    pub(crate) BTreeMap<usize, SerdeEncounter3>,
    pub(crate) Cow<'a, str>,
    pub(crate) Vis,
);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeLocation3<'a>(pub(crate) Cow<'a, str>);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeEncounter3(pub(crate) BTreeMap<EncounterType, (Option<usize>, Vis)>);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SerdeGameData3<'a> {
    pub(crate) version_3: (),
    pub(crate) quests: BTreeMap<usize, SerdeQuest3<'a>>,
    pub(crate) locations: BTreeMap<usize, SerdeLocation3<'a>>,
}

impl Data {
    fn save_encounter(encounters: &QuestLocation) -> SerdeEncounter3 {
        SerdeEncounter3(
            encounters
                .iter()
                .map(|(et, quest_location_encounter)| {
                    (
                        *et,
                        (
                            quest_location_encounter.prerequisite.map(QuestId::raw),
                            quest_location_encounter.vis,
                        ),
                    )
                })
                .collect(),
        )
    }

    fn save_quest(quest: &Quest) -> SerdeQuest3 {
        SerdeQuest3(
            quest.state,
            quest
                .encounter
                .iter()
                .filter(|(location_id, _)| **location_id != LocationId::prologue())
                .map(|(location_id, encounters)| {
                    (location_id.raw(), Self::save_encounter(encounters))
                })
                .collect(),
            Cow::Borrowed(&quest.note),
            quest.vis,
        )
    }

    pub(crate) fn save_game_data(&self) -> SerdeGameData3 {
        SerdeGameData3 {
            version_3: (),
            quests: self
                .quest
                .iter()
                .map(|(quest_id, quest)| (quest_id.raw(), Self::save_quest(quest)))
                .collect(),
            locations: self
                .location
                .iter()
                .map(|(l, n)| (l.raw(), SerdeLocation3(Cow::Borrowed(n))))
                .collect(),
        }
    }

    pub(crate) fn load_game_data_3(&mut self, game_data: SerdeGameData3) {
        self.reset();

        for (raw_location_id, data) in game_data.locations {
            if let Some(location_id) = LocationId::from_raw(raw_location_id) {
                self.location.insert(location_id, data.0.to_string().into());
            }
        }

        for (raw_quest_id, data) in game_data.quests {
            if let Some(quest_id) = QuestId::from_raw(raw_quest_id) {
                let quest = self.quest.entry(quest_id).or_default();
                quest.state = data.0;
                for (raw_location_id, e_data) in data.1 {
                    if let Some(location_id) = LocationId::from_raw(raw_location_id) {
                        let encounters = quest.encounter.entry(location_id).or_default();
                        for (encounter_type, (raw_prerequisite, vis)) in e_data.0 {
                            let prerequisite = if encounter_type == EncounterType::Gain {
                                raw_prerequisite.and_then(QuestId::from_raw)
                            } else {
                                None
                            };
                            encounters.insert(encounter_type, prerequisite, vis);
                        }
                    }
                }
                quest.note = data.2.to_string().into();
                quest.vis = data.3;
            }
        }

        self.cleanup();
    }
}
