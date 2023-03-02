use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Path2d};

#[derive(PartialEq, Clone, Copy)]
pub enum PixelRedrawRange {
  Point([u16; 2]),
  Rect([[u16; 2]; 2]),
  Empty,
}

impl PixelRedrawRange {
  fn is_empty(&self) -> bool {
    match self {
      Self::Empty => {
        true
      },
      _ => {
        false
      }
    }
  }
  fn is_not_empty(&self) -> bool {
    !self.is_empty()
  }
}

#[derive(PartialEq, Properties)]
pub struct PixelsProps {
  pub current_bmp: Option<BMP>,
  pub send_pixel_click: Callback<[u16; 2]>,
  pub should_redraw: bool,
  pub only_redraw_coords: PixelRedrawRange,
}

pub enum PixelsMessage {
  Show,
  SendClick([u16; 2]),
  PostRenderUpdate,
}

pub struct Pixels {
  display: String,
  canvas_ref: NodeRef,
  canvas_ref_top: NodeRef,
  should_update: bool,
}

impl Component for Pixels {
  type Message = PixelsMessage;
  type Properties = PixelsProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string(), canvas_ref: NodeRef::default(), canvas_ref_top: NodeRef::default(), should_update: true }
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

    if ctx.props().current_bmp.is_some() {
      if self.display == "none".to_string() {
        link.send_message(Self::Message::Show);
      }

      //let pixels_all: Vec<Path2d> = self.pixels_all.clone();

      let current_bmp = ctx.props().current_bmp.clone().unwrap();
      let canvas_ref_top = self.canvas_ref_top.clone();
      let canvas_ref_top2 = self.canvas_ref_top.clone();

      let pixel_mousemove_callback = Callback::from(move |e: MouseEvent| {
        let canvas: Option<HtmlCanvasElement> = canvas_ref_top.cast();
        if canvas.is_some() {
          let dib_header = &current_bmp.get_dib_header().unwrap();
          let width = dib_header.width;
          let height = dib_header.height.abs() as u32;
          let pixel_wh;
          if height > width {
            pixel_wh = (650 as f64/height as f64).floor() as i32;
          } else {
            pixel_wh = (650 as f64/width as f64).floor() as i32;
          }
          let x = (e.offset_x() as f64/pixel_wh as f64).floor() as i32;
          let y = (e.offset_y() as f64/pixel_wh as f64).floor() as i32;
          if x as u32 >= width || y as u32 >= height {
            return;
          }
          let canvas: HtmlCanvasElement = canvas.unwrap();
          let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
          context.clear_rect(0 as f64, 0 as f64, 650 as f64, 650 as f64);
          let pixel_path = Path2d::new().unwrap();
          let top_left = [pixel_wh*x, pixel_wh*y];
          let bottom_right = [pixel_wh*(x+1), pixel_wh*(y+1)];
          pixel_path.move_to(top_left[0] as f64, top_left[1] as f64);
          pixel_path.line_to(bottom_right[0] as f64, top_left[1] as f64);
          pixel_path.line_to(bottom_right[0] as f64, bottom_right[1] as f64);
          pixel_path.line_to(top_left[0] as f64, bottom_right[1] as f64);
          pixel_path.line_to(top_left[0] as f64, top_left[1] as f64);
          context.set_fill_style(&JsValue::from("rgba(255, 255, 230, 0.5)".to_string()));
          context.fill_with_path_2d(&pixel_path);
        }
      });

      let pixel_mousemove = {
        pixel_mousemove_callback.clone()
      };

      let pixel_mouseout_callback = Callback::from(move |_e: MouseEvent| {
        let canvas: Option<HtmlCanvasElement> = canvas_ref_top2.cast();
        if canvas.is_some() {
          let canvas: HtmlCanvasElement = canvas.unwrap();
          let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
          context.clear_rect(0 as f64, 0 as f64, 650 as f64, 650 as f64);
        }
      });

      let pixel_mouseout = {
        pixel_mouseout_callback.clone()
      };

      let pixel_click = {
        let current_bmp2 = ctx.props().current_bmp.clone().unwrap();
        ctx.link().batch_callback(move |e: MouseEvent| {
          let dib_header = &current_bmp2.get_dib_header().unwrap();
          let width = dib_header.width;
          let height = dib_header.height.abs() as u32;
          let pixel_wh;
          if height > width {
            pixel_wh = (650 as f64/height as f64).floor() as i32;
          } else {
            pixel_wh = (650 as f64/width as f64).floor() as i32;
          }
          let x = (e.offset_x() as f64/pixel_wh as f64).floor() as u16;
          let y = (e.offset_y() as f64/pixel_wh as f64).floor() as u16;
          if x as u32 >= width || y as u32 >= height {
            return None;
          }
          return Some(Self::Message::SendClick([x, y]));
        })
      };

      html! {
        <div style={"display: ".to_string()+&self.display}>
          <canvas id="pixels" width="650" height="650" ref={self.canvas_ref.clone()}></canvas>
          <br/>
          <canvas id="top-pixels" width="650" height="650" ref={self.canvas_ref_top.clone()} onclick={pixel_click} onmousemove={pixel_mousemove} onmouseout={pixel_mouseout}></canvas>
        </div>
      }
    } else {
      html! {
        <div id="pixel-grid" style={"display: ".to_string()+&self.display}></div>
      }
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    log!("Rendering");
    let canvas: Option<HtmlCanvasElement> = self.canvas_ref.cast();
    if canvas.is_some() && ctx.props().should_redraw {
      log!("Redrawing");
      let canvas: HtmlCanvasElement = canvas.unwrap();
      let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
      //clear canvas
      let unwrapped_bmp = ctx.props().current_bmp.as_ref().unwrap();
      let dib_header = unwrapped_bmp.get_dib_header().unwrap();
      let width = dib_header.width;
      let height = dib_header.height.abs() as u32;
      let pixel_wh;
      if height > width {
        pixel_wh = (650 as f64/height as f64).floor() as u32;
      } else {
        pixel_wh = (650 as f64/width as f64).floor() as u32;
      }
      let pixel_data = unwrapped_bmp.get_pixel_data().unwrap();
      let only_redraw_coords = ctx.props().only_redraw_coords;
      if only_redraw_coords.is_empty() {
        context.clear_rect(0 as f64, 0 as f64, 650 as f64, 650 as f64);
        for y in 0..height {
          for x in 0..width {
            //ctx.props().only_redraw_coords.
            let top_left = [pixel_wh*x, pixel_wh*y];
            /*let pixel_path = Path2d::new().unwrap();
            let bottom_right = [pixel_wh*(x+1), pixel_wh*(y+1)];
            pixel_path.move_to(top_left[0] as f64, top_left[1] as f64);
            pixel_path.line_to(bottom_right[0] as f64, top_left[1] as f64);
            pixel_path.line_to(bottom_right[0] as f64, bottom_right[1] as f64);
            pixel_path.line_to(top_left[0] as f64, bottom_right[1] as f64);
            pixel_path.line_to(top_left[0] as f64, top_left[1] as f64);*/
            let color = unwrapped_bmp.get_color_of_px_efficient(x as usize, y as usize, &dib_header, &pixel_data).unwrap();
            context.set_fill_style(&JsValue::from(format!("rgba({},{},{},{})", color[0], color[1], color[2], (color[3] as f64/255 as f64))));
            context.fill_rect(top_left[0].into(), top_left[1].into(), pixel_wh.into(), pixel_wh.into());
            //context.fill_with_path_2d(&pixel_path);
          }
        }
      } else {
        if let PixelRedrawRange::Rect(rect_coords) = only_redraw_coords {
          log!("redrawing only rect portion");
          //only change the pixels of the rectangular area
          for y_add in 0..(rect_coords[1][1]-rect_coords[0][1]+1) {
            for x_add in 0..(rect_coords[1][0]-rect_coords[0][0]+1) {
              let coord: [u16; 2] = [rect_coords[0][0]+x_add, rect_coords[0][1]+y_add];
              let top_left = [pixel_wh*coord[0] as u32, pixel_wh*coord[1] as u32];
              let color = unwrapped_bmp.get_color_of_px_efficient(coord[0] as usize, coord[1] as usize, &dib_header, &pixel_data).unwrap();
              context.set_fill_style(&JsValue::from(format!("rgba({},{},{},{})", color[0], color[1], color[2], (color[3] as f64/255 as f64))));
              context.fill_rect(top_left[0].into(), top_left[1].into(), pixel_wh.into(), pixel_wh.into());
            }
          }
        } else if let PixelRedrawRange::Point(coord) = only_redraw_coords {
          log!("redrawing only point portion");
          let top_left = [pixel_wh*coord[0] as u32, pixel_wh*coord[1] as u32];
          let color = unwrapped_bmp.get_color_of_px_efficient(coord[0] as usize, coord[1] as usize, &dib_header, &pixel_data).unwrap();
          context.set_fill_style(&JsValue::from(format!("rgba({},{},{},{})", color[0], color[1], color[2], (color[3] as f64/255 as f64))));
          context.fill_rect(top_left[0].into(), top_left[1].into(), pixel_wh.into(), pixel_wh.into());
        }
      }
      //we can't use first render, have to set our own attribute
      //force an update so self.pixels_all is accurate
      if self.should_update {
        let link = ctx.link();
        link.send_message(Self::Message::PostRenderUpdate);
      }
    }
  }
}
