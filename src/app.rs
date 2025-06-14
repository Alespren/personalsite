use crate::components::project;
use crate::components::socials;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let projects = vec![
        project::Props {
            title: AttrValue::from("personal website"),
            description: AttrValue::from("written in Rust using yew.rs and WebAssembly"),
            href: AttrValue::from("#"),
        },
    ];

    html! {
            <>
            <main>
                <div class="title">{ "ashley pedersen" }</div>
                <div class="subtitle">
                    { "full stack developer and student" }
                </div>
                <div class="card">
                    <div class="card-title">{ "projects" }</div>
                     <ul>
                        {
                            projects.into_iter().map(|props| {
                                html!{
                                    <project::Project ..props />
                                }
                            }).collect::<Html>()
                        }
                    </ul>

                    <div class="stay-tuned">{ "stay tuned for more" }</div>
                </div>
                <socials::SocialCard />
            </main>

            <footer>
               { "made with rust and wasm" }
            </footer>
            </>
        }
}
