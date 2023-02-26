use yew::prelude::*;
use web_sys::HtmlInputElement;
//use gloo_console::log;

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
  pub change_pixel_callback: Callback<[u8; 4]>,
}

pub enum PixelActionsMessage {
  Show,
  Hide,
  ChangePixel([u8; 4]),
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
      Self::Message::ChangePixel(color) => {
        let _ = ctx.props().change_pixel_callback.emit(color);
        false
      }
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
      let pc_input_ref = NodeRef::default();
      let pixel_info = ctx.props().pixel_info.as_ref().unwrap();
      let pixel_click_coords = pixel_info.coords.clone();
      let pixel_click_color = pixel_info.color.clone();
      let color_text = format!("({}, {}, {}, {})", pixel_click_color[0], pixel_click_color[1], pixel_click_color[2], pixel_click_color[3]);
      let coords_text = format!("Coords: ({}, {})", pixel_click_coords[0], pixel_click_coords[1]);
  
      let new_pixel = {
        let pc_input_ref2 = pc_input_ref.clone();
        ctx.link().callback(move |_| {
          //get new pixel color
          let pc_input: HtmlInputElement = pc_input_ref2.cast().unwrap();
          let new_color_vec: Vec<u8> = pc_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u8>().unwrap()).collect();
          let new_color: [u8; 4] = [new_color_vec[0], new_color_vec[1], new_color_vec[2], new_color_vec[3]];
          Self::Message::ChangePixel(new_color)
        })
      };
      
      html! {
        <div style={"display: ".to_string()+&self.display}>
          <span>{coords_text}</span>
          <br/>
          <label for="pixel-color">{"Color: "}</label>
          <input name="pixel-color" value={color_text} ref={pc_input_ref}/>
          <button onclick={new_pixel}>{ "Change" }</button>
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
