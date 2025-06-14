use yew::prelude::*;

#[function_component]
pub fn SocialCard() -> Html {
    html! {
       <div class="card social-card">
            <div class="card-title">{ "socials" }</div>
                <div class="links">
                        <a href="https://github.com/Alespren"><i class="nf nf-dev-github"></i>{ "GitHub" }</a>
                        <a href="https://linkedin.com/in/ashley-pedersen-4a2048294"><i class="nf nf-dev-linkedin"></i>{ "LinkedIn" }</a>
                </div>
       </div> 
    }
}

