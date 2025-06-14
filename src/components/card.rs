use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub content: AttrValue,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <div class="card">
            <div class="card-title">{props.title.clone()}</div>
            {props.content.clone()}
        </div>
    }
}
