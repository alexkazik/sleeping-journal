use crate::data::encounter_type::EncounterType;
use crate::data::note::Note;
use crate::data::quest::Quest;
use crate::global::data::Data;
use yew::Html;

pub(crate) fn info_view(data: &Data) -> Html {
    yew::html! {
        <ul class="list-group">
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_data_head()}</h2>
            </li>
            <li class="list-group-item">
                {data.msg.info_data_body()}
            </li>
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_icon_head()}</h2>
            </li>
            <li class="list-group-item">
                {data.msg.info_icon_active(Quest::icon_quest())}<br/>
                {data.msg.info_icon_keyword(Quest::icon_keyword())}<br/>
                {data.msg.info_icon_gain(EncounterType::Gain.icon_active())}<br/>
                {data.msg.info_icon_when(EncounterType::When.icon_active())}<br/>
                {data.msg.info_icon_complete(EncounterType::Complete.icon_active())}<br/>
                {data.msg.info_icon_lose(EncounterType::Lose.icon_active())}<br/>
                {data.msg.info_icon_unless(EncounterType::Unless.icon_active())}<br/>
                {data.msg.info_icon_note(Note::icon())}<br/>
            </li>
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_link_head()}</h2>
            </li>
            <li class="list-group-item">
                <a href="https://sleeping-gods.rulepop.com/" target="_blank">{data.msg.info_link_rules()}</a>
            </li>
        </ul>
    }
}
