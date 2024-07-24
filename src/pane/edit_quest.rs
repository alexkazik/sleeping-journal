use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState};
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::route::Route;
use crate::ser::settings::EmptySer;
use std::collections::BTreeSet;
use std::mem;
use web_sys::{Event, HtmlInputElement, MouseEvent};
use yew::{html, AttrValue, Callback, Context, Html, NodeRef};
use yew_bootstrap::component::{Alert, Button};
use yew_bootstrap::icons::BI;
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgEditQuest {
    State(QuestState),
    Vis(Vis),
    EncounterVis(LocationId, EncounterType, Vis),
    KillEncounter(LocationId, EncounterType),
    Save,
}

impl From<MsgEditQuest> for MsgApp {
    #[inline]
    fn from(msg: MsgEditQuest) -> Self {
        MsgApp::MsgEditQuest(msg)
    }
}

pub(crate) struct PaneEditQuest {
    quest_id: QuestId,
    quest: Quest,
    ref_note: NodeRef,
    encounters_to_remove: BTreeSet<(LocationId, EncounterType)>,
    // callbacks
    cb_state_not_found: Callback<Event>,
    cb_state_in_game: Callback<Event>,
    cb_state_removed: Callback<Event>,
    cb_save: Callback<MouseEvent>,
}

impl SubComponent for PaneEditQuest {
    type Message = MsgEditQuest;
    type Ser = EmptySer;

    fn create(ctx: &Context<App>) -> Self {
        Self {
            quest_id: QuestId::cottage(), // anything will do
            quest: Quest::default(),
            ref_note: NodeRef::default(),
            encounters_to_remove: BTreeSet::default(),
            cb_state_not_found: ctx
                .link()
                .callback(|_| MsgEditQuest::State(QuestState::NotFound)),
            cb_state_in_game: ctx
                .link()
                .callback(|_| MsgEditQuest::State(QuestState::InGame)),
            cb_state_removed: ctx
                .link()
                .callback(|_| MsgEditQuest::State(QuestState::Removed)),
            cb_save: ctx.link().callback(|_| MsgEditQuest::Save),
        }
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgEditQuest::State(state) => {
                self.quest.state = state;
                UpdateResult::empty()
            }
            MsgEditQuest::Vis(vis) => {
                self.quest.vis = vis;
                UpdateResult::empty()
            }
            MsgEditQuest::Save => {
                // update "our" quest
                self.quest.note = self
                    .ref_note
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .into();

                // remove some encounters
                for (location_id, encounter_type) in &self.encounters_to_remove {
                    if let Some(ql) = self.quest.encounter.get_mut(location_id) {
                        ql.remove(*encounter_type);
                    }
                }

                // save quest into data
                mem::swap(
                    data.quest.entry(self.quest_id).or_default(),
                    &mut self.quest,
                );

                // go back to list + save game data
                data.chain_msg.push_back(MsgApp::Go(Route::Edit));
                UpdateResult::SaveGameData.into()
            }
            MsgEditQuest::EncounterVis(location_id, encounter_type, vis) => {
                for (l, ql) in &mut self.quest.encounter {
                    if *l == location_id {
                        for (et, qle) in ql {
                            if *et == encounter_type {
                                qle.vis = vis;
                            }
                        }
                    }
                }
                self.encounters_to_remove
                    .remove(&(location_id, encounter_type));
                UpdateResult::empty()
            }
            MsgEditQuest::KillEncounter(location_id, encounter_type) => {
                self.encounters_to_remove
                    .insert((location_id, encounter_type));
                UpdateResult::empty()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let events = self
            .quest
            .encounter
            .iter()
            .flat_map(|(location_id, quest_location)| {
                let location_id = *location_id;
                quest_location.iter().map(move |(encounter_type, encounter)| {
                    let encounter_type = *encounter_type;
                    html! {
                        <li class="list-group-item">
                            {data.msg.location_with_icon( location_id.name(data.quest_locale.language()), encounter_type.icon_active())}<br/>
                            {PaneEditQuest::select_hidden(
                                data,
                                &format!("encounter-{}-{}",location_id.raw(), encounter_type.raw()),
                                encounter.vis,
                                ctx.link().callback(move |_|MsgEditQuest::EncounterVis(location_id, encounter_type, Vis::Visible)),
                                ctx.link().callback(move |_|MsgEditQuest::EncounterVis(location_id, encounter_type,Vis::HiddenThisCampaign)),
                                ctx.link().callback(move |_|MsgEditQuest::EncounterVis(location_id, encounter_type,Vis::HiddenForever)),
                                Some(ctx.link().callback(move |_|MsgEditQuest::KillEncounter(location_id, encounter_type))),
                            )}
                            <br/>
                            if let Some(pre_quest_id) = encounter.prerequisite {
                              {data.msg.ed_qu_prerequisite(data.quest_locale.get( pre_quest_id))}<br/>
                            }
                        </li>
                    }
                })
            });
        html! {
            <>
            <Alert style={Color::Info}>
                {data.msg.ed_qu_hint()}
            </Alert>
            <ul class="list-group">
                <li class="list-group-item">
                    {data.msg.quest_header(data.quest_locale.get( self.quest_id), self.quest.icon(self.quest_id))}
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)}>{data.msg.back()}</Button>
                </li>
                <li class="list-group-item">
                    <div class="btn-group" role="group">
                        <input
                            type="radio"
                            class="btn-check"
                            name="state"
                            id="state0"
                            autocomplete="off"
                            checked={self.quest.state == QuestState::NotFound}
                            onchange={&self.cb_state_not_found}
                        />
                        <label class="btn btn-outline-primary" for="state0">{QuestState::NotFound.text(data)}</label>

                        <input
                            type="radio"
                            class="btn-check"
                            name="state"
                            id="state1"
                            autocomplete="off"
                            checked={self.quest.state == QuestState::InGame}
                            onchange={&self.cb_state_in_game}
                        />
                        <label class="btn btn-outline-primary" for="state1">{QuestState::InGame.text(data)}</label>

                        <input
                            type="radio"
                            class="btn-check"
                            name="state"
                            id="state2"
                            autocomplete="off"
                            checked={self.quest.state == QuestState::Removed}
                            onchange={&self.cb_state_removed}
                        />
                        <label class="btn btn-outline-primary" for="state2">{QuestState::Removed.text(data)}</label>
                    </div>
                </li>
                <li class="list-group-item">
                    <textarea
                        class="form-control"
                        placeholder={data.msg.str_note_placeholder()}
                        id="floatingTextarea"
                        value={self.quest.note.clone()}
                        style="height: 150px"
                        ref={&self.ref_note}
                    />
                </li>
                <li class="list-group-item">
                    {data.msg.ed_qu_quest_vis_header()}<br/>
                    {PaneEditQuest::select_hidden(
                        data,
                        "quest",
                        self.quest.vis,
                        ctx.link().callback(|_|MsgEditQuest::Vis(Vis::Visible)),
                        ctx.link().callback(|_|MsgEditQuest::Vis(Vis::HiddenThisCampaign)),
                        ctx.link().callback(|_|MsgEditQuest::Vis(Vis::HiddenForever)),
                        None,
                    )}
                </li>
                {for events}
                <li class="list-group-item">
                    <Button
                        style={Color::Success}
                        text={data.msg.str_ed_qu_save()}
                        onclick={&self.cb_save}
                    />
                </li>
            </ul>
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}

impl PaneEditQuest {
    pub(crate) fn go(&mut self, data: &Data, quest_id: QuestId) -> bool {
        if let Some(quest) = data.quest.get(&quest_id) {
            self.quest_id = quest_id;
            self.quest = quest.clone();
            true
        } else {
            false
        }
    }

