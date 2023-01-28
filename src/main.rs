use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;

mod start;
use start::Start;
mod create_load;
use create_load::Create;
use create_load::Load;
mod pixels;
use pixels::Pixels;
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
  pixel_info: Option<PixelInfo>,
}

impl Component for App {
  type Message = AppMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { current_bmp: None, selected_tool: ToolsTypes::NoneSelected, tool_color: [255, 255, 255, 255], show_create: false, show_load: false, show_pixel_info: false, show_image_actions: false, should_redraw: true, pixel_info: None }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        } else {
          false
        }
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();
    let link3 = ctx.link().clone();
    let link4 = ctx.link().clone();
    let link5 = ctx.link().clone();
    let link6 = ctx.link().clone();
    let link7 = ctx.link().clone();
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

    //tools
    let tool_change_process = move |tool: ToolsTypes| {
      link5.send_message(Self::Message::ToolSelected(tool));
    };

    let tool_change_callback = Callback::from(move |tool: ToolsTypes| {
      tool_change_process(tool);
    });

    let change_tool_color_process = move |color: [u8; 4]| {
      link6.send_message(Self::Message::ChangeToolColor(color));
    };

    let change_tool_color_callback = Callback::from(move |color: [u8; 4]| {
      change_tool_color_process(color);
    });

    let filter_process = move |filter_type: String| {
      link7.send_message(Self::Message::Filter(filter_type));
    };

    let filter_callback = Callback::from(move |filter_type: String| {
      filter_process(filter_type);
    });

    let current_bmp = &self.to_owned().current_bmp;
  
    html! {
      <div id="main">
        <Start {create_load_callback} />
        <Create {send_bmp_callback} show={self.show_create} />
        <Load send_bmp_callback={send_bmp_callback2} show={self.show_load} />
        <ImageActions current_bmp={current_bmp.clone()} show={self.show_image_actions} {tool_change_callback} />
        <Tools selected_tool={self.selected_tool} {change_tool_color_callback} {filter_callback} tool_color={self.tool_color} show={self.show_image_actions} />
        <Pixels {send_pixel_click} current_bmp={current_bmp.clone()} should_redraw={self.should_redraw} />
        <PixelActions pixel_info={self.pixel_info.clone()} show={self.show_pixel_info} {change_pixel_callback} />
      </div>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
