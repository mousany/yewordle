use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MessageProps {
    #[prop_or(None)]
    pub message: Option<String>,
    #[prop_or(None)]
    pub grid: Option<String>,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    let MessageProps { message, grid } = props;

    let message_style = use_style!(
        r#"
        & {
          position: absolute;
          left: 50%;
          top: 80px;
          color: #fff;
          background-color: rgba(0, 0, 0, 0.85);
          padding: 16px 20px;
          z-index: 2;
          border-radius: 4px;
          transform: translateX(-50%);
          transition: opacity 0.3s ease-out;
          font-weight: 600;
        }
      "#
    );

    html! {
      <>
        if let Some(message) = message {
          <div class={message_style}>
            { message }

            if let Some(grid) = grid {
              <pre>
                { grid }
              </pre>
            }
          </div>
        }
      </>
    }
}
