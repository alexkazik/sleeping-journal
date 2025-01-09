use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState};
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::{callback_input_value, text};
use crate::ser::settings::EmptySer;
use std::str::FromStr;
use web_sys::HtmlInputElement;
use yew::{Context, Html, NodeRef, html};
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgMapNewQuest {
    SelectEncounterType(EncounterType),
    SelectQuest(QuestId),
    SetPrerequisite(String),
    Save,
}

impl From<MsgMapNewQuest> for MsgApp {
    #[inline]
    fn from(msg: MsgMapNewQuest) -> Self {
        MsgApp::MsgNewQuest(msg)
    }
}

pub(crate) struct PaneMapNewQuest {
    page: Page,
    location_id: LocationId,
    encounter_type: EncounterType,
    quest_id: QuestId,
    prerequisite: Option<QuestId>,
    note_input: NodeRef,
}

pub(crate) enum Page {
    SelectEncounterType,
    SelectQuest,
    NoteAndPrerequisite,
}

fn nqt_allow(nqt: EncounterType, data: &Data, quest_id: QuestId) -> bool {
    match nqt {
        EncounterType::Gain => data
            .quest
            .get(&quest_id)
            .map_or(true, |q| q.state == QuestState::NotFound),
        EncounterType::Complete | EncounterType::Lose => data
            .quest
            .get(&quest_id)
            .map_or(false, |q| q.state == QuestState::InGame),
        EncounterType::When | EncounterType::Unless => true,
    }
}

fn nqt_head(nqt: EncounterType) -> &'static str {
    match nqt {
        EncounterType::Gain => "Gain",
        EncounterType::Complete => "Complete",
        EncounterType::Lose => "Lose",
        EncounterType::When => "When",
        EncounterType::Unless => "Unless",
    }
}

