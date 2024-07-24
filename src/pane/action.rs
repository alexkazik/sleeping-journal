use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState};
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::data::vis::Vis;
use crate::game::{LocationId, MsgLanguage, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::{callback_input_value, text};
use crate::pane::edit_quest::PaneEditQuest;
use crate::ser::settings::EmptySer;
use yew::{html, Context, Html};
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgAction {
    Perform(EncounterType),
    Hide(EncounterType, Vis, bool),
    HideQuest(Vis),
    Note(String),
}

impl From<MsgAction> for MsgApp {
    #[inline]
    fn from(msg: MsgAction) -> Self {
        MsgApp::MsgAction(msg)
    }
}

pub(crate) struct PaneAction {
    quest_id: QuestId,
    location_id: LocationId,
    is_map: bool,
}

impl SubComponent for PaneAction {
    type Message = MsgAction;
    type Ser = EmptySer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            quest_id: QuestId::cottage(),        // any value
            location_id: LocationId::prologue(), // any value
            is_map: true,                        // any value
        }
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgAction::Perform(encounter_type) => {
                match encounter_type {
                    EncounterType::Unless | EncounterType::When => None,
                    EncounterType::Gain => data
                        .quest
                        .get_mut(&self.quest_id)
                        .map(|quest| quest.state = QuestState::InGame),
                    EncounterType::Complete | EncounterType::Lose => data
                        .quest
                        .get_mut(&self.quest_id)
                        .map(|quest| quest.state = QuestState::Removed),
                };
                data.chain_msg.push_back(MsgApp::Back);
                UpdateResult::SaveGameData.into()
            }
            MsgAction::Hide(encounter_type, vis, back) => {
                if let Some(qle) = data
                    .quest
                    .get_mut(&self.quest_id)
                    .and_then(|quest| quest.encounter.get_mut(&self.location_id))
                    .and_then(|quest_location| quest_location.get_mut(encounter_type))
                {
                    qle.vis = vis;
                }
                if back {
                    data.chain_msg.push_back(MsgApp::Back);
                }
                UpdateResult::SaveGameData.into()
            }
            MsgAction::Note(note) => {
                let quest = data.quest.get_mut(&self.quest_id).unwrap();

                quest.note = note.into();

                UpdateResult::SaveGameData.into()
            }
            MsgAction::HideQuest(vis) => {
                if let Some(q) = data.quest.get_mut(&self.quest_id) {
                    q.vis = vis;
                }

                data.chain_msg.push_back(MsgApp::Back);

                UpdateResult::SaveGameData.into()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let quest = &data.quest[&self.quest_id];
        let quest_location = quest.encounter.get(&self.location_id);
        let ignore_visibility = self.is_map;

        let (quest_location_iter, active_quest_location) =
            quest_location.map_or(Default::default(), |quest_location| {
                (
                    quest_location.iter(),
                    quest_location.get_active(quest, data, ignore_visibility),
                )
            });

        let enc = quest_location_iter
            .filter(|(_, qle)| qle.vis == Vis::Visible || ignore_visibility)
            .map(|(encounter_type, qle)| {
                let active = active_quest_location[encounter_type];
                let encounter_type = *encounter_type;
                let message = et2msg(data.msg, encounter_type, quest, active);
                let message = if let Some(message) = message {
                    html! {
                        <>
                            if active {
                                <Button style={Color::Success} onclick={ctx.link().callback(move |_|MsgAction::Perform(encounter_type))} children={text(message)} />
                            } else {
                                <Button style={Color::Success} outline={true} children={text(message)} />
                            }
                            <br/>
                        </>
                        }
                } else {
                    Html::default()
                };

                if self.is_map {
                    html! {
                        <li class="list-group-item">
                            {data.msg.location_with_icon(  self.location_id.name(data.quest_locale.language()), encounter_type.icon(active))}<br/>
                            {message}
                            {PaneEditQuest::select_hidden(
                                data,
                                &format!("encounter-{}-{}",self.location_id.raw(), encounter_type.raw()),
                                qle.vis,
                                ctx.link().callback(move |_|MsgAction::Hide(encounter_type, Vis::Visible, false)),
                                ctx.link().callback(move |_|MsgAction::Hide(encounter_type, Vis::HiddenThisCampaign, false)),
                                ctx.link().callback(move |_|MsgAction::Hide(encounter_type, Vis::HiddenForever, false)),
                                None,
                            )}<br/>
                        </li>
                    }
                } else {
                    html! {
                    <li class="list-group-item">
                        {message}
                        <Button style={Vis::HiddenThisCampaign.to_style()} onclick={ctx.link().callback(move |_|MsgAction::Hide(encounter_type, Vis::HiddenThisCampaign, true))}>{data.msg.todo_do_hide_this_campaign(encounter_type.icon(active))}</Button>
                        <br/>
                        <Button style={Vis::HiddenForever.to_style()} onclick={ctx.link().callback(move |_|MsgAction::Hide(encounter_type, Vis::HiddenForever, true))}>{data.msg.todo_do_hide_forever(encounter_type.icon(active))}</Button>
                    </li>
                }
                }
            });

        html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)}>{data.msg.back()}</Button>
                </li>
                <li class="list-group-item">
                    {data.msg.acti_quest_header(data.quest_locale.get( self.quest_id), quest.state.text(data))}
                </li>
                <li class="list-group-item">
                    {data.msg.note_quest()}<br/>
                    <div class="form-floating">
                      <textarea class="form-control" placeholder={data.msg.str_note_placeholder()} id="floatingTextarea" onchange={callback_input_value(ctx, MsgAction::Note)} value={quest.note.clone()} style="height: 150px"/>
                      <label for="floatingTextarea">{data.msg.note()}</label>
                    </div>
                </li>
                if !self.is_map {
                    <li class="list-group-item">
                        {data.msg.location( self.location_id.name(data.quest_locale.language()))}
                    </li>
                }
                { for enc }
                if !self.is_map {
                    <li class="list-group-item">
                        <Button style={Vis::HiddenThisCampaign.to_style()} onclick={ctx.link().callback(move |_|MsgAction::HideQuest(Vis::HiddenThisCampaign))}>{data.msg.todo_do_hide_quest_this_campaign()}</Button>
                        <br/>
                        <Button style={Vis::HiddenForever.to_style()} onclick={ctx.link().callback(move |_|MsgAction::HideQuest(Vis::HiddenForever))}>{data.msg.todo_do_hide_quest_forever()}</Button>
                    </li>
                }
                <li class="list-group-item">
                    {data.msg.acti_hidden_hint()}
                </li>
            </ul>
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}

