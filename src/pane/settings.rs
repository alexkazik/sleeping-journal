use crate::data::quest::QuestState;
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::data::vis::Vis;
use crate::game::{GameLanguage, MsgLanguage, QuestId};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::text;
use crate::ser::csv::MyError;
use base64::Engine;
use gloo_file::File;
use gloo_file::callbacks::FileReader;
use serde::{Deserialize, Serialize};
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlSelectElement};
use yew::{AttrValue, Context, Html, NodeRef, TargetCast, html};
use yew_bootstrap::component::form::{FormControl, FormControlType, SelectOption};
use yew_bootstrap::component::{
    Alert, Button, ButtonGroup, Modal, ModalBody, ModalFooter, ModalHeader,
};
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgSettings {
    GameLanguage(String),
    MsgLanguage(String),
    Save,
    ClickLoad,
    StartLoad,
    LoadFinished(Vec<u8>),
    CloseAlert,
    DarkMode(bool),
    NewCampaign,
    Clear,
}

impl From<MsgSettings> for MsgApp {
    #[inline]
    fn from(msg: MsgSettings) -> Self {
        MsgApp::MsgSettings(msg)
    }
}

pub(crate) struct PaneSettings {
    save_element: NodeRef,
    load_element: NodeRef,
    file_reader: Option<FileReader>,
    alert: Option<(Color, &'static str, Option<String>)>,
    dark_mode: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct PaneSettingsSer {
    dark_mode: bool,
}

impl SubComponent for PaneSettings {
    type Message = MsgSettings;
    type Ser = PaneSettingsSer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            save_element: NodeRef::default(),
            load_element: NodeRef::default(),
            file_reader: None,
            alert: None,
            dark_mode: false,
        }
    }

