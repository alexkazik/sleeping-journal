use crate::data::encounter_type::EncounterType;
use crate::data::quest::{Quest, QuestState, QuestViewMode};
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::game::QuestId;
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::Modal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use yew::{html, Context, Html};

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub(crate) enum MsgTodo {
    SettingsShowKeywords(SettingsKeywords),
    SettingsTyp(Typ),
    ShowNote(QuestId),
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum SettingsKeywords {
    #[default]
    OnlyQuests = 1,
    Both = 3,
    OnlyKeywords = 2,
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum Typ {
    #[default]
    Active = 0,
    GainWithComplete = 1,
    GainWoComplete = 2,
    Unless = 3,
}

impl From<MsgTodo> for MsgApp {
    #[inline]
    fn from(msg: MsgTodo) -> Self {
        MsgApp::MsgTodo(msg)
    }
}

pub(crate) struct PaneTodo {
    show_keywords: SettingsKeywords,
    typ: Typ,
    modal: Modal,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PaneTodoSer {
    show_keywords: SettingsKeywords,
    typ: Typ,
}

impl SubComponent for PaneTodo {
    type Message = MsgTodo;
    type Ser = PaneTodoSer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            show_keywords: SettingsKeywords::OnlyQuests,
            typ: Typ::Active,
            modal: Modal::default(),
        }
    }

    fn reset_to_new(&mut self) {
        self.show_keywords = SettingsKeywords::OnlyQuests;
        self.typ = Typ::Active;
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgTodo::SettingsShowKeywords(show_keywords) => {
                self.show_keywords = show_keywords;
                UpdateResult::SaveSettings | UpdateResult::Render
            }
            MsgTodo::SettingsTyp(typ) => {
                self.typ = typ;
                UpdateResult::SaveSettings | UpdateResult::Render
            }
            MsgTodo::ShowNote(quest_id) => {
                if let Some(quest) = data.quest.get(&quest_id) {
                    self.modal.open(
                        &data
                            .msg
                            .str_note_modal_head(data.quest_locale.get(quest_id)),
                        &quest.note,
                    );
                }
                UpdateResult::empty()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let list = data.quest_iter().filter_map(|(qid, q, qname)| {
            let qk = match q.is_keyword(qid) {
                None => SettingsKeywords::Both,
                Some(false) => SettingsKeywords::OnlyQuests,
                Some(true) => SettingsKeywords::OnlyKeywords,
            };
            if (qk as u8) & (self.show_keywords as u8) == 0 {
                return None;
            }
            if q.state == QuestState::Removed {
                // nothing to do
                return None;
            }
            if !match self.typ {
                Typ::Active => q.state == QuestState::InGame,
                Typ::GainWithComplete => {
                    q.state == QuestState::NotFound
                        && q.contains_visible_encounter_type(EncounterType::Gain)
                        && q.contains_visible_encounter_type(EncounterType::Complete)
                }
                Typ::GainWoComplete => {
                    q.state == QuestState::NotFound
                        && q.contains_visible_encounter_type(EncounterType::Gain)
                        && !q.contains_visible_encounter_type(EncounterType::Complete)
                }
                Typ::Unless => {
                    q.state == QuestState::NotFound
                        && q.contains_visible_encounter_type(EncounterType::Unless)
                }
            } {
                return None;
            }
            q.view(
                ctx,
                qid,
                qname,
                data,
                QuestViewMode::Todo,
                MsgTodo::ShowNote,
            )
        });
        html! {
            <>
            <div>
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="keywordsX"
                        id="keywordsX0"
                        autocomplete="off"
                        checked={self.show_keywords == SettingsKeywords::OnlyQuests}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsShowKeywords(SettingsKeywords::OnlyQuests))}
                    />
                    <label class="btn btn-outline-primary" for="keywordsX0">{Quest::icon_quest()}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="keywordsX"
                        id="keywordsX1"
                        autocomplete="off"
                        checked={self.show_keywords == SettingsKeywords::Both}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsShowKeywords(SettingsKeywords::Both))}
                    />
                    <label class="btn btn-outline-primary" for="keywordsX1">{data.msg.todo_quests_and_keywords(Quest::icon_quest(), Quest::icon_keyword())}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="keywordsX"
                        id="keywordsX2"
                        autocomplete="off"
                        checked={self.show_keywords == SettingsKeywords::OnlyKeywords}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsShowKeywords(SettingsKeywords::OnlyKeywords))}
                    />
                    <label class="btn btn-outline-primary" for="keywordsX2">{Quest::icon_keyword()}</label>
                </div>
                {" "}
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX0"
                        autocomplete="off"
                        checked={self.typ == Typ::Active}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::Active))}
                    />
                    <label class="btn btn-outline-primary" for="typX0">{data.msg.todo_typ_in_game()}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX1"
                        autocomplete="off"
                        checked={self.typ == Typ::GainWithComplete}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::GainWithComplete))}
                    />
                    <label class="btn btn-outline-primary" for="typX1">{data.msg.todo_typ_gain_with_complete(EncounterType::Gain.icon_active(), EncounterType::Complete.icon_active())}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX2"
                        autocomplete="off"
                        checked={self.typ == Typ::GainWoComplete}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::GainWoComplete))}
                    />
                    <label class="btn btn-outline-primary" for="typX2">{data.msg.todo_typ_gain_wo_complete(EncounterType::Gain.icon_active(), EncounterType::Complete.icon_active())}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX3"
                        autocomplete="off"
                        checked={self.typ == Typ::Unless}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::Unless))}
                    />
                    <label class="btn btn-outline-primary" for="typX3">{EncounterType::Unless.icon_active()}</label>
                </div>
            </div>
            <table class="table table-hover align-middle mt-4">
              <thead>
                <tr>
                  <th>{data.msg.todo_header_quest()}</th>
                  <th>{data.msg.todo_header_hints()}</th>
                </tr>
              </thead>
              <tbody>
                {for list}
              </tbody>
            </table>

            {self.modal.html(data)}
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        PaneTodoSer {
            show_keywords: self.show_keywords,
            typ: self.typ,
        }
    }

    fn load(&mut self, stored: Self::Ser) {
        self.show_keywords = stored.show_keywords;
        self.typ = stored.typ;
    }
}
