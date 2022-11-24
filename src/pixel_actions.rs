use yew::prelude::*;
use gloo_console::log;

//info and actions of a specific pixel
//coords, color, in future: option to change color

#[derive(PartialEq, Clone)]
pub struct PixelInfo {
  pub color: [u8; 4],
  pub coords: [u16; 2],
}

#[derive(PartialEq, Properties)]
pub struct PixelActionsProps {
  pub pixel_info: Option<PixelInfo>,
  pub show: bool,
}

pub enum PixelActionsMessage {
  Show,
  Hide,
  ClickUpdate,
}

pub struct PixelActions {
  display: String,
}

impl Component for PixelActions {
  type Message = PixelActionsMessage;
  type Properties = PixelActionsProps;

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
      Self::Message::ClickUpdate => {
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

    if ctx.props().pixel_info.is_some() {
      let pixel_info = ctx.props().pixel_info.as_ref().unwrap();
      let pixel_click_coords = pixel_info.coords.clone();
      let pixel_click_color = pixel_info.color.clone();
      let color_text = format!("Color: ({}, {}, {}, {})", pixel_click_color[0], pixel_click_color[1], pixel_click_color[2], pixel_click_color[3]);
      let coords_text = format!("Coords: ({}, {})", pixel_click_coords[0], pixel_click_coords[1]);
      html! {
        <div style={"display: ".to_string()+&self.display}>
          <span>{coords_text}</span>
          <br/>
          <span>{color_text}</span>
        </div>
      }
    } else {
      html! {
        <div style={"display: none;"}>
        </div>
      }
    }
  }
}