    pub(crate) fn select_hidden(
        data: &Data,
        prefix: &str,
        current: Vis,
        cb_vis: Callback<Event>,
        cb_hid_c: Callback<Event>,
        cb_hid_f: Callback<Event>,
        cb_kill: Option<Callback<Event>>,
    ) -> Html {
        let na = AttrValue::from(format!("radio-{prefix}"));
        let na_vis = AttrValue::from(format!("radio-{prefix}-0"));
        let na_hid_c = AttrValue::from(format!("radio-{prefix}-1"));
        let na_hid_f = AttrValue::from(format!("radio-{prefix}-2"));
        let na_kill = AttrValue::from(format!("radio-{prefix}-3"));
        html! {
            <div class="btn-group" role="group">
                <input
                    type="radio"
                    class="btn-check"
                    name={na.clone()}
                    id={na_vis.clone()}
                    autocomplete="off"
                    checked={current == Vis::Visible}
                    onchange={cb_vis}
                />
                <label class={Vis::Visible.class_btn_outline()} for={na_vis}>{Vis::Visible.text(data)}</label>

                <input
                    type="radio"
                    class="btn-check"
                    name={na.clone()}
                    id={na_hid_c.clone()}
                    autocomplete="off"
                    checked={current == Vis::HiddenThisCampaign}
                    onchange={cb_hid_c}
                />
                <label class={Vis::HiddenThisCampaign.class_btn_outline()} for={na_hid_c}>{Vis::HiddenThisCampaign.text(data)}</label>

                <input
                    type="radio"
                    class="btn-check"
                    name={na.clone()}
                    id={na_hid_f.clone()}
                    autocomplete="off"
                    checked={current == Vis::HiddenForever}
                    onchange={cb_hid_f}
                />
                <label class={Vis::HiddenForever.class_btn_outline()} for={na_hid_f}>{Vis::HiddenForever.text(data)}</label>

                if let Some(cb_kill) = cb_kill {
                    <input
                        type="radio"
                        class="btn-check"
                        name={na}
                        id={na_kill.clone()}
                        autocomplete="off"
                        checked={false}
                        onchange={cb_kill}
                    />
                    <label class="btn btn-outline-danger" for={na_kill}>{BI::TRASH}{data.msg.ed_qu_remove()}</label>
                }
            </div>
        }
    }
}
