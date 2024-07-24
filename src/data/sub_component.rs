use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use yew::{Context, Html};

#[bitflags]
#[derive(Copy, Clone)]
#[repr(u32)]
pub(crate) enum UpdateResult {
    Render,
    SaveGameData,
    SaveSettings,
}

pub(crate) type UpdateResults = BitFlags<UpdateResult>;

impl UpdateResult {
    #[inline]
    pub(crate) const fn empty() -> UpdateResults {
        BitFlags::EMPTY
    }
}

pub(crate) trait SubComponent {
    type Message: Into<MsgApp>;
    type Ser: Default + Serialize + for<'de> Deserialize<'de>;

    fn create(ctx: &Context<App>) -> Self;
    fn reset_to_new(&mut self) {}

    fn update(&mut self, data: &mut Data, ctx: &Context<App>, msg: Self::Message) -> UpdateResults;
    fn view(&self, data: &Data, ctx: &Context<App>) -> Html;

    fn save(&self) -> Self::Ser;

    fn load(&mut self, stored: Self::Ser);
}
