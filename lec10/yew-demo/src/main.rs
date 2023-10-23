#![allow(unused)]

use yew::prelude::*;

fn main() {
    let message = "hello there!";

    // JSX equivalent
    let my_html = html! {
        <div>
            <button onclick={|_| println!("button clicked!")}>{ "Clicky button!" }</button>
            <p>{ message }</p>
            <a>
                { "Here's another message, with some" }
                <b>{ "bold text" }</b>
                { "too!" }
            </a>
        </div>
    };
}
