use maud::{html, Markup};
use crate::page;

pub async fn index() -> Markup {
    let content = html! {
        button
            type="button" hx-get="/hello" hx-swap="outerHTML"
            ."bg-red-100"
            { "Click my buttons" }

        h1 ."py-10 text-lg hover:text-sm" { "Yo" }
    };

    page::page("home", content)
}
