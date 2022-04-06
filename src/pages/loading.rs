use gloo_console::console;
use gloo_timers::callback::Timeout;
use rand::Rng;
use yew::{prelude::*, suspense::Suspension, suspense::SuspensionResult};

use crate::utils::use_render_count;

#[derive(Properties, PartialEq)]
struct DelayedProps {
    delay: u32,
}

#[hook]
fn use_delay(ms: u32) -> SuspensionResult<u32> {
    #[derive(Clone, Copy)]
    enum State {
        Idle,
        Waiting,
        Done,
    }

    let (s, handle) = Suspension::new();
    let state = use_mut_ref(|| State::Idle);
    let current_state = *state.borrow();

    let res = match current_state {
        State::Idle => {
            *state.borrow_mut() = State::Waiting;
            console!(format!("Starting Timeout for {}ms", ms));
            let timeout = Timeout::new(ms, move || {
                handle.resume();
                *state.borrow_mut() = State::Done;
                console!(format!("Timeout Complete {}ms", ms));
            });
            timeout.forget(); // TODO: Figure out how to cleanup here
            Err(s)
        }
        State::Waiting => Err(s),
        State::Done => Ok(1),
    };
    res
}

#[function_component]
fn Delayed(props: &DelayedProps) -> HtmlResult {
    let _delay = use_delay(props.delay)?;

    Ok(html! {
      <>
        {"Delay of "} { props.delay } {" completed"}
      </>
    })
}

#[function_component]
pub fn Loading() -> Html {
    let render_count = use_render_count();

    let elements = use_state(|| {
        let mut rng = rand::thread_rng();
        [0; 100].map(|_| rng.gen_range(1000..10000))
    });

    let fallback = |e| html! { <>{"Delay "}{e}{" waiting..."}</>};

    html! {
      <>
        <p>{"This demonstrates Suspense by creating multiple components that load in after some time (indicated by the fallback)"}</p>
        <p>{"Render Count:"}{render_count}</p>
        <hr />
        <div>
          { for (*elements).iter().map(|e| {
            html! {
              <p>
                <Suspense fallback={fallback(e)}>
                  <Delayed delay={e} />
                </Suspense>
              </p>
            }
          })}
        </div>
      </>
    }
}
