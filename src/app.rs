use yew::prelude::*;
use crate::components::project;
use crate::components::socials;
use crate::components::socials::Link;

#[function_component(App)]
pub fn app() -> Html {
    let projects = vec![
        project::Props {
            title: AttrValue::from("personal website"),
            description: AttrValue::from("written in Rust using yew.rs and WebAssembly"),
            href: AttrValue::from("#")
         },
         project::Props {
            title: AttrValue::from("something cool"),
            description: AttrValue::from("gotta start somewhere"),
            href: AttrValue::from("#")
         }
    ];

    let links = socials::Props {
        title: AttrValue::from("socials"),
        links: vec![
            Link { title: AttrValue::from("LinkedIn"), href: AttrValue::from("#") },
            Link { title: AttrValue::from("GitHub"), href: AttrValue::from("#") },
            Link { title: AttrValue::from("Discord"), href: AttrValue::from("#") },
        ]
    };

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
            </div>
            <socials::SocialCard ..links />
        </main>

        <footer>
           { "made with rust and wasm" } 
        </footer>
        </> 
    }
}
