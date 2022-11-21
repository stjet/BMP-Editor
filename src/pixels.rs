use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

// create

#[derive(PartialEq, Properties)]
pub struct PixelsProps {
  pub current_bmp: Option<BMP>,
  pub send_pixel_click: Callback<[u16; 2]>,
}

pub enum PixelsMessage {
  Show,
  SendClick([u16; 2]),
  PostRenderUpdate,
}

pub struct Pixels {
  display: String,
  canvas_ref: NodeRef,
  pixels_all: Vec<Path2d>,
  should_update: bool,
}

impl Component for Pixels {
  type Message = PixelsMessage;
  type Properties = PixelsProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string(), canvas_ref: NodeRef::default(), pixels_all: vec![], should_update: true }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::SendClick(coords) => {
        let _ = ctx.props().send_pixel_click.emit(coords);
        false
      },
      Self::Message::PostRenderUpdate => {
        self.should_update = false;
        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();

    if ctx.props().current_bmp.is_some() {
      if self.display == "none".to_string() {
        link.send_message(Self::Message::Show);
      }

      let pixels_all: Vec<Path2d> = self.pixels_all.clone();

      let canvas_ref = self.canvas_ref.clone();
      let current_bmp = ctx.props().current_bmp.clone();
      
      let pixel_click_callback = Callback::from(move |e: MouseEvent| {
        let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        let unwrapped_bmp = current_bmp.as_ref().unwrap();
        let dib_header = unwrapped_bmp.get_dib_header().unwrap();
        let width = dib_header.width;
        let mut column_num: u16 = 0;
        let mut row_num: u16 = 0;
        //todo: improve efficiency
        for pixel_path in &pixels_all {
          let in_path: bool = context.is_point_in_path_with_path_2d_and_f64(pixel_path, e.offset_x() as f64, e.offset_y() as f64);
          if in_path {
            link2.send_message(Self::Message::SendClick([column_num, row_num]));
            break;
          }
          column_num = column_num + 1;
          if column_num == width as u16 {
            column_num = 0;
            row_num = row_num + 1;
          }
        }
      });
  
      let pixel_click = {
        pixel_click_callback.clone()
      };
      html! {
        <div style={"display: ".to_string()+&self.display}>
          <canvas id="pixels" width="650" height="650" ref={self.canvas_ref.clone()} onclick={pixel_click}></canvas>
        </div>
      }
    } else {
      html! {
        <div id="pixel-grid" style={"display: ".to_string()+&self.display}></div>
      }
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    let canvas: Option<HtmlCanvasElement> = self.canvas_ref.cast();
    if canvas.is_some() {
      let canvas: HtmlCanvasElement = canvas.unwrap();
      let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
      let unwrapped_bmp = ctx.props().current_bmp.as_ref().unwrap();
      let dib_header = unwrapped_bmp.get_dib_header();
      let width;
      let height;
      if let Ok(unwrapped_dib_header) = dib_header {
        width = unwrapped_dib_header.width;
        height = unwrapped_dib_header.height.abs() as u32;
      } else {
        width = 1;
        height = 1;
      }
      let pixel_wh;
      if height > width {
        pixel_wh = (650 as f64/height as f64).floor() as u16;
      } else {
        pixel_wh = (650 as f64/width as f64).floor() as u16;
      }
      let bmp_pixels = unwrapped_bmp.get_pixel_data().unwrap();
      let mut pixels_all: Vec<Path2d> = vec![];
      let mut column_num: u16 = 0;
      let mut row_num: u16 = 0;
      for row in bmp_pixels {
        for column in row {
          //path2d, draw pixel square, store path2d in array
          let pixel_path = Path2d::new().unwrap();
          let top_left = [pixel_wh*column_num, pixel_wh*row_num];
          let bottom_right = [pixel_wh*(column_num+1), pixel_wh*(row_num+1)];
          pixel_path.move_to(top_left[0] as f64, top_left[1] as f64);
          pixel_path.line_to(bottom_right[0] as f64, top_left[1] as f64);
          pixel_path.line_to(bottom_right[0] as f64, bottom_right[1] as f64);
          pixel_path.line_to(top_left[0] as f64, bottom_right[1] as f64);
          pixel_path.line_to(top_left[0] as f64, top_left[1] as f64);
          pixels_all.push(pixel_path.clone());
          if column.len() == 3 {
            //rgb
            context.set_fill_style(&JsValue::from(format!("rgb({},{},{})", column[0], column[1], column[2])));
          } else {
            //rgba
            context.set_fill_style(&JsValue::from(format!("rgba({},{},{},{})", column[0], column[1], column[2], column[3])));
          }
          context.fill_with_path_2d(&pixel_path);
          column_num += 1;
        }
        row_num += 1;
        column_num = 0;
      }
      self.pixels_all = pixels_all;
      //we can't use first render, have to set our own attribute
      //force an update so self.pixels_all is accurate
      if self.should_update {
        let link = ctx.link();
        link.send_message(Self::Message::PostRenderUpdate);
      }
    }
  }
}
