use crate::game::generated::{QUESTS, QUESTS_COLUMNS};
use crate::game::{GameLanguage, QuestId};
use std::array;
use std::iter::Copied;
use std::slice::Iter;

pub(crate) struct QuestLocale {
    language: GameLanguage,
    translation: [(QuestId, &'static str); QUESTS_COLUMNS],
}

impl QuestLocale {
    #[must_use]
    pub(crate) fn new() -> Self {
        let language = GameLanguage::default();
        #[allow(clippy::cast_possible_truncation)]
        let mut result = Self {
            language,
            translation: array::from_fn(|i| (QuestId(i as u16), "")),
        };
        result.set_language(language);
        result
    }

    #[inline]
    pub(crate) fn all_str(&self) -> Copied<Iter<'_, (QuestId, &'static str)>> {
        self.translation.iter().copied()
    }

    #[inline]
    #[must_use]
    pub(crate) fn language(&self) -> GameLanguage {
        self.language
    }

    pub(crate) fn set_language(&mut self, language: GameLanguage) {
        self.language = language;
        for (q, n) in &mut self.translation {
            *n = QUESTS[language as usize][q.0 as usize];
        }
        self.translation.sort_by_key(|(_, n)| *n);
    }

    #[inline]
    #[must_use]
    pub(crate) fn get(&self, q: QuestId) -> &'static str {
        QUESTS[self.language as usize][q.0 as usize]
    }

    #[must_use]
    pub(crate) fn try_get(&self, name: &str) -> Option<QuestId> {
        self.all_str().find(|(_, n)| *n == name).map(|(q, _)| q)
    }
}
