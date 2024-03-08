use maud::{html, Markup, DOCTYPE};

// html boilerplate
pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head(title))
            (body(content))
        }
    }
}

fn head(title: &str) -> Markup {
    html! {
           head {
               // metadata
               meta charset="UTF-8";
               meta name="viewport" content="width=device-width, initial-scale=1.0";
               title { (title) }
               // tailwind
               link href="/assets/css/output.css" rel="stylesheet";
           }
       }
}

fn body(content: Markup) -> Markup {
    html! {
        body ."bg-base" {
            (content)
            script src="/assets/js/htmx.min.js" {}
        }
    }
}
