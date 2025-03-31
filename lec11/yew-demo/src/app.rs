use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Counter: " }{ *counter }</h1>
            <button onclick={move |_| counter.set(*counter + 1)}>{ "Add" }</button>
        </main>
    }
}
