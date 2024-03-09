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

            // global stylesheet
            link href="/assets/css/global.css" rel="stylesheet";
        }
    }
}

fn body(content: Markup) -> Markup {
    html! {
        body .bg-base.font-eb_garamond.font-signika_negative {
            (navbar())
            (content)
            script src="/assets/js/htmx.min.js" {}
        }
    }
}

fn navbar() -> Markup {
    html! {
        div .mt-3.bg-crust.max-w-6xl.w-full.mx-auto.rounded-md.py-2.border-pink.border-2.shadow-lg.text-lg {
            ul .flex.w-full.px-2.text-pink.justify-around.font-bold {
                li {
                    a href="/home" {
                        "home"
                    }
                }
                li {
                    a href="/gallery" {
                        "gallery"
                    }
                }
                li {
                    a href="/about" {
                        "about"
                    }
                }
                li {
                    a href="/contact" {
                        "contact"
                    }
                }
            }
        }
    }
}
