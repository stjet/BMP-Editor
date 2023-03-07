use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::fmt;
//use gloo_console::log;

//gives instructions on how to use tool, and also provides the interface to actually use tool

#[derive(PartialEq, Clone, Copy)]
pub enum ToolsTypes {
  NoneSelected,
  ClickFill,
  BucketFill,
  Invert,
  Line,
  Rect,
  Ellipse,
  Greyscale,
  Gaussian,
  Box,
}

impl ToolsTypes {
  pub fn get_select_id(&self) -> &'static str {
    match self {
      ToolsTypes::NoneSelected => "none",
      ToolsTypes::ClickFill => "fills",
      ToolsTypes::BucketFill => "fills",
      ToolsTypes::Invert => "filters",
      ToolsTypes::Line => "shapes",
      ToolsTypes::Rect => "shapes",
      ToolsTypes::Ellipse => "shapes",
      ToolsTypes::Greyscale => "filters",
      ToolsTypes::Gaussian => "filters",
      ToolsTypes::Box => "filters",
    }
  }
  fn as_str(&self) -> &'static str {
    match self {
      ToolsTypes::NoneSelected => "none-selected",
      ToolsTypes::ClickFill => "click-fill",
      ToolsTypes::BucketFill => "bucket-fill",
      ToolsTypes::Invert => "invert",
      ToolsTypes::Line => "line",
      ToolsTypes::Rect => "rect",
      ToolsTypes::Ellipse => "ellipse",
      ToolsTypes::Greyscale => "World",
      ToolsTypes::Gaussian => "gaussian",
      ToolsTypes::Box => "box",
    }
  }
}

impl fmt::Display for ToolsTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.as_str())
  }
}

#[derive(PartialEq, Properties)]
pub struct ToolsProps {
  pub selected_tool: ToolsTypes,
  pub change_tool_color_callback: Callback<[u8; 4]>,
  pub filter_callback: Callback<String>,
  pub line_callback: Callback<[[u16; 2]; 2]>,
  pub rect_callback: Callback<[[u16; 2]; 2]>,
  pub ellipse_callback: Callback<[[u16; 2]; 2]>,
  pub blur_callback: Callback<u8>,
  pub tool_color: [u8; 4],
  pub show: bool,
}

pub enum ToolsMessage {
  Show,
  Hide,
  Filter(String),
  Line([[u16; 2]; 2]),
  Rect([[u16; 2]; 2]),
  Ellipse([[u16; 2]; 2]),
  ChangeToolColor([u8; 4]),
  Blur(u8),
  Grayscale,
}

pub struct Tools {
  display: String,
}

