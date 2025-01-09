use crate::game::GameLanguage;
use crate::game::generated::{LOCATIONS, LOCATIONS_ROWS};
use std::iter::Map;
use std::ops::Range;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct LocationId(u16);

impl LocationId {
    #[inline]
    #[must_use]
    pub(crate) fn raw(self) -> usize {
        self.0.into()
    }

    #[inline]
    #[must_use]
    pub(crate) fn from_raw(raw: usize) -> Option<Self> {
        if raw > 0 && raw < LOCATIONS_ROWS {
            #[allow(clippy::cast_possible_truncation)]
            Some(Self(raw as u16))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn all() -> Map<Range<u16>, fn(u16) -> LocationId> {
        #[allow(clippy::cast_possible_truncation)]
        (0..(LOCATIONS_ROWS as u16)).map(LocationId)
    }

    #[inline]
    #[must_use]
    pub(crate) fn name(self, locale: GameLanguage) -> &'static str {
        LOCATIONS[self.0 as usize][0].unwrap_or_else(|| locale.prologue())
    }

    #[inline]
    #[must_use]
    pub(crate) fn try_from_name(name: &str) -> Option<Self> {
        #[allow(clippy::cast_possible_truncation)]
        LOCATIONS
            .iter()
            .position(|[x, _]| x == &Some(name))
            .map(|x| LocationId(x as u16))
    }

    #[inline]
    #[must_use]
    pub(crate) fn page(self) -> Option<&'static str> {
        LOCATIONS[self.0 as usize][1]
    }

    #[inline]
    #[must_use]
    pub(crate) const fn prologue() -> Self {
        LocationId(0)
    }
}
