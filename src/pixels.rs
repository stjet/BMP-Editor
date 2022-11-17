use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;

// create

#[derive(PartialEq, Properties)]
pub struct PixelsProps {
  pub current_bmp: Option<BMP>,
  pub send_pixel_click: Callback<[u16; 2]>,
}

pub enum PixelsMessage {
  Show,
  SendClick([u16; 2]),
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
      Self::Message::SendClick(coords) => {
        let _ = ctx.props().send_pixel_click.emit(coords);
        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();

    /*
    let pixel_click_callback = Callback::from(move |_| {
      //pixel_click
      link2.send_message(Self::Message::SendClick([1,2]));
    });
    */
    

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
      let bmp_pixels = unwrapped_bmp.get_pixel_data().unwrap();
      let mut pixels_all = vec![];
      let mut column_num: u16 = 0;
      let mut row_num: u16 = 0;
      for row in bmp_pixels {
        for column in row {
          let link2 = ctx.link().clone();
          pixels_all.push(html! {
            <div class="pixel" onclick={move |_|{ link2.send_message(Self::Message::SendClick([column_num as u16, row_num as u16])); }} style={"background-color: rgba(".to_string()+&column[0].to_string()+&",".to_string()+&column[1].to_string()+&",".to_string()+&column[2].to_string()+&",".to_string()+&column[3].to_string()+&")".to_string()}></div>
          });
          column_num += 1;
        }
        row_num += 1;
        column_num = 0;
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