impl Component for Tools {
  type Message = ToolsMessage;
  type Properties = ToolsProps;

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
      Self::Message::Filter(filter_type) => {
        let _ = ctx.props().filter_callback.emit(filter_type);
        true
      },
      Self::Message::Line(endpoints) => {
        let _ = ctx.props().line_callback.emit(endpoints);
        true
      },
      Self::Message::Rect(endpoints) => {
        let _ = ctx.props().rect_callback.emit(endpoints);
        true
      },
      Self::Message::Ellipse(ellipse_args) => {
        let _ = ctx.props().ellipse_callback.emit(ellipse_args);
        true
      }
      Self::Message::ChangeToolColor(color) => {
        //run callback of parent
        let _ = ctx.props().change_tool_color_callback.emit(color);
        false
      },
      Self::Message::Blur(blur_radius) => {
        let _ = ctx.props().blur_callback.emit(blur_radius);
        false
      },
      Self::Message::Grayscale => {
        let _ = ctx.props().filter_callback.emit("grayscale".to_string());
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
    
    let mut selected_tool_name: String = "Selected Tool: ".to_string();
    let selected_tool_info: String;

    let mut color_picker_display: String = "none".to_string();
    let mut invert_button_display: String = "none".to_string();
    let mut end_points_display: String = "none".to_string();
    let mut ellipse_display: String = "none".to_string();
    let mut blur_display: String = "none".to_string();
    let mut grayscale_display: String = "none".to_string();

    let selected_tool = ctx.props().selected_tool;
    
    match selected_tool {
      ToolsTypes::ClickFill => {
        selected_tool_name += "Click Fill";
        selected_tool_info = "Click a pixel to change it's fill to the currently selected color.".to_string();
        color_picker_display = "block".to_string();
      },
      ToolsTypes::BucketFill => {
        selected_tool_name += "Bucket Fill";
        selected_tool_info = "Click a pixel to change all surrounding pixels with the same color to the currently selected color.".to_string();
        color_picker_display = "block".to_string();
      },
      ToolsTypes::Invert => {
        selected_tool_name += "Invert";
        selected_tool_info = "Click the button below to invert the image colors.".to_string();
        invert_button_display = "block".to_string();
      },
      ToolsTypes::Line => {
        selected_tool_name += "Line";
        selected_tool_info = "Specify endpoint coordinates and color to create a line.".to_string();
        end_points_display = "block".to_string();
        color_picker_display = "block".to_string();
      },
      ToolsTypes::Rect => {
        selected_tool_name += "Rect";
        selected_tool_info = "Specify endpoint coordinates and color to create a rectangle.".to_string();
        end_points_display = "block".to_string();
        color_picker_display = "block".to_string();
      },
      ToolsTypes::Ellipse => {
        selected_tool_name += "Ellipse";
        selected_tool_info = "Specify coordinates, lengths, colors, and create a ellipse.".to_string();
        ellipse_display = "block".to_string();
        color_picker_display = "block".to_string();
      },
      ToolsTypes::Greyscale => {
        selected_tool_name += "Greyscale";
        selected_tool_info = "Choose one channel (or all of them) to do a greyscale filter on.".to_string();
        grayscale_display = "block".to_string();
      },
      ToolsTypes::Gaussian => {
        selected_tool_name += "Gaussian Blur";
        selected_tool_info = "Specify blur radius and do a blur.".to_string();
        blur_display = "block".to_string();
      },
      ToolsTypes::Box => {
        selected_tool_name += "Box Blur";
        selected_tool_info = "Specify blur radius and do a blur.".to_string();
        blur_display = "block".to_string();
      },
      ToolsTypes::NoneSelected => {
        selected_tool_name += "None Selected";
        selected_tool_info = "Use the 'Tools' dropdown at the top to select a tool.".to_string();
      },
    }

    let tc_input_ref = NodeRef::default();

    let first_endpoint_ref = NodeRef::default();
    let second_endpoint_ref = NodeRef::default();

    let center_input_ref = NodeRef::default();
    let xlength_input_ref = NodeRef::default();
    let ylength_input_ref = NodeRef::default();

    let blur_radius_ref = NodeRef::default();

    let tool_color = ctx.props().tool_color;

    let color_text = format!("({}, {}, {}, {})", tool_color[0], tool_color[1], tool_color[2], tool_color[3]);

    let new_tool_color = {
      let tc_input_ref2 = tc_input_ref.clone();
      ctx.link().callback(move |_| {
        let tc_input: HtmlInputElement = tc_input_ref2.clone().cast().unwrap();
        let new_color_vec: Vec<u8> = tc_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u8>().unwrap()).collect();
        let new_color: [u8; 4] = [new_color_vec[0], new_color_vec[1], new_color_vec[2], new_color_vec[3]];
        Self::Message::ChangeToolColor(new_color)
      })
    };

    let invert = ctx.link().callback(|_| Self::Message::Filter("invert".to_string()));

