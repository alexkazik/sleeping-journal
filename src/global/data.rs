use crate::data::encounter_type::EncounterType;
use crate::data::note::Note;
use crate::data::quest::{Quest, QuestState};
use crate::data::vis::Vis;
use crate::game::{LocationId, MsgLanguage, QuestId, QuestLocale};
use crate::global::app::MsgApp;
use crate::html::text;
use std::collections::{HashMap, VecDeque};
use std::iter::{Copied, FilterMap, Map};
use std::slice::Iter;
use yew::Html;

pub(crate) struct Data {
    // game data
    pub(crate) quest: HashMap<QuestId, Quest>,
    pub(crate) location: HashMap<LocationId, Note>,
    // global settings
    pub(crate) quest_locale: QuestLocale,
    pub(crate) msg: MsgLanguage,
    // messages
    pub(crate) chain_msg: VecDeque<MsgApp>,
}

type QuestAllIter<'a> =
    Map<Copied<Iter<'a, (QuestId, &'static str)>>, fn((QuestId, &'static str)) -> (QuestId, Html)>;

impl Data {
    pub(crate) fn reset(&mut self) {
        self.quest.clear();
        self.location.clear();
        self.built_in();
    }

    pub(crate) fn cleanup(&mut self) {
        self.quest.retain(|_, quest| {
            quest
                .encounter
                .retain(|_, quest_location| !quest_location.is_empty());
            !quest.encounter.is_empty() || !quest.note.is_empty()
        });
    }

    fn built_in(&mut self) {
        for quest_id in [QuestId::raid(), QuestId::cottage()] {
            let quest = self.quest.entry(quest_id).or_default();
            if quest.state != QuestState::Removed {
                quest.state = QuestState::InGame;
            }
            let loc_pro = quest.encounter.entry(LocationId::prologue()).or_default();
            loc_pro.clear();
            loc_pro.insert(EncounterType::Gain, None, Vis::Visible);
        }
    }

    pub(crate) fn quest_all_iter(&self) -> QuestAllIter<'_> {
        self.quest_locale.all_str().map(|(l, n)| (l, text(n)))
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn quest_iter<'a>(
        &'a self,
    ) -> FilterMap<
        Copied<Iter<'a, (QuestId, &'static str)>>,
        impl FnMut((QuestId, &'static str)) -> Option<(QuestId, &'a Quest, &'static str)>,
    > {
        self.quest_locale
            .all_str()
            .filter_map(move |(quest_id, quest_name)| {
                self.quest
                    .get(&quest_id)
                    .map(move |quest| (quest_id, quest, quest_name))
            })
    }
}
