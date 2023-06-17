// Run `trunk serve --open` to start the development server
// The port the server will run is 8080
// The server can be found at `http://127.0.0.1:8080`

// The Tutorial I followed can be found at the following link:
// https://yew.rs/docs/getting-started/build-a-sample-app

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html! {
        <>
            <div
                class="
                    w-100
                    p-3
                "
                style="
                    background-color: lightblue;
                "
            >
                <h1>{ "Rust Web Application Yew!" }</h1>
            </div>
            <div
                class="
                    container
                "
                style="
                    height: 93vh;
                "
            >
                <div>
                    <h1
                        class="
                            h3
                        "
                    >
                        { "Introduction to Yew" }
                    </h1>
                    <p>
                        { "The page you are view now was developed using Rust and a Web Framework called Yew!" }
                    </p>
                </div>
                <div
                    class="
                        d-flex
                        flex-column
                        align-items-center
                    "
                >
                    <p>{ *counter }</p>
                    <button
                        class="
                            btn
                            btn-primary
                        "
                        {onclick}
                    >
                        { "+" }
                    </button>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}