impl SubComponent for PaneMapNewQuest {
    type Message = MsgMapNewQuest;
    type Ser = EmptySer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            page: Page::SelectEncounterType,       // anything will do
            location_id: LocationId::prologue(),   // anything will do
            encounter_type: EncounterType::Unless, // anything will do
            quest_id: QuestId::cottage(),          // anything will do
            prerequisite: None,                    // anything will do
            note_input: NodeRef::default(),
        }
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgMapNewQuest::SelectEncounterType(nqt) => {
                self.encounter_type = nqt;
                self.page = Page::SelectQuest;
                UpdateResult::Render.into()
            }
            MsgMapNewQuest::SelectQuest(quest_id) => {
                self.quest_id = quest_id;
                self.prerequisite = None;
                self.page = Page::NoteAndPrerequisite;
                UpdateResult::Render.into()
            }
            MsgMapNewQuest::SetPrerequisite(prerequisite) => {
                self.prerequisite = usize::from_str(&prerequisite)
                    .ok()
                    .and_then(QuestId::from_raw);
                UpdateResult::empty()
            }
            MsgMapNewQuest::Save => {
                let quest = data.quest.entry(self.quest_id).or_default();
                match self.encounter_type {
                    EncounterType::Gain => quest.state = QuestState::InGame,
                    EncounterType::Complete | EncounterType::Lose => {
                        quest.state = QuestState::Removed;
                    }
                    EncounterType::When | EncounterType::Unless => (),
                }
                quest.note = self
                    .note_input
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .into();
                let encounters = quest.encounter.entry(self.location_id).or_default();
                encounters.insert(self.encounter_type, self.prerequisite, Vis::Visible);
                data.chain_msg.push_back(MsgApp::Back);
                UpdateResult::SaveGameData.into()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        match self.page {
            Page::SelectEncounterType => self.view_new_quest(data, ctx),
            Page::SelectQuest => self.view_new_quest2(data, ctx),
            Page::NoteAndPrerequisite => self.view_new_quest3(data, ctx),
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}

impl PaneMapNewQuest {
    pub(crate) fn go(&mut self, location_id: LocationId) {
        self.location_id = location_id;
        self.page = Page::SelectEncounterType;
    }

    pub(crate) fn view_new_quest(&self, data: &Data, ctx: &Context<App>) -> Html {
        html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)} children={data.msg.back()} />
                </li>
                <li class="list-group-item">
                    {data.msg.location( self.location_id.name(data.quest_locale.language()))}
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgMapNewQuest::SelectEncounterType(EncounterType::Gain))} children={data.msg.ma_nq_gain()} />
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgMapNewQuest::SelectEncounterType(EncounterType::Complete))} children={data.msg.ma_nq_complete()} />
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgMapNewQuest::SelectEncounterType(EncounterType::Lose))} children={data.msg.ma_nq_lose()} />
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgMapNewQuest::SelectEncounterType(EncounterType::When))} children={data.msg.ma_nq_when()} />
                </li>
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgMapNewQuest::SelectEncounterType(EncounterType::Unless))} children={data.msg.ma_nq_unless()} />
                </li>
            </ul>
        }
    }

    pub(crate) fn view_new_quest2(&self, data: &Data, ctx: &Context<App>) -> Html {
        let quests = data.quest_all_iter().filter_map(|(quest_id, quest_name)| {
            if nqt_allow(self.encounter_type, data, quest_id) {
                Some(html! {
                    <li class="list-group-item">
                        <Button style={Color::Primary} onclick={ctx.link().callback(move |_|MsgMapNewQuest::SelectQuest(quest_id))} children={quest_name} />
                    </li>
                })
            } else {
                None
            }
        });
        html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)} children={data.msg.back()} />
                </li>
                <li class="list-group-item">
                    {data.msg.location( self.location_id.name(data.quest_locale.language()))}
                </li>
                <li class="list-group-item">
                    <h3>{nqt_head(self.encounter_type)}</h3>
                </li>
                {for quests}
            </ul>
        }
    }

    pub(crate) fn view_new_quest3(&self, data: &Data, ctx: &Context<App>) -> Html {
        let mut prev = if self.encounter_type == EncounterType::Gain {
            data.quest
                .iter()
                .filter_map(|(quest_id, quest)| {
                    if quest.state == QuestState::Removed
                        && quest.encounter.iter().any(|(location_id, es)| {
                            *location_id == self.location_id
                                && (es.contains_key(EncounterType::Complete)
                                    || es.contains_key(EncounterType::Lose))
                        })
                    {
                        Some(*quest_id)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        };
        prev.sort_by_cached_key(|quest_id| data.quest_locale.get(*quest_id));
        let mut prerequisite = Vec::new();
        if !prev.is_empty() {
            prerequisite.push(html! {
                <div class="form-check">
                    <input class="form-check-input" type="radio" name="selectPrerequisite" id="selectPrerequisiteOff" value="" checked={true} onchange={callback_input_value(ctx, MsgMapNewQuest::SetPrerequisite)} />
                    <label class="form-check-label" for="selectPrerequisiteOff">
                        {data.msg.ma_nq_no_prerequisite()}
                    </label>
                </div>
            });
            for (pos, quest_id) in prev.into_iter().enumerate() {
                let id = format!("selectPrerequisite{pos}");
                let name = data.quest_locale.get(quest_id);
                prerequisite.push(html! {
                    <div class="form-check">
                        <input class="form-check-input" type="radio" name="selectPrerequisite" id={id.clone()} value={quest_id.raw().to_string()} onchange={callback_input_value(ctx, MsgMapNewQuest::SetPrerequisite)} />
                        <label class="form-check-label" for={id}>
                            {text(name)}
                        </label>
                    </div>
            });
            }
        }

        let quest_note = data
            .quest
            .get(&self.quest_id)
            .map_or("", |quest| &quest.note)
            .to_string();

        html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)} children={data.msg.back()} />
                </li>
                <li class="list-group-item">
                    {data.msg.location( self.location_id.name(data.quest_locale.language()))}
                </li>
                <li class="list-group-item">
                    <h3>{nqt_head(self.encounter_type)}</h3>
                </li>
                <li class="list-group-item">
                    <h3>{data.msg.quest_header( data.quest_locale.get(self.quest_id), Quest::icon_from_raw(self.quest_id))}</h3>
                </li>
                if !prerequisite.is_empty() {
                    <li class="list-group-item">
                    {data.msg.ma_nq_prerequisite()}
                    {for prerequisite}
                    </li>
                }
                <li class="list-group-item">
                    {data.msg.note_quest()}<br/>
                    <div class="form-floating">
                      <textarea class="form-control" placeholder={data.msg.str_note_placeholder()} id="floatingTextarea" value={quest_note} style="height: 150px" ref={&self.note_input} />
                      <label for="floatingTextarea">{data.msg.note()}</label>
                    </div>
                </li>
                <li class="list-group-item">
                    <Button style={Color::Success} onclick={ctx.link().callback(|_|MsgMapNewQuest::Save)} children={data.msg.ma_nq_save()} />
                </li>
            </ul>
        }
    }
}
