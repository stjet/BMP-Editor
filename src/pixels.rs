use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;

// create

#[derive(PartialEq, Properties)]
pub struct PixelsProps {
  pub current_bmp: Option<BMP>,
}

pub enum PixelsMessage {
  Show,
}

pub struct Pixels {
  display: String,
}

impl Component for Pixels {
  type Message = PixelsMessage;
  type Properties = PixelsProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string() }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Show => {
        self.display = "grid".to_string();
        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();

    if ctx.props().current_bmp.is_some() {
      if self.display == "none".to_string() {
        link.send_message(Self::Message::Show);
      }
      let unwrapped_bmp = ctx.props().current_bmp.as_ref().unwrap();
      let dib_header = unwrapped_bmp.get_dib_header();
      let width;
      if let Ok(unwrapped_dib_header) = dib_header {
        width = unwrapped_dib_header.width;
      } else {
        width = 1;
      }
      log!(width);
      let bmp_pixels = unwrapped_bmp.get_pixel_data().unwrap();
      let mut pixels_all = vec![];
      for row in bmp_pixels {
        for column in row {
          pixels_all.push(html! {
            <div class="pixel" style={"background-color: rgba(".to_string()+&column[0].to_string()+&",".to_string()+&column[1].to_string()+&",".to_string()+&column[2].to_string()+&",".to_string()+&column[3].to_string()+&")".to_string()}></div>
          });
        }
      } 
      html! {
        <div id="pixel-grid" style={"display: ".to_string()+&self.display+&"; grid-template-columns: ".to_string()+&std::iter::repeat("auto ").take(width as usize).collect::<String>()}>
          {for pixels_all}
        </div>
      }
    } else {
      html! {
        <div id="pixel-grid" style={"display: ".to_string()+&self.display}></div>
      }
    }
  }
}
