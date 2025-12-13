use crate::game::data::{NUM_QUESTS, QUEST_KEYWORDS};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct QuestId(pub(super) u16);

impl QuestId {
    #[inline]
    #[must_use]
    pub(crate) fn raw(self) -> usize {
        self.0 as usize
    }

    #[inline]
    #[must_use]
    pub(crate) fn from_raw(raw: usize) -> Option<Self> {
        if raw < NUM_QUESTS {
            #[allow(clippy::cast_possible_truncation)]
            Some(Self(raw as u16))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn is_keyword_raw(self) -> bool {
        QUEST_KEYWORDS[self.0 as usize]
    }

    #[inline]
    #[must_use]
    pub(crate) fn raid() -> Self {
        QuestId(0)
    }

    #[inline]
    #[must_use]
    pub(crate) fn cottage() -> Self {
        QuestId(1)
    }
}
