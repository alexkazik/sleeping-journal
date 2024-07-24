use crate::data::note::Note;
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::data::vis::Vis;
use crate::game::{LocationId, MAP, MAP_DEFAULT_POSITION};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::{text, Modal};
use crate::route::Route;
use serde::{Deserialize, Serialize};
use yew::{html, Context, Html};
use yew_bootstrap::component::{Button, ButtonSize};
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgMap {
    Page(usize),
    ShowNote(LocationId),
}

impl From<MsgMap> for MsgApp {
    #[inline]
    fn from(msg: MsgMap) -> Self {
        MsgApp::MsgMap(msg)
    }
}

pub(crate) struct PaneMap {
    position: usize,
    modal: Modal,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PaneMapSer {
    position: usize,
}

impl SubComponent for PaneMap {
    type Message = MsgMap;
    type Ser = PaneMapSer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            position: MAP_DEFAULT_POSITION,
            modal: Modal::default(),
        }
    }

    fn reset_to_new(&mut self) {
        self.position = MAP_DEFAULT_POSITION;
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgMap::Page(pos) => {
                if pos < MAP.len() && MAP[pos].is_some() {
                    self.position = pos;
                    UpdateResult::Render | UpdateResult::SaveSettings
                } else {
                    UpdateResult::empty()
                }
            }
            MsgMap::ShowNote(location_id) => {
                if let Some(note) = data.location.get(&location_id) {
                    self.modal.open(
                        &data
                            .msg
                            .str_note_modal_head(location_id.name(data.quest_locale.language())),
                        note,
                    );
                }
                UpdateResult::empty()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let map = MAP.iter().enumerate().map(|(pos, pl_pr)| {
            match pl_pr {
                None => html! {<td/>},
                Some((pl, pr)) => {
                    let page = if pr.is_empty() {
                        data.msg.page_short(pl)
                    } else { data.msg.page_short_lr(pl, pr) };
                    let style = if self.position == pos { Color::Success } else { Color::Warning };
                    html! {<td width="33%" align="center"><Button size={ButtonSize::Small} style={style} onclick={ctx.link().callback(move |_| MsgMap::Page(pos))}>{page}</Button></td>}
                }
            }
        }).collect::<Vec<_>>();
        let map = map.chunks(3).map(|tr| {
            // let tr = VNode::VList(VList::with_children(tr,None));
            let tr = tr.to_vec();
            html! {<tr>{for tr}</tr>}
        });

        let left = Self::render_list(data, ctx, MAP[self.position].unwrap().0);
        let right = Self::render_list(data, ctx, MAP[self.position].unwrap().1);

        html! {
            <>
            <table style="background-color: #FAE4BC">
                <tbody>
                    {for map}
                </tbody>
            </table>
            <div class="row">
              <div class="col-lg-6">
                {left}
              </div>
              <div class="col-lg-6">
                {right}
              </div>
            </div>

            {self.modal.html(data)}
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        PaneMapSer {
            position: self.position,
        }
    }

    fn load(&mut self, stored: Self::Ser) {
        if stored.position < MAP.len() && MAP[stored.position].is_some() {
            self.position = stored.position;
        } else {
            self.position = MAP_DEFAULT_POSITION;
        }
    }
}

impl PaneMap {
    fn render_list(data: &Data, ctx: &Context<App>, page: &str) -> Html {
        let locations = LocationId::all().filter(|l| l.page() == Some(page))
            .map(|l| {
                let location_name = text(l.name(data.quest_locale.language()));
                let quests = itertools::Itertools::intersperse(data.quest_iter().filter_map(|(_, quest, name)| {
                    if let Some(ql) = quest.encounter.get(&l) {
                        let vis = ql.iter().map(|(_, qle)| qle.vis).min().unwrap_or(Vis::Visible);
                        let vis = vis.max(quest.vis);
                        let icons = ql.iter().map(|(et, _)| {
                            html! { <span class="ms-1">{ et.icon(false) }</span> }
                        });

                        Some(html! {<>
                          <span class={vis.class_text()}>
                            {text(name)}
                            <small>{for icons}</small>
                          </span>
                        </>})
                    } else {
                        None
                    }
                })
                                                               , text(", ")).collect::<Vec<_>>();

                html! {
                    <tr>
                        <td>
                            <Button
                              class="btn-sm d-flex align-items-center"
                              onclick={ctx.link().callback(move |_|MsgApp::Go(Route::MapLocation(l)))}
                              children={location_name}
                            />
                        </td>
                        <td>
                            {for quests}
                        </td>
                        <td>
                        if let Some(note) = data.location.get(&l) {
                            {" "}
                            <Button
                              class="btn-sm d-flex align-items-center"
                              style={Color::Info}
                              onclick={ctx.link().callback(move |_|MsgMap::ShowNote(l))}
                            >
                                {data.msg.note_with_icon(Note::icon(), &note_head(note))}
                            </Button>
                        }
                        </td>
                    </tr>
                }
            });
        html! {
            <table class="map-table table table-hover align-middle">
                <thead>
                    <tr>
                        <td colspan="3">
                          <div class="h5">
                            {data.msg.page(page)}
                          </div>
                        </td>
                    </tr>
                </thead>
                <tbody>
                    {for locations}
                    if page == "*" {
                        <tr>
                            <td colspan="3">
                                {data.msg.page_star_help()}
                            </td>
                        </tr>
                    }                </tbody>
            </table>
        }
    }
}

pub(crate) fn note_head(s: &Note) -> String {
    const MAX: usize = 50;
    let s = s.replace('\n', "⏎ ");
    let mut cs: Vec<_> = s.chars().collect();
    if cs.len() <= MAX {
        s
    } else {
        cs.truncate(MAX - 2);
        while cs.last().map_or(false, |c| !c.is_whitespace()) {
            cs.pop();
        }
        cs.push('…');
        cs.into_iter().collect::<String>()
    }
}
