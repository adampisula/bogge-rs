use yew::{Component, Html, html, Context, Properties, Callback, MouseEvent};

pub struct GraphComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct GraphComponentProps {
  pub values: Vec<i64>,
  pub onclick: Callback<MouseEvent>,
}

impl Component for GraphComponent {
  type Message = ();
  type Properties = GraphComponentProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let max_value = ctx.props()
      .values
      .to_owned()
      .into_iter()
      .max()
      .expect("Empty vector") as f64;
    let onclick = &ctx.props().onclick;

    html! {
      <div
        class="graph-component"
        {onclick}
      >
        {
          ctx.props()
            .values
            .to_owned()
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
              let height = 100.0 * (value as f64) / max_value;

              html! {
                <div
                  key={index}
                  class="graph-component__bar"
                  style={format!("
                    height: {}%;
                  ", height)}
                  title={format!("Value: {}", value)}
                ></div>
              }
            })
            .collect::<Html>()
        }
      </div>
    }
  }
}