#![allow(unused)]

use yew::prelude::*;

fn main() {
    let message = "hello there!";
    let count = use_state(|| 0);

    // JSX equivalent
    let my_html = html! {
        <div>
            <button onclick={|_| { count += 1; }}>{ "Clicky button!" }</button>
            <p>{ message }</p>
            <a>
                { "You have clicked the button" }
                <b>{ count }</b>
                { "times!" }
            </a>
        </div>
    };
}
