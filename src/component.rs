use leptos::prelude::*;
use leptos_router::hooks::use_url;

use crate::app::Language;

#[component]
pub fn Header() -> impl IntoView {
    let language: Language = use_context().unwrap_or(Language::Thai);
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
pub fn Footer() -> impl IntoView {
    let language: Language = use_context().unwrap_or(Language::Thai);
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
                            <p>"เว็บไซต์ที่เกี่ยวข้อง"</p>
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
                    }
                }
                Language::English => {
                    view! {
                        <div>
                            <p>
                                "Phuket Forensic Police" <br /> "323/39 Yaowarat Road, Talat Yai,"
                                <br /> "Mueang Phuket, Phuket 83000" <br /> "Tel: 0 - 7621 - 1176"
                            </p>
                        </div>
                        <div>
                            <p>
                                "Phuket Forensic Police (Mai Khao)" <br /> "102 Moo 7, Mai Khao,"
                                <br /> "Thalang, Phuket 83110" <br /> "Tel: 0 - 7653 - 0124"
                            </p>
                        </div>
                        <div>
                            <p>"Related Sites"</p>
                            <ul>
                                <li>
                                    <a rel="external" href="https://www.royalthaipolice.go.th">
                                        "Royal Thai Police"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://www.forensic.police.go.th">
                                        "Office of Police Forensic Science"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://criminal.police.go.th">
                                        "Criminal Record Division"
                                    </a>
                                </li>
                                <li>
                                    <a rel="external" href="https://scdc8.forensic.police.go.th">
                                        "Police Forensic Science Center 8"
                                    </a>
                                </li>
                            </ul>
                        </div>
                    }
                }
            }}
        </footer>
    }
}
