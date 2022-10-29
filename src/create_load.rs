use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_utils::document;
use gloo_console::log;
use web_sys::HtmlInputElement;

// create

#[derive(PartialEq, Properties)]
pub struct CreateProps {
  pub send_bmp_callback: Callback<BMP>,
  pub show: bool,
}

pub enum CreateMessage {
  Show,
  Hide,
  CreateBMP(i32, u32),
}

pub struct Create {
  display: String,
}

impl Component for Create {
  type Message = CreateMessage;
  type Properties = CreateProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string() }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Hide => {
        log!("Received");
        self.display = "none".to_string();
        true
      },
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::CreateBMP(height, width) => {
        self.display = "none".to_string();
        let _ = ctx.props().send_bmp_callback.emit(BMP::new(height, width));
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();

    if ctx.props().show && self.display == "none".to_string() {
      link.send_message(Self::Message::Show);
    }

    let h_input_ref = NodeRef::default();
    let w_input_ref = NodeRef::default();

    let h_input_ref2 = h_input_ref.clone();
    let w_input_ref2 = w_input_ref.clone();

    let create_bmp_callback = Callback::from(move |_| {
      //get height and width
      let height_input: HtmlInputElement = h_input_ref2.cast().unwrap();
      let width_input: HtmlInputElement = w_input_ref2.clone().cast().unwrap();
      let height: i32 = height_input.value().parse().unwrap();
      let width: u32 = width_input.value().parse().unwrap();
      link.send_message(Self::Message::CreateBMP(height, width));
      link.send_message(Self::Message::Hide);
    });

    let create_bmp = {
      create_bmp_callback.clone()
    };
  
    html! {
      <div style={"display: ".to_string()+&self.display}>
        <label for="height">{ "Height:" }</label>
        <input ref={h_input_ref} id="height" type="number" name="height" min="1" max="4200" value="10"/>
        <label for="width">{ "Width:" }</label>
        <input ref={w_input_ref}  id="width" type="number" name="width" min="1" max="4200" value="10"/>
        <button onclick={create_bmp}>{ "Create" }</button>
      </div>
    }
  }
}

// load
