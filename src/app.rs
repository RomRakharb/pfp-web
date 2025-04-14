use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    nested_router::Outlet,
    path, StaticSegment,
};

use crate::component::{Footer, Header};

#[derive(Clone, Debug, Default)]
pub enum Language {
    #[default]
    Thai,
    English,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="th">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pfp-web.css" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("/en") view=English>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=path!("/news") view=NewsPage />
                    </ParentRoute>
                    <ParentRoute path=path!("") view=Thai>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=path!("/news") view=NewsPage />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn English() -> impl IntoView {
    provide_context(Language::English);
    view! {
        <Title text="Phuket Forensic Police" />
        <Outlet />
    }
}

#[component]
fn Thai() -> impl IntoView {
    view! {
        <Title text="พิสูจน์หลักฐานจังหวัดภูเก็ต" />
        <Outlet />
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Header />
        <Hero />
        <NewsAndAnnoucement />
        <Footer />
    }
}

#[component]
fn NewsPage() -> impl IntoView {
    view! {
        <Header />
        <h1>"News"</h1>
        <Footer />
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! { <section class="hero-placeholder"></section> }
}

#[component]
fn NewsAndAnnoucement() -> impl IntoView {
    view! {
        <section class="news-announcement">
            <h1>"ข่าว / ประกาศ"</h1>
        </section>
    }
}

#[component]
fn CriminalRecordCheck() -> impl IntoView {
    view! {}
}

#[component]
fn Example() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
