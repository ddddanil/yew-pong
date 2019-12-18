extern crate stdweb;
extern crate yew;

use yew::{
  prelude::*,
  html::EmptyBuilder,
  services::ConsoleService,
};
use stdweb::{
  Reference,
  web::{ window, IEventTarget, event::IKeyboardEvent }
};

pub struct Model {
  console: ConsoleService,
  link: ComponentLink<Self>,
  posl: f32,
  posr: f32,
  scorel: u8,
  scorer: u8,
}

#[derive(Debug, Clone)]
pub enum Msg {
  KeyDown(KeyDownEvent),
  MoveDown(f32),
  MoveUp(f32),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::KeyDown);
    window().add_event_listener(move |event: KeyDownEvent| { callback.emit(event); });
    Model { console: ConsoleService::new(), link, posl: 0.3, posr: 0.1, scorel: 0, scorer: 0 }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::MoveDown(amount) => {
        self.move_down(amount);
        true
      }
      Msg::MoveUp(amount) => {
        self.move_up(amount);
        true
      }
      Msg::KeyDown(e) => {
        // self.console.log(&*format!("{:?}", e.code()));
        match e.code().as_ref() {
          "ArrowUp" => {
            self.move_up(0.05);
            true
          }
          "ArrowDown" => {
            self.move_down(0.05);
            true
          }
          _ => false
        }
      }
      _ => false
    }
  }

  fn view(&self) -> Html {
    let move_down = self.link.callback(|_| Msg::MoveDown(0.05));
    html! {
      <div class="game">
        <div class="paddle paddle-left" onkeydown=move_down style={ format!("top: {}px;", Model::height_to_px(self.posl)) } />
        <div class="paddle paddle-right" style={ format!("top: {}px;", Model::height_to_px(self.posr)) } />
      </div>
    }
  }
}

impl Model {
  fn height_to_px(pos: f32) -> u16 {
    (420.0 * pos) as u16
  }

  fn move_up(&mut self, amount: f32) {
    self.posl -= amount;
    if self.posl < 0.0 {
      self.posl = 0.0;
    }
  }

  fn move_down(&mut self, amount: f32) {
    self.posl += amount;
    if self.posl > 1.0 {
      self.posl = 1.0;
    }
  }
}
