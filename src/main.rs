mod components;

use yew::prelude::*;
use wasm_logger;
use components::graph::GraphComponent;

enum Msg {
  AddOne,
  SubtractOne,
  IncrementCounter,
}

struct Model {
  value: i64,
  counter: i64,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      value: 5,
      counter: 0,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::AddOne => {
        self.value += 1;
      }

      Msg::SubtractOne => {
        self.value -= 1;
      }

      Msg::IncrementCounter => {
        self.counter += 1;
      }
    }

    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();
    let items: Vec<i64> = (1..(self.value + 1))
      .map(|v| v*v)
      .collect();

    // let onclick_cb = Callback::from(|_| {
    //   link.callback(|_| Msg::IncrementCounter);

    //   log::info!("Counter: {}", self.counter);
    // });

    html! {
      <div>
        <button
          onclick={link.callback(|_| Msg::AddOne)}
        >
          { "++" }
        </button>
        <button
          onclick={link.callback(|_| Msg::SubtractOne)}
        >
          { "--" }
        </button>
        <p>{ format!("Value: {}", self.value) }</p>
        <p>{ format!("Counter: {}", self.counter) }</p>
        <GraphComponent
          values={items}
          onclick={link.callback(|_| {
            log::info!("Counter inc");

            Msg::IncrementCounter
          })}
        />
      </div>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
  wasm_logger::init(wasm_logger::Config::default());
}
