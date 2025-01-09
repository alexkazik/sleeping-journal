use crate::data::sub_component::{SubComponent, UpdateResults};
use crate::data::vis::Vis;
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::route::Route;
use crate::ser::settings::EmptySer;
use yew::{Context, Html, html};
use yew_bootstrap::component::{Alert, Button};
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgEdit {}

impl From<MsgEdit> for MsgApp {
    #[inline]
    fn from(msg: MsgEdit) -> Self {
        MsgApp::MsgEditList(msg)
    }
}

pub(crate) struct PaneEdit {}

impl SubComponent for PaneEdit {
    type Message = MsgEdit;
    type Ser = EmptySer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        _data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {}
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let quests = data.quest_iter().map(|(quest_id, quest, quest_name)| {
            let max_vis = quest.encounter.values().map(|ql| ql.values().map(|qle| qle.vis).max().unwrap_or(Vis::Visible)).max().unwrap_or(Vis::Visible).max(quest.vis);
            html! {
                <li class="list-group-item">
                    <Button style={max_vis.to_style()}
                        onclick={ctx.link().callback(move |_|MsgApp::Go(Route::EditQuest(quest_id)))}
                    >
                        {data.msg.quest_header(quest_name, quest.icon(quest_id))}
                    </Button>
                </li>
            }
        });
        html! {
            <>
            <Alert style={Color::Info}>
                {data.msg.edit_hint()}
            </Alert>
            <ul class="list-group">
                {for quests}
            </ul>
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}
