use maud::{html, Markup, PreEscaped, DOCTYPE};

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
               style { (PreEscaped(include_str!(concat!(env!("OUT_DIR"), "/output.css")))) }
           }
       }
}

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            script src="/_assets/js/htmx.min.js" {}
        }
    }
}
