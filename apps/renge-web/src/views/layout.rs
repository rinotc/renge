use maud::{DOCTYPE, Markup, html};

pub fn layout(title: &str, content: Markup) -> String {
    html! {
        (DOCTYPE) html lang="ja" data-theme="light" {
            head { 
                meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1"; title { (title) " | 蓮華" } link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/daisyui@5"; script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {} script src="https://unpkg.com/htmx.org@2.0.4" {} 
            } 
            body class="min-h-screen bg-base-200" {
                header class="navbar bg-base-100 shadow-sm" {
                    div class="mx-auto w-full max-w-6xl px-4" {
                        a href="/" class="text-xl font-bold" { "蓮華" } 
                        span class="ml-3 text-sm text-base-content/60" { "イベント管理" } 
                    } 
                }
                main class="mx-auto max-w-6xl p-4 md:p-8" { (content) } 
            } 
        }
    }.into_string()
}
