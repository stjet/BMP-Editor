use yew::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Blob, Url, HtmlLinkElement, HtmlSelectElement};
use js_sys::{Uint8Array, Array};
use bmp_rust::bmp::BMP;
use gloo_utils::document;
use gloo_events::EventListener;
use gloo_console::log;
use std::collections::HashMap;

use crate::tools::ToolsTypes;

//keyboard shortcuts: ctrl+[ for tool left cycle, ctrl+] for tool right cycle, and then ctrl+shift+key for specific tools
//ctrl+z to redo, possibly ctrl+y to undo redo

#[derive(PartialEq, Properties)]
pub struct ImageActionsProps {
  pub show: bool,
  pub current_bmp: Option<BMP>,
  pub tool_change_callback: Callback<ToolsTypes>,
  pub selected_tool: ToolsTypes,
  pub keybinds: HashMap<String, KeybindActions>,
}

pub enum ImageActionsMessage {
  Show,
  Hide,
  ToolChange(ToolsTypes),
  SetKeybindsListener(Option<EventListener>),
}

#[derive(Clone, PartialEq)]
pub enum KeybindActions {
  Undo,
  PreviousTool,
  NextTool,
  ToolChange(ToolsTypes),
}

pub struct ImageActions {
  display: String,
  keybinds_listener: Option<EventListener>,
}

