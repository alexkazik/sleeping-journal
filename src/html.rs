use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::virtual_dom::{VNode, VText};
use yew::{html, AttrValue, Callback, Context, Html, NodeRef, TargetCast};

#[inline]
pub(crate) const fn text(content: &'static str) -> Html {
    VNode::VText(VText {
        text: AttrValue::Static(content),
    })
}

#[inline]
pub(crate) fn callback_input_value<F, M, S>(ctx: &Context<App>, f: F) -> Callback<Event>
where
    F: Fn(S) -> M + 'static,
    S: From<String>,
    M: Into<MsgApp>,
{
    ctx.link()
        .callback(move |e: Event| f(e.target_unchecked_into::<HtmlInputElement>().value().into()))
}

#[derive(Default)]
pub(crate) struct Modal {
    head: NodeRef,
    text: NodeRef,
    btn: NodeRef,
}

impl Modal {
    pub(crate) fn html(&self, data: &Data) -> Html {
        html! {
            <>
                // Modal
                <div class="modal fade" id="modal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                  <div class="modal-dialog">
                    <div class="modal-content">
                      <div class="modal-header">
                        <h1 class="modal-title fs-5" id="exampleModalLabel" ref={&self.head}></h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label={data.msg.str_close()}></button>
                      </div>
                      <div class="modal-body" ref={&self.text}>
                      </div>
                      <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{data.msg.close()}</button>
                      </div>
                    </div>
                  </div>
                </div>

                // Button trigger modal
                <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#modal" style="display: none" ref={&self.btn}></button>
            </>
        }
    }

    pub(crate) fn open(&self, head: &str, text: &str) {
        self.head
            .cast::<HtmlElement>()
            .unwrap()
            .set_inner_text(head);
        self.text
            .cast::<HtmlElement>()
            .unwrap()
            .set_inner_text(text);
        self.btn.cast::<HtmlElement>().unwrap().click();
    }
}
