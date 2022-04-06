use yew::prelude::*;

#[hook]
pub fn use_render_count() -> i32 {
    let counter = use_mut_ref(|| 0);

    use_effect({
        let counter = counter.clone();
        move || {
            *counter.borrow_mut() += 1;

            || ()
        }
    });

    return *counter.borrow() + 1;
}
