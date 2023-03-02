use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;

mod start;
use start::Start;
mod create_load;
use create_load::Create;
use create_load::Load;
mod pixels;
use pixels::{Pixels, PixelRedrawRange};
mod pixel_actions;
use pixel_actions::{PixelActions, PixelInfo};
mod image_actions;
use image_actions::ImageActions;
mod tools;
use tools::{Tools, ToolsTypes};

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
  ToolSelected(ToolsTypes),
  FillBucket([u8; 4]),
  ChangeToolColor([u8; 4]),
  Filter(String),
  DrawLine([[u16; 2]; 2]),
  DrawRect([[u16; 2]; 2]),
  DrawEllipse([[u16; 2]; 2]),
  Blur(u8),
}

pub struct App {
  current_bmp: Option<BMP>,
  selected_tool: ToolsTypes,
  tool_color: [u8; 4],
  show_create: bool,
  show_load: bool,
  show_pixel_info: bool,
  show_image_actions: bool,
  should_redraw: bool,
  only_redraw_coords: PixelRedrawRange,
  pixel_info: Option<PixelInfo>,
}

impl Component for App {
  type Message = AppMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { current_bmp: None, selected_tool: ToolsTypes::NoneSelected, tool_color: [255, 255, 255, 255], show_create: false, show_load: false, show_pixel_info: false, show_image_actions: false, should_redraw: true, only_redraw_coords: PixelRedrawRange::Empty, pixel_info: None }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    self.only_redraw_coords = PixelRedrawRange::Empty;
    let link = ctx.link().clone();
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
        self.show_image_actions = true;
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
        match self.selected_tool {
          ToolsTypes::ClickFill => {
            link.send_message(Self::Message::ChangeSelectedPixel(self.tool_color));
          },
          ToolsTypes::BucketFill => {
            link.send_message(Self::Message::FillBucket(self.tool_color));
          },
          _ => {
            //do nothing
          },
        }
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
        self.only_redraw_coords = PixelRedrawRange::Point(coord);
        true
      },
      Self::Message::FillBucket(color) => {
        //get selected pixel and fill paint bucket
        let coord = self.pixel_info.as_ref().unwrap().coords;
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.fill_bucket(color, coord[0] as usize, coord[1] as usize).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        true
      },
      Self::Message::ToolSelected(tool) => {
        self.selected_tool = tool;
        self.should_redraw = false;
        true
      },
      Self::Message::ChangeToolColor(color) => {
        self.tool_color = color;
        self.should_redraw = false;
        false
      },
      Self::Message::Filter(filter_type) => {
        if filter_type == "invert" {
          let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
          current_bmp.invert(None).unwrap();
          self.current_bmp = Some(current_bmp);
          self.should_redraw = true;
          true
        } else if filter_type == "grayscale" {
          let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
          current_bmp.grayscale().unwrap();
          self.current_bmp = Some(current_bmp);
          self.should_redraw = true;
          true
        } else {
          false
        }
      },
      Self::Message::DrawLine(endpoints) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.draw_line(self.tool_color, endpoints[0], endpoints[1]).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        self.only_redraw_coords = PixelRedrawRange::Rect(endpoints);
        true
      },
      Self::Message::DrawRect(endpoints) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.draw_rectangle(Some(self.tool_color), Some(self.tool_color), endpoints[0], endpoints[1]).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        self.only_redraw_coords = PixelRedrawRange::Rect(endpoints);
        true
      },
      Self::Message::DrawEllipse(ellipse_args) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.draw_ellipse(ellipse_args[0], ellipse_args[1][0], ellipse_args[1][1], self.tool_color, Some(self.tool_color), true).unwrap();
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        true
      },
      Self::Message::Blur(blur_radius) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        match self.selected_tool {
          ToolsTypes::Gaussian => {
            current_bmp.gaussian_blur(blur_radius).unwrap();
            self.current_bmp = Some(current_bmp);
            self.should_redraw = true;
            true
          },
          ToolsTypes::Box => {
            current_bmp.box_blur(blur_radius).unwrap();
            self.current_bmp = Some(current_bmp);
            self.should_redraw = true;
            true
          },
          _ => {
            //do nothing
            self.should_redraw = false;
            false
          },
        }
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    //let mut from_scratch = false;

    let create_load_callback = ctx.link().callback(|from_scratch: bool| {
      if from_scratch {
        Self::Message::Create
      } else {
        Self::Message::Load
      }
    });

    let send_bmp_callback = ctx.link().callback(|new_bmp: BMP| {
      Self::Message::NewBMP(new_bmp)
    });

    let send_pixel_click = ctx.link().callback(|coords: [u16; 2]| {
      Self::Message::PixelClicked(coords[0], coords[1])
    });

    //pixel change
    let change_pixel_callback = ctx.link().callback(|new_color: [u8; 4]| {
      Self::Message::ChangeSelectedPixel(new_color)
    });

    //tools
    let tool_change_callback = ctx.link().callback(|tool: ToolsTypes| {
      Self::Message::ToolSelected(tool)
    });

    let change_tool_color_callback = ctx.link().callback(|color: [u8; 4]| {
      Self::Message::ChangeToolColor(color)
    });

    let filter_callback = ctx.link().callback(|filter_type: String| {
      Self::Message::Filter(filter_type)
    });

    let rect_callback = ctx.link().callback(|endpoints: [[u16; 2]; 2]| {
      Self::Message::DrawRect(endpoints)
    });

    let line_callback = ctx.link().callback(|endpoints: [[u16; 2]; 2]| {
      Self::Message::DrawLine(endpoints)
    });

    let ellipse_callback = ctx.link().callback(|ellipse_args: [[u16; 2]; 2]| {
      Self::Message::DrawEllipse(ellipse_args)
    });

    let blur_callback = ctx.link().callback(|blur_radius: u8| {
      Self::Message::Blur(blur_radius)
    });

    let current_bmp = &self.to_owned().current_bmp;
  
    html! {
      <div id="main">
        <Start {create_load_callback} />
        <Create send_bmp_callback={send_bmp_callback.clone()} show={self.show_create} />
        <Load send_bmp_callback={send_bmp_callback} show={self.show_load} />
        <ImageActions current_bmp={current_bmp.clone()} show={self.show_image_actions} {tool_change_callback} />
        <Tools selected_tool={self.selected_tool} {change_tool_color_callback} {filter_callback} {line_callback} {rect_callback} {ellipse_callback} {blur_callback} tool_color={self.tool_color} show={self.show_image_actions} />
        <Pixels {send_pixel_click} current_bmp={current_bmp.clone()} should_redraw={self.should_redraw} only_redraw_coords={self.only_redraw_coords} />
        <PixelActions pixel_info={self.pixel_info.clone()} show={self.show_pixel_info} {change_pixel_callback} />
      </div>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
