use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;
use web_sys::HtmlInputElement;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
//use gloo_utils::document;
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
        self.display = "none".to_string();
        true
      },
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::CreateBMP(height, width) => {
        let _ = ctx.props().send_bmp_callback.emit(BMP::new(height, width, None));
        ctx.link().clone().send_message(Self::Message::Hide);
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

    let create_bmp = {
      let h_input_ref2 = h_input_ref.clone();
      let w_input_ref2 = w_input_ref.clone();
      ctx.link().batch_callback(move |_| {
        //get height and width
        let height_input: HtmlInputElement = h_input_ref2.cast().unwrap();
        let width_input: HtmlInputElement = w_input_ref2.clone().cast().unwrap();
        let height: i32 = height_input.value().parse().unwrap();
        let width: u32 = width_input.value().parse().unwrap();
        vec![Self::Message::CreateBMP(height, width), Self::Message::Hide]
      })
    };
  
    html! {
      <div style={"display: ".to_string()+&self.display}>
        <label for="height">{ "Height:" }</label>
        <input ref={h_input_ref} id="height" type="number" name="height" min="1" max="4200" value="10"/>
        <br class="mobile-only"/>
        <label for="width">{ "Width:" }</label>
        <input ref={w_input_ref}  id="width" type="number" name="width" min="1" max="4200" value="10"/>
        <br class="mobile-only"/>
        <button onclick={create_bmp}>{ "Create" }</button>
      </div>
    }
  }
}

// load

#[derive(PartialEq, Properties)]
pub struct LoadProps {
  pub send_bmp_callback: Callback<BMP>,
  pub show: bool,
}

pub enum LoadMessage {
  Show,
  Hide,
  GenBMP(File),
  LoadBMP(BMP),
}

pub struct Load {
  display: String,
  reader: Option<FileReader>,
}

impl Component for Load {
  type Message = LoadMessage;
  type Properties = LoadProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { display: "none".to_string(), reader: None }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Hide => {
        self.display = "none".to_string();
        true
      },
      Self::Message::Show => {
        self.display = "block".to_string();
        true
      },
      Self::Message::GenBMP(file) => {
        let link = ctx.link().clone();
        self.reader = Some(gloo::file::callbacks::read_as_bytes(&file, move |res| {
          //res.expect("Error reading file as bytes");
          let mut new_bmp = BMP::new(1, 1, None);
          new_bmp.contents = res.unwrap();
          link.send_message(Self::Message::LoadBMP(new_bmp));
          link.send_message(Self::Message::Hide);
        }));
        true
      }
      Self::Message::LoadBMP(bmp) => {
        let _ = ctx.props().send_bmp_callback.emit(bmp);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();

    //self.reader.is_none() is necessary so the component properly 
    if ctx.props().show && self.display == "none".to_string() {
      link.send_message(Self::Message::Show);
    }

    let file_input_ref = NodeRef::default();
    let file_input_ref2 = file_input_ref.clone();

    let load_bmp_callback = Callback::from(move |_| {
      log!("Loading");
      //get height and width
      let file_input: HtmlInputElement = file_input_ref2.cast().unwrap();
      let files = file_input.files().unwrap();
      let link2 = link.clone();
      if files.item(0).is_some() {
        log!(files.item(0).unwrap());
        //convert websys file to gloo file/blob
        let file = File::from(files.item(0).unwrap());
        link2.clone().send_message(Self::Message::GenBMP(file));
        link2.send_message(Self::Message::Hide);
      }
    });

    let load_bmp = {
      load_bmp_callback.clone()
    };
  
    html! {
      <div style={"display: ".to_string()+&self.display}>
        <label for="file-upload-initial">{ "File upload:" }</label>
        <input ref={file_input_ref} id="file-upload-initial" type="file" name="file-upload-initial" accept="image/bmp" multiple={false} />
        <br class="mobile-only"/>
        <button onclick={load_bmp}>{ "Load" }</button>
      </div>
    }
  }
}
