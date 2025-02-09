use dioxus::prelude::*;

const STYLE_CSS: Asset = asset!("/assets/style.css");
const HEADER_CSS: Asset = asset!("/assets/header.css");
const HERO_CSS: Asset = asset!("/assets/hero.css");
const FOOTER_CSS: Asset = asset!("/assets/footer.css");

const FAVICON: Asset = asset!("/assets/favicon.ico");
const PHONE_ICON: Asset = asset!("/assets/phone.ico");
const FACEBOOK_ICON: Asset = asset!("/assets/facebook.ico");
const HEADER_LOGO: Asset = asset!("/assets/logo.png");
const FOOTER_LOGO: Asset = asset!("/assets/white_logo.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name:"viewport", content:"width=device-width, initial-scale=1" }
        document::Link { rel: "icon", href: FAVICON }
        document::Script { crossorigin: "anonymous", src: "https://kit.fontawesome.com/yourcode.js" }
        document::Stylesheet { href: STYLE_CSS }
        document::Title { "พิสูจน์หลักฐานจังหวัดภูเก็ต" }

        body {
            Header {  }
            Hero {  }
            News {  }
            Footer {  }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        document::Stylesheet { href: HEADER_CSS }

        header {
            img { class: "logo", src: HEADER_LOGO }
            nav {
                button { "หน้าแรก" }
                button { "ติดต่อ" }
                button { "โครงสร้างหน่วยงาน" }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        document::Stylesheet { href: HERO_CSS }

        div { class:"hero-placeholder"
        }
    }
}

#[component]
fn News() -> Element {
    rsx! {
        div { class: "news-card",
            div { class: "news-placeholder" }
            div { class: "news-details",
                h3 { "Title" }
                div { class: "bottom-bar",
                    p { "15-02-2568" }
                    button { "อ่านต่อ" }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        document::Stylesheet { href: FOOTER_CSS }
        footer {
            img { class: "logo", src: FOOTER_LOGO }
            div { id: "address",
                h3 { "ติดต่อ" }
                p { "พิสูจน์หลักฐานจังหวัดภูเก็ต (Phuet Forensic Police)" }
                p { "323/39 ถนนเยาวราช ตำบลวิชิต อำเภอเมืองภูเก็ต จังหวัดภูเก็ต 83000" }
                p {
                    img { class: "icon", src: PHONE_ICON }
                    " 0 7621 1176 (อาคารในเมือง), 0 7621 1176 (อาคารไม้ขาว)"
                }
                p {
                    img { class: "icon", src: FACEBOOK_ICON }
                    a {
                        href: "https://www.facebook.com/phuketforensicpolice",
                        " www.facebook.com/phuketforensicpolice/"
                    }
                }
            }
            nav {
                ul {
                    li { h3 { "หน่วยงานในสังกัด" } }
                    li { a { href: "https://forensic.police.go.th", "สำนักงานพิสูจน์หลักฐานตำรวจ" } }
                    li { a { href: "https://criminal.police.go.th", "กองทะเบียนประวัติอาชญากร" } }
                    li { a { href: "http://scdc8.forensic.police.go.th", "ศูนย์พิสูจน์หลักฐาน 8" } }
                }
            }
        }
    }
}
