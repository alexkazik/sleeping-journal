use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState};
use crate::data::vis::Vis;
use crate::game::QuestId;
use crate::global::data::Data;
use std::collections::btree_map::{Iter, IterMut};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default, Clone)]
#[repr(transparent)]
pub(crate) struct QuestLocation(BTreeMap<EncounterType, QuestLocationEncounter>);

impl QuestLocation {
    // shortcuts, which does not require a reference
    #[inline]
    pub(crate) fn contains_key(&self, encounter_type: EncounterType) -> bool {
        self.0.contains_key(&encounter_type)
    }

    #[inline]
    pub(crate) fn remove(&mut self, key: EncounterType) -> Option<QuestLocationEncounter> {
        self.0.remove(&key)
    }

    #[inline]
    pub(crate) fn get_mut(&mut self, key: EncounterType) -> Option<&mut QuestLocationEncounter> {
        self.0.get_mut(&key)
    }

    // shortcut to create a QuestLocationEncounter and insert it
    #[inline]
    pub(crate) fn insert(
        &mut self,
        key: EncounterType,
        prerequisite: Option<QuestId>,
        vis: Vis,
    ) -> Option<QuestLocationEncounter> {
        self.0
            .insert(key, QuestLocationEncounter { prerequisite, vis })
    }

    pub(crate) fn get_active(
        &self,
        quest: &Quest,
        data: &Data,
        ignore_visibility: bool,
    ) -> BTreeMap<EncounterType, bool> {
        let mut result = BTreeMap::new();
        if quest.vis == Vis::Visible || ignore_visibility {
            for (et, qle) in &self.0 {
                if qle.vis == Vis::Visible || ignore_visibility {
                    result.insert(
                        *et,
                        match quest.state {
                            QuestState::NotFound => match et {
                                EncounterType::Unless => true,
                                EncounterType::Gain => {
                                    match qle.prerequisite {
                                        None => true, // always ready to pick up
                                        Some(prerequisite) => data
                                            .quest
                                            .get(&prerequisite)
                                            .map_or(false, |q| q.state == QuestState::Removed),
                                    }
                                }
                                EncounterType::When
                                | EncounterType::Complete
                                | EncounterType::Lose => false,
                            },
                            QuestState::InGame => match et {
                                EncounterType::When
                                | EncounterType::Complete
                                | EncounterType::Lose => true,
                                EncounterType::Unless | EncounterType::Gain => false,
                            },
                            QuestState::Removed => false,
                        },
                    );
                }
            }
        }
        result
    }
}

impl Deref for QuestLocation {
    type Target = BTreeMap<EncounterType, QuestLocationEncounter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QuestLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> IntoIterator for &'a QuestLocation {
    type Item = (&'a EncounterType, &'a QuestLocationEncounter);
    type IntoIter = Iter<'a, EncounterType, QuestLocationEncounter>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut QuestLocation {
    type Item = (&'a EncounterType, &'a mut QuestLocationEncounter);
    type IntoIter = IterMut<'a, EncounterType, QuestLocationEncounter>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct QuestLocationEncounter {
    pub(crate) prerequisite: Option<QuestId>,
    pub(crate) vis: Vis,
}
