use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::use_url,
    nested_router::Outlet,
    path, StaticSegment,
};
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    language: Language,
}

#[derive(Clone, Debug, Default)]
enum Language {
    #[default]
    Thai,
    English,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
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

        // sets the document title
        <Title text="พิสูจน์หลักฐานจังหวัดภูเก็ต" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <ParentRoute path=path!("/en") view=English>
                        <Route path=StaticSegment("") view=HomePage />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn English() -> impl IntoView {
    provide_context(Language::English);
    view! { <Outlet /> }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let language: Language = use_context().unwrap_or(Language::Thai);

    view! {
        <Header language=&language />
        <h1>HomePage</h1>
        <h1>HomePage</h1>
        <h1>HomePage</h1>
        <h1>HomePage</h1>
        <Footer language=&language />
    }
}

#[component]
fn Header<'a>(language: &'a Language) -> impl IntoView {
    let url = use_url().get_untracked();
    view! {
        <header>
            <img class="logo" src="/public/logo.png" />
            {match language {
                Language::Thai => {
                    view! {
                        <nav>
                            <a href="">"หน้าแรก"</a>
                            <a>"ติดต่อ"</a>
                            <a>"เกี่ยวกับ"</a>
                            <a href=format!("/en{}", url.path())>"English"</a>
                        </nav>
                    }
                }
                Language::English => {
                    view! {
                        <nav>
                            <a href="">"Home"</a>
                            <a>"Contact"</a>
                            <a>"About"</a>
                            <a href=format!(
                                "{}",
                                url.path().trim_start_matches("/en"),
                            )>"ภาษาไทย"</a>
                        </nav>
                    }
                }
            }}
        </header>
    }
}

#[component]
fn Footer<'a>(language: &'a Language) -> impl IntoView {
    view! {
        <footer>
            <img class="logo" src="/public/white_logo.png" />
            {match language {
                Language::Thai => {
                    view! {
                        <div>
                            <p>
                                "พิสูจน์หลักฐานจังหวัดภูเก็ต"
                                <br />
                                "323/39 ถนนเยาวราช ตำบลตลาดใหญ่"
                                <br />
                                "อำเภอเมืองภูเก็ต จังหวัดภูเก็ต 83000"
                                <br /> "โทรศัพท์: 0 - 7621 - 1176"
                            </p>
                        </div>
                        <div>
                            <p>
                                "พิสูจน์หลักฐานจังหวัดภูเก็ต (ไม้ขาว)"
                                <br /> "102 หมู่ที่ 7 ตำบลไม้ขาว"
                                <br />
                                "อำเภอถลาง จังหวัดภูเก็ต 83110"
                                <br /> "โทรศัพท์: 0 - 7653 - 0124"
                            </p>
                        </div>
                        <div>
                            <p>เว็บไซต์ที่เกี่ยวข้อง</p>
                            <ul>
                                <li>
                                    <a rel="external" href="https://www.royalthaipolice.go.th">
                                        "สำนักงานตำรวจแห่งชาติ"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://www.forensic.police.go.th">
                                        "สำนักงานพิสูจน์หลักฐานตำรวจ"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://criminal.police.go.th">
                                        "กองทะเบียนประวัติอาชญากร"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://scdc8.forensic.police.go.th">
                                        "ศูนย์พิสูจน์หลักฐาน 8"
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div></div>
                    }
                }
                Language::English => todo!(),
            }}
        </footer>
    }
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
