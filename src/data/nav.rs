use crate::route::Route;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum Nav {
    Info,
    TodoAndMap,
    Edit,
    Settings,
}

impl From<&Route> for Nav {
    fn from(value: &Route) -> Self {
        match value {
            Route::Info => Nav::Info,
            Route::Todo | Route::TodoAction(_, _) => Nav::TodoAndMap,
            Route::Map | Route::MapLocation(_) | Route::MapAction(_, _) | Route::MapNewQuest(_) => {
                Nav::TodoAndMap
            }
            Route::Edit | Route::EditQuest(_) => Nav::Edit,
            Route::Settings => Nav::Settings,
        }
    }
}
