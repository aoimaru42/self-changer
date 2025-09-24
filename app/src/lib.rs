mod pages;
mod api_client;
pub mod api;
mod css_sanitizer;
use crate::pages::chat_page::ChatPage;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

// #[cfg(feature = "ssr")]
// use server::api_handlers::send_message; // serverクレートから関数をインポート

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/self-changer.css"/>

        // sets the document title
        <Title text="Self Changer"/>

        // content for this welcome page
        <Router>
            <main class="w-full h-screen flex flex-col items-center justify-center bg-gray-200 py-2">
                <Routes fallback=|| "ページが見つかりません".into_view()>
                    <Route path=StaticSegment("") view=ChatPage/>
                </Routes>
            </main>
        </Router>
    }
}
