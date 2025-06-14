use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub description: AttrValue,
    pub href: AttrValue
}

#[function_component]
pub fn Project(props: &Props) -> Html {
    html! {
        <li>
            <a href={props.href.clone()}>{props.title.clone()}</a>
        </li>
    }
}
