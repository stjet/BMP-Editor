use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;
use wasm_bindgen::JsValue;

mod start;
use start::Start;
mod create_load;
use create_load::Create;
use create_load::Load;
mod pixels;
use pixels::Pixels;
mod pixel_actions;
use pixel_actions::{PixelActions, PixelInfo};

#[derive(PartialEq, Properties, Default)]
pub struct Props;

//App

pub enum AppMessage {
  Create,
  Load,
  NewBMP(BMP),
  PixelClicked(u16, u16),
  ChangePixels(Vec<[u16; 2]>, [u8; 4]),
  ChangeSelectedPixel([u8; 4]),
}

pub struct App {
  current_bmp: Option<BMP>,
  show_create: bool,
  show_load: bool,
  show_pixel_info: bool,
  should_redraw: bool,
  pixel_info: Option<PixelInfo>,
}

impl Component for App {
  type Message = AppMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { current_bmp: None, show_create: false, show_load: false, show_pixel_info: false, should_redraw: true, pixel_info: None }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Create => {
        //Create
        self.show_create = true;
        true
      },
      Self::Message::Load => {
        self.show_load = true;
        true
      },
      Self::Message::NewBMP(bmp_inside) => {
        log!("new bmp");
        self.show_create = false;
        self.show_load = false;
        self.current_bmp = Some(bmp_inside);
        true
      },
      Self::Message::PixelClicked(x, y) => {
        self.show_pixel_info = true;
        let pixel_color = self.current_bmp.as_ref().unwrap().clone().get_color_of_px(x as usize, y as usize).unwrap();
        self.pixel_info = Some(PixelInfo {
          color: pixel_color,
          coords: [x, y],
        });
        //can safely update entire, without having to worry about pixel canvas being redrawn
        self.should_redraw = false;
        true
      },
      Self::Message::ChangePixels(pixels, color) => {
        //iterate through pixels and change them
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.change_color_of_pixels(pixels, color).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        true
      },
      //vec![self.pixel_info.unwrap().coords], 
      Self::Message::ChangeSelectedPixel(color) => {
        //get selected pixel and change the color of it
        let coord = self.pixel_info.as_ref().unwrap().coords;
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.change_color_of_pixel(coord[0], coord[1], color).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        true
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();
    let link3 = ctx.link().clone();
    let link4 = ctx.link().clone();
    //let mut from_scratch = false;

    let create_load_process = move |from_scratch: bool| {
      if from_scratch {
        link.send_message(Self::Message::Create);
      } else {
        link.send_message(Self::Message::Load);
      }
    };
  
    let create_load_callback = Callback::from(move |from_scratch_passed: bool| {
      create_load_process(from_scratch_passed);
    });

    let send_bmp_process = move |new_bmp: BMP| {
      log!("yes", new_bmp.contents.len());
      link2.send_message(Self::Message::NewBMP(new_bmp))
      //new_bmp so emit NewBMP message
    };
  
    let send_bmp_callback = Callback::from(move |new_bmp: BMP| {
      send_bmp_process(new_bmp);
    });
    let send_bmp_callback2 = send_bmp_callback.clone();

    let pixel_click_process = move |x: u16, y: u16| {
      link3.send_message(Self::Message::PixelClicked(x, y))
    };
  
    let send_pixel_click = Callback::from(move |coords: [u16; 2]| {
      pixel_click_process(coords[0], coords[1]);
    });

    //pixel change
    let change_pixel_process = move |new_color: [u8; 4]| {
      link4.send_message(Self::Message::ChangeSelectedPixel(new_color));
    };

    let change_pixel_callback = Callback::from(move |new_color: [u8; 4]| {
      change_pixel_process(new_color);
    });

    let current_bmp = &self.to_owned().current_bmp;
  
    html! {
      <div id="main">
        <Start {create_load_callback} />
        <Create {send_bmp_callback} show={self.show_create} />
        <Load send_bmp_callback={send_bmp_callback2} show={self.show_load} />
        <Pixels {send_pixel_click} current_bmp={current_bmp.clone()} should_redraw={self.should_redraw} />
        <PixelActions pixel_info={self.pixel_info.clone()} show={self.show_pixel_info} {change_pixel_callback} />
      </div>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
