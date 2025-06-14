use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub links: Vec<Link>
}

#[derive(PartialEq, Clone)]
pub struct Link {
    pub title: AttrValue,
    pub href: AttrValue
}

#[function_component]
pub fn SocialCard(p: &Props) -> Html {
    html! {
       <div class="card">
            <div class="card-title">{ p.title.clone() }</div>
            <div class="links">
            {
                p.links.clone().into_iter().map(|l| {
                    html!{<a key={l.title.to_string()} href={l.href}>{l.title}</a>}
                }).collect::<Html>()
            }
            </div>
       </div> 
    }
}