impl Component for ImageActions {
  type Message = ImageActionsMessage;
  type Properties = ImageActionsProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string(), keybinds_listener: None }
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
      },
      Self::Message::SetKeybindsListener(keybinds_listener) => {
        self.keybinds_listener = keybinds_listener;
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

    let fills_ref = NodeRef::default();
    let shapes_ref = NodeRef::default();
    let filters_ref = NodeRef::default();

    let keybinds_callback = {
      let filters_ref2 = filters_ref.clone();
      let shapes_ref2 = shapes_ref.clone();
      let fills_ref2 = fills_ref.clone();
      let keybinds = ctx.props().keybinds.clone();
      let selected_tool = ctx.props().selected_tool.clone();
      ctx.link().batch_callback(move |e: Event| {
        fn set_select(tool_type: &ToolsTypes, filters_select: HtmlSelectElement, shapes_select: HtmlSelectElement, fills_select: HtmlSelectElement) {
          filters_select.set_value("none-selected");
          shapes_select.set_value("none-selected");
          fills_select.set_value("none-selected");
          let select_id = tool_type.get_select_id();
          if select_id == "fills" {
            fills_select.set_value(&tool_type.to_string());
          } else if select_id == "shapes" {
            shapes_select.set_value(&tool_type.to_string());
          } else if select_id == "filters" {
            filters_select.set_value(&tool_type.to_string());
          }
        }
        let keyboard_event: KeyboardEvent = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let mut pressed_key: String = keyboard_event.key();
        if keyboard_event.ctrl_key() {
          pressed_key = "ctrl+".to_owned()+&pressed_key;
        }
        let bind = keybinds.get(&pressed_key);
        log!(pressed_key);
        if bind.is_some() {
          let tools_vec: Vec<ToolsTypes> = vec![ToolsTypes::NoneSelected, ToolsTypes::ClickFill, ToolsTypes::BucketFill, ToolsTypes::Line, ToolsTypes::Rect, ToolsTypes::Ellipse, ToolsTypes::Invert, ToolsTypes::Greyscale, ToolsTypes::Gaussian, ToolsTypes::Box];
          let filters_select: HtmlSelectElement = filters_ref2.cast().unwrap();
          let shapes_select: HtmlSelectElement = shapes_ref2.cast().unwrap();
          let fills_select: HtmlSelectElement = fills_ref2.cast().unwrap();
          match bind.unwrap() {
            KeybindActions::Undo => {
              //
            },
            KeybindActions::PreviousTool => {
              let tool_index = tools_vec.iter().position(|&item| item == selected_tool).unwrap();
              let new_tool_index;
              if tool_index == 0 {
                new_tool_index = tools_vec.len()-1;
              } else {
                new_tool_index = tool_index-1;
              }
              let tool_type = tools_vec[new_tool_index];
              set_select(&tool_type, filters_select, shapes_select, fills_select);
              return Some(Self::Message::ToolChange(tool_type));
            },
            KeybindActions::NextTool => {
              let tool_index = tools_vec.iter().position(|&item| item == selected_tool).unwrap();
              let new_tool_index;
              if tool_index == tools_vec.len()-1 {
                new_tool_index = 0;
              } else {
                new_tool_index = tool_index+1;
              }
              let tool_type = tools_vec[new_tool_index];
              set_select(&tool_type, filters_select, shapes_select, fills_select);
              return Some(Self::Message::ToolChange(tool_type));
            },
            KeybindActions::ToolChange(tool_type) => {
              set_select(&tool_type, filters_select, shapes_select, fills_select);
              return Some(Self::Message::ToolChange(*tool_type));
            },
          }
        }
        None
      })
    };

    let keybinds_listener = Some(EventListener::new(&document(), "keydown", move |e| keybinds_callback.emit(e.clone())));
    link.send_message(Self::Message::SetKeybindsListener(keybinds_listener));

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
        "line" => {
          tool_type = ToolsTypes::Line;
        },
        "rect" => {
          tool_type = ToolsTypes::Rect;
        },
        "ellipse" => {
          tool_type = ToolsTypes::Ellipse;
        },
        "greyscale" => {
          tool_type = ToolsTypes::Greyscale;
        },
        "gaussian" => {
          tool_type = ToolsTypes::Gaussian;
        },
        "box" => {
          tool_type = ToolsTypes::Box;
        },
        _ => {
          tool_type = ToolsTypes::NoneSelected;
        }
      }
      link2.send_message(Self::Message::ToolChange(tool_type));
    });

    let fills = {
      let filters_ref2 = filters_ref.clone();
      let shapes_ref2 = shapes_ref.clone();
      let select_callback2 = select_callback.clone();
      Callback::from(move |e: Event| {
        let filters_select: HtmlSelectElement = filters_ref2.cast().unwrap();
        let shapes_select: HtmlSelectElement = shapes_ref2.cast().unwrap();
        //set the other selects to non selected
        filters_select.set_value("none-selected");
        shapes_select.set_value("none-selected");
        select_callback2.emit(e);
      })
    };

    let shapes = {
      let filters_ref2 = filters_ref.clone();
      let fills_ref2 = fills_ref.clone();
      let select_callback2 = select_callback.clone();
      Callback::from(move |e: Event| {
        let filters_select: HtmlSelectElement = filters_ref2.cast().unwrap();
        let fills_select: HtmlSelectElement = fills_ref2.cast().unwrap();
        //set the other selects to non selected
        filters_select.set_value("none-selected");
        fills_select.set_value("none-selected");
        select_callback2.emit(e);
      })
    };

    let filters = {
      let shapes_ref2 = shapes_ref.clone();
      let fills_ref2 = fills_ref.clone();
      let select_callback2 = select_callback.clone();
      Callback::from(move |e: Event| {
        let shapes_select: HtmlSelectElement = shapes_ref2.cast().unwrap();
        let fills_select: HtmlSelectElement = fills_ref2.cast().unwrap();
        //set the other selects to non selected
        shapes_select.set_value("none-selected");
        fills_select.set_value("none-selected");
        select_callback2.emit(e);
      })
    };

    let contents = ctx.clone().props().clone().current_bmp.as_ref().unwrap_or(&BMP::new(1, 1, None)).contents.to_owned();

    let download_ref = NodeRef::default();

    let download = {
      let download_ref2 = download_ref.clone();
      let window = web_sys::window().unwrap();
      Callback::from(move |_| {
        //create blob
        let bmp_array: Array = Array::new();
        for i in 0..contents.len() {
          bmp_array.push(&JsValue::from(contents[i]));
        }
        let bmp_uint8array = Uint8Array::new(&JsValue::from(bmp_array));
        let bmp_array2 = Array::new();
        bmp_array2.push(&bmp_uint8array.buffer());
        let bmp_blob = Blob::new_with_u8_array_sequence(&bmp_array2).unwrap();
        //get object url
        let obj_url = Url::create_object_url_with_blob(&bmp_blob).unwrap();
        //download
        let download_link: HtmlLinkElement = download_ref2.clone().cast().unwrap();
        download_link.set_href(&obj_url);
        //prompt to set file name
        let file_name = window.prompt_with_message_and_default("File name?", "edited.bmp").unwrap_or(Some("edited.bmp".to_string())).unwrap();
        if file_name != "" {
          download_link.set_attribute("download", &file_name).unwrap();
          download_link.click();
        }
      })
    };

    html! {
      <div id={"image-actions-container"} style={"display: ".to_string()+&self.display}>
        <a ref={download_ref}></a>
        <select ref={fills_ref} class={"image-actions"} onchange={fills}>
          <option value={"none-selected"} selected={true}>{ "-- Fills --" }</option>
          <option value={"click-fill"}>{ "Click Fill" }</option>
          <option value={"bucket-fill"}>{ "Bucket Fill" }</option>
        </select>
        <select ref={shapes_ref} class={"image-actions"} onchange={shapes}>
          <option value={"none-selected"} selected={true}>{ "-- Shapes --" }</option>
          <option value={"line"}>{ "Line" }</option>
          <option value={"rect"}>{ "Rectangle" }</option>
          <option value={"ellipse"}>{ "Ellipse" }</option>
        </select>
        <select ref={filters_ref} class={"image-actions"} onchange={filters}>
          <option value={"none-selected"} selected={true}>{ "-- Filters --" }</option>
          <option value={"invert"}>{ "Invert" }</option>
          <option value={"greyscale"}>{ "Greyscale" }</option>
          <option value={"gaussian"}>{ "Gaussian Blur" }</option>
          <option value={"box"}>{ "Box Blur" }</option>
        </select>
        <button onclick={download} class={"image-actions"}>{ "Download" }</button>
        <br/>
      </div>
    }
  }
}