fn et2msg(
    msg_language: MsgLanguage,
    encounter_type: EncounterType,
    quest: &Quest,
    active: bool,
) -> Option<&'static str> {
    match encounter_type {
        EncounterType::Unless | EncounterType::When => {
            None // will not be displayed
        }
        EncounterType::Gain => {
            if active {
                Some(msg_language.str_acti_gain_quest())
            } else if quest.state != QuestState::NotFound {
                Some(msg_language.str_acti_gain_quest_found())
            } else {
                Some(msg_language.str_acti_gain_quest_no_pre())
            }
        }
        EncounterType::Complete => {
            if active {
                Some(msg_language.str_acti_complete_quest())
            } else {
                Some(msg_language.str_acti_complete_quest_no())
            }
        }
        EncounterType::Lose => {
            if active {
                Some(msg_language.str_acti_lose_quest())
            } else {
                Some(msg_language.str_acti_lose_quest_no())
            }
        }
    }
}

impl PaneAction {
    pub(crate) fn go(
        &mut self,
        data: &Data,
        quest_id: QuestId,
        location_id: LocationId,
        is_map: bool,
    ) -> bool {
        if let Some(q) = data.quest.get(&quest_id) {
            if q.encounter.contains_key(&location_id) {
                self.quest_id = quest_id;
                self.location_id = location_id;
                self.is_map = is_map;
                return true;
            }
        }
        false
    }
}
