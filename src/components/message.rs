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
          font-weight: 600;          
        }
      "#
    );

    let message_transition_style = use_style!(
        r#"
        & {
          transition: opacity 0.5s ease;
        }

        &.show {
          opacity: 1;
        }

        &.hide {
          opacity: 0;
        }
      "#
    );

    html! {
      <>
        <div
          class={
            vec![vec![message_transition_style.get_class_name().to_string()]
              , {
                if message.is_some() {
                  vec!["show".to_string(), message_style.get_class_name().to_string()]
                } else {
                  vec!["hide".to_string()]
                }
              }
            ].concat()
          }
        >
          if let Some(message) = message {
            { message }
          }
          if let Some(grid) = grid {
            <pre>
              { grid }
            </pre>
          }
        </div>
      </>
    }
}
