use crate::page;
use maud::{html, Markup};

pub async fn index() -> Markup {
    let content = html! {
        // content
        div .flex.justify-center.w-full."mt-[50px]".flex-col.items-center {
            h1 .text-5xl.text-white.font-extrabold {
                "home"
            }
        }
    };

    page::page("home", content)
}
