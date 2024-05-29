use std::ops::Add;

use super::*;

/// The function is used to pair a CSS class with an associated key (which is mandatory for
/// performance reasons) and wrap it around an HTML element. You can use it for any custom CSS
/// wrapping. If you want to do anything meaningful with this function you'd want to use some
/// associated class or id to track the style.
#[allow(dead_code)]
pub fn html_wrapper(item: Html, key: String, class: Option<String>, id: Option<String>) -> Html {
    html! {
        <div key={key} class={class} id={id}>
            {item}
        </div>
    }
}

// NOTE: Unused
// (My GitHub account got flagged so I'm tired of relying on anything related to Microsoft.)
#[allow(unused)]
pub fn set_iframe_gist(link: &str, height: Option<u32>) -> Html {
    let auto_for_none = || match height {
        Some(_) => None,
        None => Some(String::from("auto")),
    };
    html! {
        <iframe frameborder=0 class={data::IFRAME_GIST}
                scrolling={"no"} seamless={"seamless"}
                style={format!("height: {}px", auto_for_none().unwrap_or(height.unwrap_or(0).add(48).to_string()))}
                srcdoc={format!(
                "<html>
                    <body>
                        <style type=\"text/css\">
                        .gist,
                        .gist-data {{
                            height: {}px;
                        }}
                        </style>
                        <script 
                            src=\"{}\">
                        </script>
                    </body>
                </html>", auto_for_none().unwrap_or(height.unwrap_or(0).to_string()), link.to_owned() + ".js")}
        ></iframe>
    }
}

pub fn set_iframe_godbolt(link: &'static str, width: Option<u32>, height: Option<u32>) -> Html {
    let width_default = || match width {
        Some(_) => None,
        None => Some(String::from("")),
    };
    let height_default = || match height {
        Some(_) => None,
        None => Some(String::from("")),
    };
    // Remove the iframe shenanigans. (It looks funky I know)
    let link = link
        .replace("<iframe width=\"800px\" height=\"200px\" src=\"", "")
        .replace("\"></iframe>", "");
    html! {
        <iframe class={data::GODBOLT_IFRAME}
                src={link}
                width={width_default().unwrap_or(width.unwrap_or(0).to_string())}
                height={height_default().unwrap_or(width.unwrap_or(0).to_string())}
        ></iframe>
    }
}

/// The link has to be an embed link. Example:
/// https://www.youtube-nocookie.com/embed/fwxjMKBMR7s
pub fn set_youtube_iframe(link: &'static str) -> Html {
    html! {
        <div class={"youtube-video-container"}>
                <iframe
                    height="100%"
                    width="100%"
                    src={link}
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen=true>
                </iframe>
        </div>
    }
}
