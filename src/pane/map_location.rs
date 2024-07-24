use crate::data::quest::QuestViewMode;
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::game::{LocationId, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::{callback_input_value, Modal};
use crate::route::Route;
use crate::ser::settings::EmptySer;
use yew::{html, Context, Html};
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgMapLocation {
    Note(String),
    ViewQuestNote(QuestId),
}

impl From<MsgMapLocation> for MsgApp {
    #[inline]
    fn from(msg: MsgMapLocation) -> Self {
        MsgApp::MsgLocation(msg)
    }
}

pub(crate) struct PaneMapLocation {
    location_id: LocationId,
    modal: Modal,
}

impl SubComponent for PaneMapLocation {
    type Message = MsgMapLocation;
    type Ser = EmptySer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            location_id: LocationId::prologue(), // anything will do
            modal: Modal::default(),
        }
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgMapLocation::Note(str) => {
                data.location.insert(self.location_id, str.into());
                UpdateResult::SaveGameData.into()
            }
            MsgMapLocation::ViewQuestNote(quest_id) => {
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
        let location_id = self.location_id;
        let location_note = data
            .location
            .get(&self.location_id)
            .map(ToString::to_string);
        let quests = data
            .quest_iter()
            .filter_map(|(quest_id, quest, quest_name)| {
                quest.view(
                    ctx,
                    quest_id,
                    quest_name,
                    data,
                    QuestViewMode::Location(location_id),
                    MsgMapLocation::ViewQuestNote,
                )
            });

        html! {
            <>
            <div class="d-flex align-items-center mb-4">
              <h2 class="h4 mb-0">
                {data.msg.location(self.location_id.name(data.quest_locale.language()))}
              </h2>
              <Button
                style={Color::Primary}
                class="ms-auto"
                onclick={ctx.link().callback(|_|MsgApp::Back)}
              >
                <i class="bi bi-chevron-left"></i>
                {data.msg.back()}
              </Button>
            </div>

            <div class="form-floating mb-5">
              <textarea class="form-control" placeholder={data.msg.str_note_placeholder()} id="floatingTextarea" onchange={callback_input_value(ctx, MsgMapLocation::Note)} value={location_note} style="height: 150px"/>
              <label for="floatingTextarea">{data.msg.ma_lo_location_note()}</label>
            </div>

            <div class="d-flex align-items-center mb-4">
              <h2 class="h4 mb-0">
                {data.msg.ma_lo_quests_header()}
              </h2>
              <Button
                style={Color::Primary}
                onclick={ctx.link().callback(move |_|MsgApp::Go(Route::MapNewQuest(location_id)))}
                class="ms-auto"
                children={data.msg.ma_lo_new_quest()}
              />
            </div>

            <table class="table table-hover align-middle">
              <thead>
                <tr>
                  <th>{data.msg.ma_lo_header_quest()}</th>
                  <th>{data.msg.ma_lo_header_hints()}</th>
                </tr>
              </thead>
              <tbody>
                {for quests}
              </tbody>
            </table>

            {self.modal.html(data)}
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}

impl PaneMapLocation {
    pub(crate) fn go(&mut self, location_id: LocationId) {
        self.location_id = location_id;
    }
}
