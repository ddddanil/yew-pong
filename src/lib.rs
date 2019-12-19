extern crate stdweb;
extern crate yew;

use core::time::Duration;
use yew::{
  prelude::*,
  html::EmptyBuilder,
  services::{
    ConsoleService,
    // KeyboardService,
    timeout::{
      TimeoutService,
      TimeoutTask
    }
  }
};
use stdweb::{
  Reference,
  web::{ window, IEventTarget, event::IKeyboardEvent }
};

pub struct Model {
  console: ConsoleService,
  // keyboard: KeyboardService,
  timeout: TimeoutService,
  timet: TimeoutTask,
  link: ComponentLink<Self>,
  posl: f32,
  posr: f32,
  scorel: u8,
  scorer: u8,
  ballx: f32,
  bally: f32,
  ballvx: f32,
  ballvy: f32,
}

#[derive(Debug, Clone)]
pub enum Msg {
  KeyDown(KeyDownEvent),
  Tick,
  MoveDown(f32),
  MoveUp(f32),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(Msg::KeyDown);
    window().add_event_listener(move |event: KeyDownEvent| { callback.emit(event); });
    let mut service = TimeoutService::new();
    let c = link.callback(|_| Msg::Tick);
    let timeout = service.spawn(Duration::from_millis(20), c);
    Model { 
      console: ConsoleService::new(), 
      // keyboard: KeyboardService::new(),
      timeout: service,
      timet: timeout,
      link, 
      posl: 0.3, 
      posr: 0.1, 
      scorel: 0, 
      scorer: 0,
      ballx: 0.0,
      bally: 0.5,
      ballvx: 1.0,
      ballvy: 1.0,
    }
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
        self.key_event(e.code())
      }
      Msg::Tick => {
        self.tick();
        let c = self.link.callback(|_| Msg::Tick);
        self.timet = self.timeout.spawn(Duration::from_millis(20), c);
        true
      }
    }
  }

  fn view(&self) -> Html {
    let move_down = self.link.callback(|_: ()| Msg::MoveDown(0.05));
    html! {
      <div class="game">
        <div class="paddle paddle-left"  style={ format!("top: {}px;", Model::px_to_paddley(Model::height_to_px(self.posl))) } />
        <div class="paddle paddle-right" style={ format!("top: {}px;", Model::height_to_px(self.posr)) } />
        <div class="ball"                style={ format!("top: {}px; left: {}px;", Model::px_to_bally(Model::height_to_px(self.bally)), Model::px_to_ballx(Model::width_to_px(self.ballx))) } />
      </div>
    }
  }
}

impl Model {
  fn key_event(&mut self, code: String) -> ShouldRender {
    match code.as_ref() {
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

  fn tick(&mut self) {
    self.ballx += 0.005 * self.ballvx;
    if self.ballx > 1.0 {
      self.ballx = 1.0;
      if let Some(k) = self.intersect_paddle(self.bally, self.posr) {
        self.ballvy = k * self.ballvy.signum();
      } else {
        self.scorel += 1;
        self.console.info(&*format!("New score {}:{}", self.scorel, self.scorer))
      }
      self.ballvx *= -1.0;
    }
    if self.ballx < 0.0 {
      self.ballx = 0.0;
      if let Some(k) = self.intersect_paddle(self.bally, self.posl) {
        self.ballvy = k * self.ballvy.signum();
      } else {
        self.scorer += 1;
        self.console.info(&*format!("New score {}:{}", self.scorel, self.scorer))
      }
      self.ballvx *= -1.0;
    }

    self.bally += 0.005 * self.ballvy;
    if self.bally > 1.0 {
      self.bally = 1.0;
      self.ballvy *= -1.0;
    }
    if self.bally < 0.0 {
      self.bally = 0.0;
      self.ballvy *= -1.0;
    }
  }

  fn px_to_paddley(px: i32) -> i32 {
    (px * 430) / 480
  }

  fn px_to_ballx(px: i32) -> i32 {
    (px * 570) / 590 + 10
  }

  fn px_to_bally(px: i32) -> i32 {
    px - 100
  }

  fn height_to_px(pos: f32) -> i32 {
    (480.0 * pos) as i32
  }

  fn width_to_px(pos: f32) -> i32 {
    (590.0 * pos) as i32
  }

  fn intersect_paddle(&mut self, b: f32, p: f32) -> Option<f32> {
    let u = 0.1 * p;
    let d = 0.1 - u;
    let up = p - u;
    let dp = p + d;
    let m = (up + dp) * 0.5;
    self.console.debug(&*format!("Intersect from {} to {} with {}", up, dp, b));
    if b >= up && b <= dp {
      let k = (m - b).abs() * 35.0;
      self.console.debug(&*format!("Intersected at {} from the middle {}", k, m));
      Some(k)
    }
    else { None }
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
