use crate::game::{LocationId, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use gloo_history::{BrowserHistory, History, HistoryListener};
use std::borrow::Cow;
use urlencoding::{decode, encode};
use yew::Context;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub(crate) enum Route {
    Info,
    Todo,
    TodoAction(QuestId, LocationId),
    Map,
    MapLocation(LocationId),
    MapAction(LocationId, QuestId),
    MapNewQuest(LocationId),
    Edit,
    EditQuest(QuestId),
    Settings,
}

impl Route {
    fn path(&self, data: &Data, base: &str) -> String {
        match self {
            Route::Info => format!("{base}/#info"),
            Route::Todo => format!("{base}/#todo"),
            Route::TodoAction(q, l) => format!(
                "{base}/#todo/{}/{}",
                encode(data.quest_locale.get(*q)),
                encode(l.name(data.quest_locale.language())),
            ),
            Route::Map => format!("{base}/#map"),
            Route::MapLocation(l) => format!(
                "{base}/#map/{}",
                encode(l.name(data.quest_locale.language()))
            ),
            Route::MapAction(l, q) => format!(
                "{base}/#map/{}/{}",
                encode(l.name(data.quest_locale.language())),
                encode(data.quest_locale.get(*q)),
            ),
            Route::MapNewQuest(l) => format!(
                "{base}/#map/{}/new",
                encode(l.name(data.quest_locale.language()))
            ),
            Route::Edit => format!("{base}/#edit"),
            Route::EditQuest(q) => format!("{base}/#edit/{}", encode(data.quest_locale.get(*q)),),
            Route::Settings => format!("{base}/#settings"),
        }
    }

    fn parse(data: &Data, hash: &str) -> Self {
        let path = hash.strip_prefix('#').unwrap_or(hash);
        let path = path
            .split('/')
            .map(decode)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
        let mut path = path.iter().map(Cow::as_ref);
        match path.next() {
            Some("todo") => {
                let q = path.next().and_then(|q| data.quest_locale.try_get(q));
                let l = path.next().and_then(LocationId::try_from_name);
                if let Some((q, l)) = q.zip(l) {
                    Route::TodoAction(q, l)
                } else {
                    Route::Todo
                }
            }
            Some("map") => {
                let l = path.next().and_then(LocationId::try_from_name);
                if let Some(l) = l {
                    let snd = path.next();
                    let q = snd.and_then(|q| data.quest_locale.try_get(q));
                    if let Some(q) = q {
                        Route::MapAction(l, q)
                    } else if snd == Some("new") {
                        Route::MapNewQuest(l)
                    } else {
                        Route::MapLocation(l)
                    }
                } else {
                    Route::Map
                }
            }
            Some("edit") => {
                let q = path.next().and_then(|q| data.quest_locale.try_get(q));
                if let Some(q) = q {
                    Route::EditQuest(q)
                } else {
                    Route::Edit
                }
            }
            Some("settings") => Route::Settings,
            _ => Route::Info,
        }
    }
}

pub(crate) struct Router {
    browser_history: BrowserHistory,
    _history_listener: HistoryListener,
    base: String,
    redirect_counter: usize,
}

impl Router {
    pub(crate) fn new(ctx: &Context<App>) -> Self {
        let browser_history = BrowserHistory::new();
        ctx.link().send_message(MsgApp::HistoryChanged);
        let link_cloned = ctx.link().clone();
        let history_listener =
            browser_history.listen(move || link_cloned.send_message(MsgApp::HistoryChanged));
        Self {
            browser_history,
            _history_listener: history_listener,
            base: yew_router::utils::fetch_base_url().unwrap_or_default(),
            redirect_counter: 0,
        }
    }

    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn go(&mut self, data: &Data, route: Route) {
        let path = route.path(data, &self.base);
        #[cfg(feature = "debug")]
        web_sys::console::log_2(&"goto".into(), &path.as_str().into());
        self.browser_history.push(path);
        self.redirect_counter = 0;
    }

    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn go_replace(&mut self, data: &Data, route: Route) {
        let path = route.path(data, &self.base);
        #[cfg(feature = "debug")]
        web_sys::console::log_2(&"goto, replace".into(), &path.as_str().into());
        self.browser_history.replace(path);
        self.redirect_counter = 0;
    }

    pub(crate) fn back(&mut self) {
        self.browser_history.back();
        self.redirect_counter = 0;
    }

    pub(crate) fn current_route(&mut self, data: &Data) -> Option<Route> {
        let loc = self.browser_history.location();
        let route = Route::parse(data, loc.hash());

        let new_path = route.path(data, &self.base);
        #[cfg(feature = "debug")]
        web_sys::console::log_1(
            &format!(
                "path \"{}{}\" -> {route:?} (\"{new_path}\")  [c:{}]",
                loc.path(),
                loc.hash(),
                self.redirect_counter,
            )
            .into(),
        );

        if format!("{}{}", loc.path(), loc.hash()) == new_path || self.redirect_counter > 3 {
            Some(route)
        } else {
            self.browser_history.replace(&new_path);
            self.redirect_counter += 1;
            None
        }
    }
}
