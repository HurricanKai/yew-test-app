use gloo_timers::callback::Timeout;
use rand::Rng;
use std::mem::replace;
use std::ops::DerefMut;
use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionHandle, SuspensionResult},
};

use crate::utils::use_render_count;

#[derive(Properties, PartialEq)]
struct DelayedProps {
    delay: u32,
}

// thanks @WorldSEnder for the improved delay hook
#[hook]
fn use_delay(ms: u32) -> SuspensionResult<bool> {
    enum TriggerState {
        Done,
        // I couldn't find a way to tie the knot here.
        // This state exists only during the first initialization.
        Initial,
        Waiting(SuspensionHandle),
    }
    let is_initialized = use_state(|| false);
    let trigger_state = use_mut_ref(|| TriggerState::Initial);
    {
        let is_initialized = is_initialized.clone();
        let trigger_state = trigger_state.clone();
        use_effect_with_deps(
            move |_| {
                let timeout = Timeout::new(ms, move || {
                    match replace(trigger_state.borrow_mut().deref_mut(), TriggerState::Done) {
                        TriggerState::Done | TriggerState::Initial => unreachable!("Impossible!"),
                        TriggerState::Waiting(handle) => handle.resume(),
                    }
                });
                is_initialized.set(true);

                || {
                    timeout.cancel();
                }
            },
            (),
        );
    };

    if !*is_initialized {
        return Ok(true);
    }

    let mut trigger_state = trigger_state.borrow_mut();
    if let TriggerState::Done = *trigger_state {
        Ok(false)
    } else {
        let (suspense, handle) = Suspension::new();
        *trigger_state = TriggerState::Waiting(handle);
        Err(suspense)
    }
}

#[function_component]
fn Delayed(props: &DelayedProps) -> HtmlResult {
    let is_server_side = use_delay(props.delay)?;

    if is_server_side {
        return Ok(html! {
          {"Please wait while your client loads..."}
        });
    }

    let fallback = html! { {"Loading Rec"} };

    Ok(html! {
      <>
        {"Delay of "} { props.delay } {" completed"}
        <ul>
          <li>
            <Suspense fallback={fallback}>
              <Delayed delay={10000} />
            </Suspense>
          </li>
        </ul>
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
        <ul>
          { for (*elements).iter().map(|e| {
            html! {
              <li>
                <Suspense fallback={fallback(e)}>
                  <Delayed delay={e} />
                </Suspense>
              </li>
            }
          })}
        </ul>
      </>
    }
}
