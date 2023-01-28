use yew::prelude::*;
use wasm_bindgen::{JsValue};
use web_sys::{Blob, Url, HtmlLinkElement, HtmlSelectElement};
use js_sys::{Uint8Array, Array};
use bmp_rust::bmp::BMP;

use crate::tools::ToolsTypes;

#[derive(PartialEq, Properties)]
pub struct ImageActionsProps {
  pub show: bool,
  pub current_bmp: Option<BMP>,
  pub tool_change_callback: Callback<ToolsTypes>,
}

pub enum ImageActionsMessage {
  Show,
  Hide,
  ToolChange(ToolsTypes),
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

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::Hide => {
        self.display = "none".to_string();
        true
      },
      Self::Message::ToolChange(tool_type) => {
        ctx.props().tool_change_callback.emit(tool_type);
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();

    if self.display == "none".to_string() && ctx.props().show {
      link.send_message(Self::Message::Show);
    } else if self.display == "block".to_string() && !ctx.props().show {
      link.send_message(Self::Message::Hide);
    }

    let download_ref = NodeRef::default();
    let download_ref_2 = download_ref.clone();

    let select_callback = Callback::from(move |e: Event| {
      let select: HtmlSelectElement = e.target_unchecked_into();
      let select_value = select.value();
      let tool_type: ToolsTypes; 
      match select_value.as_str() {
        "click-fill" => {
          tool_type = ToolsTypes::ClickFill;
        },
        "bucket-fill" => {
          tool_type = ToolsTypes::BucketFill;
        },
        "invert" => {
          tool_type = ToolsTypes::Invert;
        },
        _ => {
          tool_type = ToolsTypes::NoneSelected;
        }
      }
      link2.send_message(Self::Message::ToolChange(tool_type));
    });

    let select = {
      select_callback.clone()
    };

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
        <select class={"image-actions"} onchange={select}>
          <option value={"none-selected"} selected={true}>{ "-- Tools --" }</option>
          <option value={"click-fill"}>{ "Click Fill" }</option>
          <option value={"bucket-fill"}>{ "Bucket Fill" }</option>
          <option value={"invert"}>{ "Invert" }</option>
        </select>
        <button onclick={download} class={"image-actions"}>{ "Download" }</button>
        <br/>
      </div>
    }
  }
}
