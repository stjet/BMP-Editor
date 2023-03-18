use yew::prelude::*;
use bmp_rust::bmp::{BMP, ImageDiff};
use gloo_console::log;
use std::collections::HashMap;

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
use image_actions::{ImageActions, KeybindActions};
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
  Undo,
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
  last_diff: Vec<ImageDiff>,
  keybinds: HashMap<String, KeybindActions>,
}

impl Component for App {
  type Message = AppMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    let keybinds: HashMap<String, KeybindActions> = HashMap::from([
      ("ctrl+z".to_string(), KeybindActions::Undo),
      ("[".to_string(), KeybindActions::PreviousTool),
      ("]".to_string(), KeybindActions::NextTool),
      ("c".to_string(), KeybindActions::ToolChange(ToolsTypes::ClickFill)),
      ("b".to_string(), KeybindActions::ToolChange(ToolsTypes::BucketFill)),
      ("i".to_string(), KeybindActions::ToolChange(ToolsTypes::Invert)),
      ("l".to_string(), KeybindActions::ToolChange(ToolsTypes::Line)),
      ("r".to_string(), KeybindActions::ToolChange(ToolsTypes::Rect)),
      ("e".to_string(), KeybindActions::ToolChange(ToolsTypes::Ellipse)),
      ("g".to_string(), KeybindActions::ToolChange(ToolsTypes::Greyscale)),
      ("a".to_string(), KeybindActions::ToolChange(ToolsTypes::Gaussian)),
      ("o".to_string(), KeybindActions::ToolChange(ToolsTypes::Box)),
    ]);
    Self { current_bmp: None, selected_tool: ToolsTypes::NoneSelected, tool_color: [255, 255, 255, 255], show_create: false, show_load: false, show_pixel_info: false, show_image_actions: false, should_redraw: true, only_redraw_coords: PixelRedrawRange::Empty, pixel_info: None, last_diff: Vec::new(), keybinds }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    self.only_redraw_coords = PixelRedrawRange::Empty;
    //don't store more than last 10 last diffs
    if self.last_diff.len() == 10 {
      self.last_diff.remove(0);
    }
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
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
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
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
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
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
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
          self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
          self.current_bmp = Some(current_bmp);
          self.should_redraw = true;
          true
        } else if filter_type == "greyscale" {
          let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
          current_bmp.greyscale().unwrap();
          self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
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
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        self.only_redraw_coords = PixelRedrawRange::Rect(endpoints);
        true
      },
      Self::Message::DrawRect(endpoints) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.draw_rectangle(Some(self.tool_color), Some(self.tool_color), endpoints[0], endpoints[1]).unwrap();
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        self.only_redraw_coords = PixelRedrawRange::Rect(endpoints);
        true
      },
      Self::Message::DrawEllipse(ellipse_args) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        current_bmp.draw_ellipse(ellipse_args[0], ellipse_args[1][0], ellipse_args[1][1], self.tool_color, Some(self.tool_color), true).unwrap();
        self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
        self.current_bmp = Some(current_bmp);
        self.should_redraw = true;
        true
      },
      Self::Message::Blur(blur_radius) => {
        let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
        match self.selected_tool {
          ToolsTypes::Gaussian => {
            current_bmp.gaussian_blur(blur_radius).unwrap();
            self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
            self.current_bmp = Some(current_bmp);
            self.should_redraw = true;
            true
          },
          ToolsTypes::Box => {
            current_bmp.box_blur(blur_radius).unwrap();
            self.last_diff.push(BMP::diff(&self.current_bmp.as_ref().unwrap(), &current_bmp).unwrap());
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
      },
      Self::Message::Undo => {
        if self.last_diff.len() > 0 {
          let mut current_bmp = self.current_bmp.as_ref().unwrap().clone();
          let last_diff: &ImageDiff = &self.last_diff.last().unwrap();
          let dib_header = current_bmp.get_dib_header().unwrap();
          let file_header = current_bmp.get_header();
          //write diff to bmp, specifically the first color
          if last_diff.is_same_size() {
            //write the difference
            for diff_num in 0..last_diff.diff.len() {
              let diff = &last_diff[diff_num];
              let diff_coord = diff.coord;
              let diff_color = diff.color1.unwrap();
              current_bmp.change_color_of_pixel_efficient(diff_coord[0], diff_coord[1], diff_color, &dib_header, &file_header).unwrap();
            }
            self.last_diff.pop();
            self.current_bmp = Some(current_bmp);
            self.should_redraw = true;
          } else {
            //todo: make it work for images of different sizes (extend the image or shrink it and write the diffs)
          }
          true
        } else {
          false
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

    let undo_callback = ctx.link().callback(|_: bool| {
      Self::Message::Undo
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
        <ImageActions selected_tool={self.selected_tool} current_bmp={current_bmp.clone()} show={self.show_image_actions} {tool_change_callback} {undo_callback} keybinds={self.keybinds.clone()} />
        <Tools selected_tool={self.selected_tool} {change_tool_color_callback} {filter_callback} {line_callback} {rect_callback} {ellipse_callback} {blur_callback} tool_color={self.tool_color} show={self.show_image_actions} keybinds={self.keybinds.clone()} />
        <Pixels {send_pixel_click} current_bmp={current_bmp.clone()} should_redraw={self.should_redraw} only_redraw_coords={self.only_redraw_coords} />
        <PixelActions pixel_info={self.pixel_info.clone()} show={self.show_pixel_info} {change_pixel_callback} />
        <div id={"bottom-links"}>
          <a href="https://github.com/jetstream0/BMP-Editor" target="_blank">{ "Editor Github" }</a>
          <span class="link-divider">{ "-" }</span>
          <a href="https://github.com/jetstream0/BMP-Rust" target="_blank">{ "Library Github" }</a>
          <span class="link-divider">{ "-" }</span>
          <a href="https://www.prussiafan.club/articles/bmp-deep-dive" target="_blank">{ "BMP Format" }</a>
        </div>
      </div>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
