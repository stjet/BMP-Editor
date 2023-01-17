use yew::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{Blob, Url, HtmlLinkElement};
use js_sys::{Uint8Array, Array};
use bmp_rust::bmp::BMP;

#[derive(PartialEq, Properties)]
pub struct ImageActionsProps {
  pub show: bool,
  pub current_bmp: Option<BMP>,
}

pub enum ImageActionsMessage {
  Show,
  Hide,
}

pub struct ImageActions {
  display: String,
}

impl Component for ImageActions {
  type Message = ImageActionsMessage;
  type Properties = ImageActionsProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string() }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::Hide => {
        self.display = "none".to_string();
        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();

    if self.display == "none".to_string() && ctx.props().show {
      link.send_message(Self::Message::Show);
    } else if self.display == "block".to_string() && !ctx.props().show {
      link.send_message(Self::Message::Hide);
    }

    let download_ref = NodeRef::default();
    let download_ref_2 = download_ref.clone();

    let contents = ctx.clone().props().clone().current_bmp.as_ref().unwrap_or(&BMP::new(1, 1, None)).contents.to_owned();

    let download_callback = Callback::from(move |_| {
      //create blob
      let bmp_array: Array = Array::new();
      for i in 0..contents.len() {
        bmp_array.push(&JsValue::from(contents[i]));
      }
      let bmp_uint8array = Uint8Array::new(&JsValue::from(bmp_array));
      let bmp_array_2 = Array::new();
      bmp_array_2.push(&bmp_uint8array.buffer());
      let bmp_blob = Blob::new_with_u8_array_sequence(&bmp_array_2).unwrap();
      //get object url
      let obj_url = Url::create_object_url_with_blob(&bmp_blob).unwrap();
      //download
      let download_link: HtmlLinkElement = download_ref_2.clone().cast().unwrap();
      download_link.set_href(&obj_url);
      download_link.set_attribute("download", "edited.bmp").unwrap();
      download_link.click();
    });

    let download = {
      download_callback.clone()
    };

    html! {
      <div style={"display: ".to_string()+&self.display}>
        <a ref={download_ref}></a>
        <button onclick={download}>{ "Download" }</button>
        <br/>
      </div>
    }
  }
}
