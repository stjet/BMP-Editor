use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct StartProps {
  pub create_load_callback: Callback<bool>,
}

pub enum StartMessage {
  Show,
  Hide,
  SendCreate,
  SendLoad,
}

pub struct Start {
  display: String,
}

impl Component for Start {
  type Message = StartMessage;
  type Properties = StartProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "inline-block".to_string() }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Hide => {
        self.display = "none".to_string();
        true
      },
      Self::Message::Show => {
        self.display = "inline-block".to_string();
        true
      },
      Self::Message::SendCreate => {
        let _ = ctx.props().create_load_callback.emit(true);
        self.display = "inline-block".to_string();
        true
      },
      Self::Message::SendLoad => {
        let _ = ctx.props().create_load_callback.emit(false);
        self.display = "inline-block".to_string();
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    //true means file from scratch (ask user about dimensions, etc)
    //false means ask user to upload file
    //let mut scratch: bool = false;

    let create_new = ctx.link().batch_callback(move |_| {
      vec![Self::Message::SendCreate, Self::Message::Hide]
    });

    let load_from_file = ctx.link().batch_callback(move |_| {
      vec![Self::Message::SendLoad, Self::Message::Hide]
    });
  
    html! {
      <div id="start" style={"display: ".to_string()+&self.display}>
        <button onclick={create_new}>{ "Create New" }</button>
        <button onclick={load_from_file}>{ "Load from File" }</button>
      </div>
    }
  }
}
