use std::fmt::{Display, Formatter};
use std::ops::Deref;
use yew::html::IntoPropValue;
use yew::AttrValue;
use yew_bootstrap::icons::BI;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct Note(String);

impl Note {
    #[inline]
    pub(crate) const fn icon() -> BI {
        BI::JOURNAL_TEXT
    }
}

impl Deref for Note {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Note {
    #[inline]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl IntoPropValue<Option<AttrValue>> for Note {
    #[inline]
    fn into_prop_value(self) -> Option<AttrValue> {
        self.0.into_prop_value()
    }
}

impl Display for Note {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
