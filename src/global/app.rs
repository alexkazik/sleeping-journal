use crate::data::nav::Nav;
use crate::data::sub_component::{SubComponent, UpdateResult};
use crate::game::{MsgLanguage, QuestLocale};
use crate::global::data::Data;
use crate::pane::action::{MsgAction, PaneAction};
use crate::pane::edit::{MsgEdit, PaneEdit};
use crate::pane::edit_quest::{MsgEditQuest, PaneEditQuest};
use crate::pane::info::info_view;
use crate::pane::map::{MsgMap, PaneMap};
use crate::pane::map_location::{MsgMapLocation, PaneMapLocation};
use crate::pane::map_new_quest::{MsgMapNewQuest, PaneMapNewQuest};
use crate::pane::settings::{MsgSettings, PaneSettings};
use crate::pane::todo::{MsgTodo, PaneTodo};
use crate::route::{Route, Router};
use crate::ser::game_data_1::SerdeGameData1;
use crate::ser::game_data_2::SerdeGameData2;
use crate::ser::game_data_3::SerdeGameData3;
use crate::ser::settings::SerdeSettings;
use gloo_storage::errors::StorageError;
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
#[cfg(feature = "debug")]
use gloo_utils::format::JsValueSerdeExt;
use std::collections::{HashMap, VecDeque};
#[cfg(feature = "debug")]
use web_sys::wasm_bindgen::JsValue;
use yew::{Component, Context, Html, classes, html};

#[derive(Clone)]
pub(crate) enum MsgApp {
    MsgTodo(MsgTodo),
    MsgMap(MsgMap),
    MsgLocation(MsgMapLocation),
    MsgNewQuest(MsgMapNewQuest),
    MsgAction(MsgAction),
    MsgSettings(MsgSettings),
    MsgEditList(MsgEdit),
    MsgEditQuest(MsgEditQuest),
    Tick,
    Go(Route),
    Back,
    HistoryChanged,
    ResetToNew,
}

pub(crate) struct App {
    pub(crate) data: Data,
    save_settings: bool,
    router: Router,
    _tick_interval: Interval,
    // panes
    route: Route,
    pub(crate) quests_is_map: bool,
    pub(crate) pane_todo: PaneTodo,
    pub(crate) pane_map: PaneMap,
    pub(crate) pane_map_location: PaneMapLocation,
    pub(crate) pane_map_new_quest: PaneMapNewQuest,
    pub(crate) pane_action: PaneAction,
    pub(crate) pane_edit: PaneEdit,
    pub(crate) pane_edit_quest: PaneEditQuest,
    pub(crate) pane_settings: PaneSettings,
}

impl App {
    const STORAGE_KEY_GAME_DATA: &'static str = "sleeping-gods-journal.game-data";
    const STORAGE_KEY_SETTINGS: &'static str = "sleeping-gods-journal.settings";
}

impl Component for App {
    type Message = MsgApp;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link_cloned = ctx.link().clone();

        let mut result = Self {
            data: Data {
                // game data
                quest: HashMap::new(),
                location: HashMap::new(),
                // global settings
                quest_locale: QuestLocale::new(),
                msg: MsgLanguage::default(),
                // messages
                chain_msg: VecDeque::with_capacity(2),
            },
            save_settings: false,
            router: Router::new(ctx),
            _tick_interval: Interval::new(10_000, move || link_cloned.send_message(MsgApp::Tick)),
            // panes
            route: Route::Info,
            quests_is_map: false,
            pane_todo: PaneTodo::create(ctx),
            pane_map: PaneMap::create(ctx),
            pane_map_location: PaneMapLocation::create(ctx),
            pane_map_new_quest: PaneMapNewQuest::create(ctx),
            pane_action: PaneAction::create(ctx),
            pane_settings: PaneSettings::create(ctx),
            pane_edit: PaneEdit::create(ctx),
            pane_edit_quest: PaneEditQuest::create(ctx),
        };
        result.data.reset(); // is required for all built-in's to work

        if let Ok(settings) = LocalStorage::get::<SerdeSettings>(Self::STORAGE_KEY_SETTINGS) {
            result.load_settings(settings);
        }
        if let Ok(game_data) = LocalStorage::get::<SerdeGameData3>(Self::STORAGE_KEY_GAME_DATA) {
            result.data.load_game_data_3(game_data);
        } else if let Ok(game_data) =
            LocalStorage::get::<SerdeGameData2>(Self::STORAGE_KEY_GAME_DATA)
        {
            result.data.load_game_data_2(game_data);
        } else if let Ok(game_data) =
            LocalStorage::get::<SerdeGameData1>(Self::STORAGE_KEY_GAME_DATA)
        {
            result.data.load_game_data_1(game_data);
        }