    let create = {
      let first_endpoint_ref2 = first_endpoint_ref.clone();
      let second_endpoint_ref2 = second_endpoint_ref.clone();
      ctx.link().callback(move |_| {
        let first_endpoint_input: HtmlInputElement = first_endpoint_ref2.cast().unwrap();
        let first_endpoint_vec: Vec<u16> = first_endpoint_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u16>().unwrap()).collect();
        let first_endpoint: [u16; 2] = [first_endpoint_vec[0], first_endpoint_vec[1]];
        let second_endpoint_input: HtmlInputElement = second_endpoint_ref2.cast().unwrap();
        let second_endpoint_vec: Vec<u16> = second_endpoint_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u16>().unwrap()).collect();
        let second_endpoint: [u16; 2] = [second_endpoint_vec[0], second_endpoint_vec[1]];
        let endpoints: [[u16; 2]; 2] = [first_endpoint, second_endpoint];
        if let ToolsTypes::Line = selected_tool {
          Self::Message::Line(endpoints)
        } else {
          //"else if let ToolsTypes::Rect = selected_tool" would be better but would not match all arms
          Self::Message::Rect(endpoints)
        }
      })
    };

    //ellipse display needs center, xlength, ylength
    let ellipse = {
      let center_input_ref2 = center_input_ref.clone();
      let xlength_input_ref2 = xlength_input_ref.clone();
      let ylength_input_ref2 = ylength_input_ref.clone();
      ctx.link().callback(move |_| {
        let center_input: HtmlInputElement = center_input_ref2.cast().unwrap();
        let center_input_vec: Vec<u16> = center_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u16>().unwrap()).collect();
        let xlength_input: HtmlInputElement = xlength_input_ref2.cast().unwrap();
        let xlength: u16 = xlength_input.value().parse().unwrap();
        let ylength_input: HtmlInputElement = ylength_input_ref2.cast().unwrap();
        let ylength: u16 = ylength_input.value().parse().unwrap();
        let ellipse_args: [[u16; 2]; 2] = [[center_input_vec[0], center_input_vec[1]], [xlength, ylength]];
        Self::Message::Ellipse(ellipse_args)
      })
    };

    //blur only needs blur radius
    let blur = {
      let blur_radius_ref2 = blur_radius_ref.clone();
      ctx.link().callback(move |_| {
        let blur_radius_input: HtmlInputElement = blur_radius_ref2.cast().unwrap();
        let blur_radius: u8 = blur_radius_input.value().parse().unwrap();
        Self::Message::Blur(blur_radius)
      })
    };

    let grayscale = {
      ctx.link().callback(move |_| {
        Self::Message::Grayscale
      })
    };
  
    html! {
      <div id={"tools"} style={"display: ".to_string()+&self.display}>
        <div>
          <h2>{ selected_tool_name }</h2>
          <p>{ selected_tool_info }</p>
          <div style={"display: ".to_string()+&color_picker_display}>
            <label for="tool-color">{"Color: "}</label>
            <input name="tool-color" value={color_text} ref={tc_input_ref}/>
            <button onclick={new_tool_color}>{ "Change" }</button>
          </div>
          <div style={"display: ".to_string()+&ellipse_display}>
            <label for="center">{"Center: "}</label>
            <input name="center" placeholder="(0, 0)" ref={&center_input_ref}/>
            <br/>
            <label for="x-length">{"X Length: "}</label>
            <input name="x-length" ref={&xlength_input_ref}/>
            <br/>
            <label for="y-length">{"Y Length: "}</label>
            <input name="y-length" ref={&ylength_input_ref}/>
            <br/>
            <button onclick={ellipse}>{ "Create" }</button>
          </div>
          <div style={"display: ".to_string()+&end_points_display}>
            <label for="first-endpoint">{"First Endpoint: "}</label>
            <input name="first-endpoint" placeholder="(0, 0)" ref={first_endpoint_ref}/>
            <br/>
            <label for="second-endpoint">{"Second Endpoint: "}</label>
            <input name="second-endpoint" placeholder="(0, 0)" ref={second_endpoint_ref}/>
            <br/>
            <button onclick={create}>{ "Create" }</button>
          </div>
          <div style={"display: ".to_string()+&invert_button_display}>
            <button onclick={invert}>{ "Invert" }</button>
          </div>
          <div style={"display: ".to_string()+&blur_display}>
            <label for="blur-radius">{"Blur Radius: "}</label>
            <input type="number" name="blur-radius" value="3" ref={blur_radius_ref}/>
            <br/>
            <button onclick={blur}>{ "Blur" }</button>
          </div>
          <div style={"display: ".to_string()+&grayscale_display}>
            <button onclick={grayscale}>{ "Grayscale Filter" }</button>
          </div>
        </div>
      </div>
    }
  }
}
