use yew::prelude::*;
use web_sys::HtmlInputElement;
//use gloo_console::log;

//gives instructions on how to use tool, and also provides the interface to actually use tool

#[derive(PartialEq, Clone, Copy)]
pub enum ToolsTypes {
  NoneSelected,
  ClickFill,
  BucketFill,
  Invert,
}

#[derive(PartialEq, Properties)]
pub struct ToolsProps {
  pub selected_tool: ToolsTypes,
  pub change_tool_color_callback: Callback<[u8; 4]>,
  pub filter_callback: Callback<String>,
  pub tool_color: [u8; 4],
  pub show: bool,
}

pub enum ToolsMessage {
  Show,
  Hide,
  Filter(String),
  ChangeToolColor([u8; 4]),
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
      Self::Message::ChangeToolColor(color) => {
        //run callback of parent
        let _ = ctx.props().change_tool_color_callback.emit(color);
        false
      },
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();
    let link3 = ctx.link().clone();

    if self.display == "none".to_string() && ctx.props().show {
      link.send_message(Self::Message::Show);
    } else if self.display == "block".to_string() && !ctx.props().show {
      link.send_message(Self::Message::Hide);
    }

    let tc_input_ref = NodeRef::default();
    let tc_input_ref2 = tc_input_ref.clone();
    
    let mut selected_tool_name: String = "Selected Tool: ".to_string();
    let selected_tool_info: String;

    let mut color_picker_display: String = "none".to_string();
    let mut invert_button_display: String = "none".to_string();
    
    match ctx.props().selected_tool {
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
      ToolsTypes::NoneSelected => {
        selected_tool_name += "None Selected";
        selected_tool_info = "Use the 'Tools' dropdown at the top to select a tool.".to_string();
      },
    }

    let tool_color = ctx.props().tool_color;

    let color_text = format!("({}, {}, {}, {})", tool_color[0], tool_color[1], tool_color[2], tool_color[3]);

    let new_tool_color_callback = Callback::from(move |_| {
      //get new pixel color
      let tc_input: HtmlInputElement = tc_input_ref2.clone().cast().unwrap();
      let new_color_vec: Vec<u8> = tc_input.value().replace("(", "").replace(")", "").split(", ").map(|value| value.parse::<u8>().unwrap()).collect();
      let new_color: [u8; 4] = [new_color_vec[0], new_color_vec[1], new_color_vec[2], new_color_vec[3]];
      link2.send_message(Self::Message::ChangeToolColor(new_color));
    });

    let new_tool_color = {
      new_tool_color_callback.clone()
    };

    let invert_callback = Callback::from(move |_| {
      link3.send_message(Self::Message::Filter("invert".to_string()));
    });

    let invert = {
      invert_callback.clone()
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
          <div style={"display: ".to_string()+&invert_button_display}>
            <button onclick={invert}>{ "Invert" }</button>
          </div>
        </div>
      </div>
    }
  }
}
