use maud::{html, Markup};

use crate::page::page;

pub async fn index() -> Markup {
    let content = html! {
        div ."md:flex"."mt-[50px]".border-2.border-pink.justify-center {
            div .border-2.border-red.flex-1.w-full {
                p .text-text {
                    "about me"
                }
            }

            div .border-2.border-red.p-2.flex-1.w-full {
                "..."
            }
        }
    };
    
    page("about", content)
}