    fn update(&mut self, data: &mut Data, ctx: &Context<App>, msg: Self::Message) -> UpdateResults {
        match msg {
            MsgSettings::GameLanguage(language) => {
                if let Some((ql, _)) = GameLanguage::iter()
                    .zip(GameLanguage::names())
                    .find(|(_, n)| **n == language)
                {
                    data.quest_locale.set_language(ql);
                }
                UpdateResult::Render | UpdateResult::SaveGameData
            }
            MsgSettings::MsgLanguage(language) => {
                if let Some((ml, _)) = MsgLanguage::iter()
                    .zip(MsgLanguage::names())
                    .find(|(_, n)| **n == language)
                {
                    data.msg = ml;
                }
                UpdateResult::Render | UpdateResult::SaveSettings
            }
            MsgSettings::Save => {
                let csv = data.save_csv();
                let mut file = String::with_capacity((50 + csv.len()) * 6 / 8);
                file.push_str("data:text/csv;charset=UTF-8;base64,");
                base64::engine::general_purpose::STANDARD.encode_string(csv, &mut file);
                let link: HtmlElement = self.save_element.cast::<HtmlElement>().unwrap();
                let _ = link.set_attribute("href", &file);
                let _ = link.set_attribute(
                    "download",
                    &format!(
                        "sgh_{}.csv",
                        chrono::offset::Local::now().format("%Y-%m-%d_%H:%M")
                    ),
                );
                link.click();

                UpdateResult::empty()
            }
            MsgSettings::ClickLoad => {
                let load_form: HtmlElement = self.load_element.cast::<HtmlElement>().unwrap();
                load_form.click();
                UpdateResult::empty()
            }
            MsgSettings::StartLoad => {
                let load_form: HtmlInputElement = self
                    .load_element
                    .cast::<HtmlInputElement>()
                    .expect("cast failed");
                if let Some(file) = load_form.files().and_then(|x| x.get(0)) {
                    let task = {
                        let link = ctx.link().clone();
                        gloo_file::callbacks::read_as_bytes(&File::from(file), move |res| {
                            link.send_message(MsgSettings::LoadFinished(
                                res.expect("failed to read file"),
                            ));
                        })
                    };
                    self.file_reader = Some(task);
                }
                load_form.set_value("");
                UpdateResult::empty()
            }
            MsgSettings::LoadFinished(file) => {
                match data.load_csv(&file) {
                    Ok(err) => {
                        if !err.is_empty() {
                            self.alert = Some((
                                Color::Info,
                                "The following rows have errors and are skipped:",
                                Some(err),
                            ));
                        }
                    }
                    Err(err) => {
                        self.alert = Some((
                            Color::Danger,
                            match err {
                                MyError::CsvError => "CsvError",
                                MyError::Header => "Header",
                                MyError::Language => "Language",
                            },
                            None,
                        ));
                    }
                }
                self.file_reader = None;
                data.chain_msg.push_back(MsgApp::ResetToNew);
                UpdateResult::SaveGameData.into()
            }
            MsgSettings::CloseAlert => {
                self.alert = None;
                UpdateResult::empty()
            }
            MsgSettings::DarkMode(dark_mode) => {
                self.dark_mode = dark_mode;
                self.set_dark_mode();
                UpdateResult::SaveSettings.into()
            }
            MsgSettings::NewCampaign => {
                for (quest_id, quest) in &mut data.quest {
                    if *quest_id == QuestId::raid() || *quest_id == QuestId::cottage() {
                        quest.state = QuestState::InGame;
                    } else {
                        quest.state = QuestState::NotFound;
                    }
                    if quest.vis == Vis::HiddenThisCampaign {
                        quest.vis = Vis::Visible;
                    }
                    for quest_location in quest.encounter.values_mut() {
                        for (_, e) in quest_location {
                            if e.vis == Vis::HiddenThisCampaign {
                                e.vis = Vis::Visible;
                            }
                        }
                    }
                }
                data.chain_msg.push_back(MsgApp::ResetToNew);
                UpdateResult::SaveGameData.into()
            }
            MsgSettings::Clear => {
                data.reset();
                data.chain_msg.push_back(MsgApp::ResetToNew);
                UpdateResult::SaveGameData.into()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let ql = Self::language_settings(
            data,
            ctx,
            data.quest_locale.language(),
            GameLanguage::iter().zip(GameLanguage::names()),
            "input-select-quest-language",
            "Language of the game",
            MsgSettings::GameLanguage,
        );
        let ml = Self::language_settings(
            data,
            ctx,
            data.msg,
            MsgLanguage::iter().zip(MsgLanguage::names()),
            "input-select-msg-language",
            "Language of the website",
            MsgSettings::MsgLanguage,
        );
        html! {
            <>
            if let Some(alert) = &self.alert {
                <Alert style={alert.0.clone()}>
                    {text(alert.1)}
                    if let Some(extra) = &alert.2 {
                        <br/>
                        {extra}
                    }
                    <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label={data.msg.str_close()} onclick={ctx.link().callback(|_|MsgSettings::CloseAlert)}></button>
                </Alert>
            }
            <ul class="list-group">
                <li class="list-group-item">
                    <ButtonGroup>
                        <input
                            type="radio"
                            class="btn-check"
                            name="dark_mode"
                            id="dark_mode0"
                            autocomplete="off"
                            checked={!self.dark_mode }
                            onchange={ctx.link().callback(|_|MsgSettings::DarkMode(false))}
                        />
                        <label class="btn btn-outline-primary" for="dark_mode0">{data.msg.sett_light_mode()}</label>

                        <input
                            type="radio"
                            class="btn-check"
                            name="dark_mode"
                            id="dark_mode1"
                            autocomplete="off"
                            checked={self.dark_mode}
                            onchange={ctx.link().callback(|_|MsgSettings::DarkMode(true))}
                        />
                        <label class="btn btn-outline-primary" for="dark_mode1">{data.msg.sett_dark_mode()}</label>
                    </ButtonGroup>
                </li>
                {ql}
                {ml}
                <li class="list-group-item">
                    {data.msg.sett_data()}<br/>
                    <Button
                        text={data.msg.str_sett_data_load()}
                        onclick={ctx.link().callback(|_|MsgSettings::ClickLoad)}
                    />
                    {" "}
                    <Button
                        text={data.msg.str_sett_data_save()}
                        onclick={ctx.link().callback(|_|MsgSettings::Save)}
                    />
                    {" "}
                    <Button
                        style={Color::Warning}
                        text={data.msg.str_sett_data_new_campaign()}
                        modal_target="NewCampaignModal"
                    />
                    {" "}
                    <Button
                        style={Color::Danger}
                        text={data.msg.str_sett_data_clear()}
                        modal_target="ClearModal"
                    /><br/>
                </li>
            </ul>
            <input
                ref={&self.load_element}
                type="file"
                accept="text/csv"
                onchange={ctx.link().callback(|_|MsgSettings::StartLoad)}
                style="display: none"
            />
            <a
                ref={&self.save_element}
                style="display: none"
            />
            <Modal id="NewCampaignModal">
                <ModalHeader title={data.msg.str_sett_model_new_campaign_head()} />
                <ModalBody>
                    <p>{data.msg.sett_model_new_campaign_body()}</p>
                </ModalBody>
                <ModalFooter>
                    <Button style={Color::Secondary} modal_dismiss={true}>{data.msg.close()}</Button>
                    <Button style={Color::Warning} modal_dismiss={true} onclick={ctx.link().callback(|_|MsgSettings::NewCampaign)}>{data.msg.sett_data_new_campaign()}</Button>
                </ModalFooter>
            </Modal>
            <Modal id="ClearModal">
                <ModalHeader title={data.msg.str_sett_model_clear_head()} />
                <ModalBody>
                    <p>{data.msg.sett_model_clear_body()}</p>
                </ModalBody>
                <ModalFooter>
                    <Button style={Color::Secondary} modal_dismiss={true}>{data.msg.close()}</Button>
                    <Button style={Color::Danger} modal_dismiss={true} onclick={ctx.link().callback(|_|MsgSettings::Clear)}>{data.msg.sett_data_clear()}</Button>
                </ModalFooter>
            </Modal>
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        PaneSettingsSer {
            dark_mode: self.dark_mode,
        }
    }

    fn load(&mut self, stored: Self::Ser) {
        self.dark_mode = stored.dark_mode;
        self.set_dark_mode();
    }
}

impl PaneSettings {
    fn language_settings<T, I, M>(
        data: &Data,
        ctx: &Context<App>,
        current: T,
        iter: I,
        id: &'static str,
        label: &'static str,
        msg: M,
    ) -> Html
    where
        I: IntoIterator<Item = (T, &'static str)>,
        M: Fn(String) -> MsgSettings + 'static,
        T: Copy + PartialEq,
    {
        let mut incomplete = false;
        let ql = iter.into_iter().map(|(ql, n)| {
            if n.ends_with('*') {
                incomplete = true;
            }
            html! {
              <SelectOption
                value={AttrValue::Static(n)}
                label={AttrValue::Static(n)}
                selected={ql == current}
            />
            }
        });

        let onchange_ql = ctx.link().callback(move |event: Event| {
            msg(event.target_unchecked_into::<HtmlSelectElement>().value())
        });

        html! {
            <li class="list-group-item">
                <FormControl
                    id={AttrValue::Static(id)}
                    ctype={ FormControlType::Select}
                    label={AttrValue::Static(label)}
                    onchange={onchange_ql}
                >
                {for ql}
                </FormControl>
                <br/>
                <a href="https://github.com/alexkazik/sleeping-journal#Translation" target="_blank">{data.msg.sett_translation_help()}</a>
                if incomplete {
                    {", "}{data.msg.sett_translation_incomplete()}
                }
            </li>
        }
    }

    fn set_dark_mode(&self) {
        let _ = gloo_utils::document_element().set_attribute(
            "data-bs-theme",
            if self.dark_mode { "dark" } else { "light" },
        );
    }
}