        #[cfg(feature = "debug")]
        web_sys::console::log_1(&JsValue::from_serde(&result.save_settings).unwrap());
        #[cfg(feature = "debug")]
        web_sys::console::log_1(&JsValue::from_serde(&result.data.save_game_data()).unwrap());

        result
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut r = UpdateResult::empty();
        self.data.chain_msg.push_back(msg);
        while let Some(msg) = self.data.chain_msg.pop_front() {
            r |= match msg {
                MsgApp::MsgTodo(msg) => self.pane_todo.update(&mut self.data, ctx, msg),
                MsgApp::MsgMap(msg) => self.pane_map.update(&mut self.data, ctx, msg),
                MsgApp::MsgLocation(msg) => self.pane_map_location.update(&mut self.data, ctx, msg),
                MsgApp::MsgNewQuest(msg) => {
                    self.pane_map_new_quest.update(&mut self.data, ctx, msg)
                }
                MsgApp::MsgAction(msg) => self.pane_action.update(&mut self.data, ctx, msg),
                MsgApp::MsgSettings(msg) => self.pane_settings.update(&mut self.data, ctx, msg),
                MsgApp::MsgEditList(msg) => self.pane_edit.update(&mut self.data, ctx, msg),
                MsgApp::MsgEditQuest(msg) => self.pane_edit_quest.update(&mut self.data, ctx, msg),
                MsgApp::Tick => {
                    if self.save_settings {
                        let _: Result<(), StorageError> =
                            LocalStorage::set(Self::STORAGE_KEY_SETTINGS, self.save_settings());
                        #[cfg(feature = "debug")]
                        web_sys::console::log_1(
                            &JsValue::from_serde(&self.save_settings()).unwrap(),
                        );
                        self.save_settings = false;
                    }
                    UpdateResult::empty()
                }
                MsgApp::Go(route) => {
                    self.router.go(&self.data, route);
                    UpdateResult::empty()
                }
                MsgApp::Back => {
                    self.router.back();
                    UpdateResult::empty()
                }
                MsgApp::HistoryChanged => {
                    match self.router.current_route(&self.data) {
                        Some(route) => {
                            self.route = route.clone();
                            match route {
                                Route::Todo | Route::TodoAction(_, _) => self.quests_is_map = false,
                                Route::Map
                                | Route::MapLocation(_)
                                | Route::MapAction(_, _)
                                | Route::MapNewQuest(_) => self.quests_is_map = true,
                                Route::Info
                                | Route::Edit
                                | Route::EditQuest(_)
                                | Route::Settings => (),
                            }
                            match route {
                                Route::Info
                                | Route::Todo
                                | Route::Map
                                | Route::Edit
                                | Route::Settings => UpdateResult::Render.into(),
                                Route::TodoAction(q, l) => {
                                    if self.pane_action.go(&self.data, q, l, false) {
                                        UpdateResult::Render.into()
                                    } else {
                                        // not valid -> "redirect" to start page
                                        self.router.go_replace(&self.data, Route::Todo);
                                        UpdateResult::empty()
                                    }
                                }
                                Route::MapLocation(l) => {
                                    self.pane_map_location.go(l);
                                    UpdateResult::Render.into()
                                }
                                Route::MapAction(l, q) => {
                                    if self.pane_action.go(&self.data, q, l, true) {
                                        UpdateResult::Render.into()
                                    } else {
                                        // not valid -> "redirect" to start page
                                        self.router.go_replace(&self.data, Route::MapLocation(l));
                                        UpdateResult::empty()
                                    }
                                }
                                Route::MapNewQuest(l) => {
                                    self.pane_map_new_quest.go(l);
                                    UpdateResult::Render.into()
                                }
                                Route::EditQuest(q) => {
                                    if self.pane_edit_quest.go(&self.data, q) {
                                        UpdateResult::Render.into()
                                    } else {
                                        // not valid -> "redirect" to start page
                                        self.router.go_replace(&self.data, Route::Edit);
                                        UpdateResult::empty()
                                    }
                                }
                            }
                        }
                        None => UpdateResult::empty(),
                    }
                }
                MsgApp::ResetToNew => {
                    self.pane_todo.reset_to_new();
                    self.pane_map.reset_to_new();
                    self.pane_map_location.reset_to_new();
                    self.pane_map_new_quest.reset_to_new();
                    self.pane_action.reset_to_new();
                    self.pane_edit.reset_to_new();
                    self.pane_edit_quest.reset_to_new();
                    self.pane_settings.reset_to_new();

                    self.route = Route::Todo;
                    UpdateResult::Render.into()
                }
            };
        }
        if r.contains(UpdateResult::SaveGameData) {
            self.data.cleanup();
            let _: Result<(), StorageError> =
                LocalStorage::set(Self::STORAGE_KEY_GAME_DATA, self.data.save_game_data());
            #[cfg(feature = "debug")]
            web_sys::console::log_1(&JsValue::from_serde(&self.data.save_game_data()).unwrap());
        }
        if r.contains(UpdateResult::SaveSettings) {
            #[cfg(feature = "debug")]
            web_sys::console::log_1(&"Settings should be saved".into());
            self.save_settings = true;
        }
        r.contains(UpdateResult::Render)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inner = match self.route {
            Route::Info => info_view(&self.data),
            Route::Todo => self.pane_todo.view(&self.data, ctx),
            Route::Map => self.pane_map.view(&self.data, ctx),
            Route::MapLocation(_) => self.pane_map_location.view(&self.data, ctx),
            Route::MapNewQuest(_) => self.pane_map_new_quest.view(&self.data, ctx),
            Route::TodoAction(_, _) | Route::MapAction(_, _) => {
                self.pane_action.view(&self.data, ctx)
            }
            Route::Settings => self.pane_settings.view(&self.data, ctx),
            Route::Edit => self.pane_edit.view(&self.data, ctx),
            Route::EditQuest(_) => self.pane_edit_quest.view(&self.data, ctx),
        };
        let nav_bar = [
            (Route::Info, self.data.msg.nav_info()),
            (if self.quests_is_map { Route::Map } else { Route::Todo }, self.data.msg.nav_quests()),
            (Route::Edit, self.data.msg.nav_edit()),
            (Route::Settings, self.data.msg.nav_settings()),
        ]
            .into_iter()
            .map(|(route, name)| {
                html! {
                      <li class="nav-item ms-2">
                        <a
                            class={classes!("nav-link", if Nav::from(&route) == Nav::from(&self.route) {"active"}else{""})}
                            onclick={ ctx.link().callback(move |_| MsgApp::Go(route.clone())) }
                        >{name}</a>
                      </li>
            }
            });
        let nav_bar2 = if Nav::from(&self.route) == Nav::TodoAndMap {
            if self.quests_is_map {
                Some(html! {
                    <div class="navbar-nav">
                        <ul class="nav nav-pills">
                            <li class="nav-item ms-2">
                                <a class="nav-link" onclick={ ctx.link().callback(move |_| MsgApp::Go(Route::Todo)) }>
                                    {self.data.msg.nav_todo()}
                                </a>
                            </li>
                            <li class="nav-item ms-2">
                                <a class="nav-link active" onclick={ ctx.link().callback(move |_| MsgApp::Go(Route::Map)) }>
                                    {self.data.msg.nav_map()}
                                </a>
                            </li>
                        </ul>
                    </div>
                })
            } else {
                Some(html! {
                    <div class="navbar-nav">
                        <ul class="nav nav-pills">
                            <li class="nav-item ms-2">
                                <a class="nav-link active" onclick={ ctx.link().callback(move |_| MsgApp::Go(Route::Todo)) }>
                                    {self.data.msg.nav_todo()}
                                </a>
                            </li>
                            <li class="nav-item ms-2">
                                <a class="nav-link" onclick={ ctx.link().callback(move |_| MsgApp::Go(Route::Map)) }>
                                    {self.data.msg.nav_map()}
                                </a>
                            </li>
                        </ul>
                    </div>
                })
            }
        } else {
            None
        };
        html! {
          <div class="app-wrap">
            <nav class="navbar sticky-top bg-body-tertiary">
              <div class="container-fluid">
                <a class="navbar-brand" href="#">
                  {"Unofficial Sleeping Gods Journal"}
                </a>

                {nav_bar2}

                <div class="navbar-nav">
                  <ul class="nav nav-pills">
                    { for nav_bar }
                  </ul>
                </div>
              </div>
            </nav>

            <main class="py-4">
              <div class="container">
                { inner }
              </div>
            </main>

            <nav class="navbar sticky-bottom bg-body-tertiary">
              <div class="container-fluid">
                <h5 class="mb-0">
                  {"Written by Alex."}
                </h5>

                <div class="ms-auto">
                  {"Version: "}{env!("CARGO_PKG_VERSION")}
                  <a
                    href="https://github.com/alexkazik/sleeping-journal"
                    target="_blank"
                    class="btn btn-dark btn-sm ms-4"
                  >
                    <i class="bi bi-github"></i>
                    {"Source"}
                  </a>
                </div>
              </div>
            </nav>
          </div>
        }
    }
}
