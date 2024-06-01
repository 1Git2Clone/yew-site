use super::*;

use crate::data::footer::*;
use crate::structs::ImageElement;

#[function_component(Footer)]
pub fn footer() -> Html {
    use crate::enums::Socials as s;

    let twitter = get_socials_images(s::Twitter);
    let github = get_socials_images(s::GitHub);
    let gitlab = get_socials_images(s::GitLab);
    let codeberg = get_socials_images(s::Codeberg);
    let discord = get_socials_images(s::Discord);

    let set_footer_img = |key: Option<&str>,
                          img_element: &ImageElement,
                          alt: &'static str|
     -> Html {
        html! {
            <div key={key.unwrap_or(&img_element.img_src.clone())} class={"footer-img-container"}>
                <a href={img_element.href.clone()} target="_blank">
                    <img
                        class="footer-img"
                        src={img_element.img_src.clone()}
                        alt={alt}
                    />
                </a>
            </div>
        }
    };

    html! {
        <footer>
            <h2>
                { "By " }
                <a href="https://gitlab.com/1kill2steal" target="_blank">
                    { "1Kill2Steal" }
                </a>
            </h2>
            <div class="footer-images">
                {set_footer_img(
                        Some(&twitter.img_src.clone()),
                        &twitter,
                        "twitter",
                )}
                {set_footer_img(
                        Some(&github.img_src.clone()),
                        &github,
                        "github",
                )}
                {set_footer_img(
                        Some(&gitlab.img_src.clone()),
                        &gitlab,
                        "gitlab",
                )}
                {set_footer_img(
                        Some(&codeberg.img_src.clone()),
                        &codeberg,
                        "codeberg",
                )}
                {set_footer_img(
                        Some(&discord.img_src.clone()),
                        &discord,
                        "discord",
                )}
            </div>
            <p>
                {"Hosted on "}
                <a href="https://www.netlify.com/" target="_blank">
                {"Netflify"}
                </a>
                {" & Code is free to use (MIT license) on "}
                <a href="https://www.gitlab.com/1k2s/yew-site" target="_blank">
                {"GitLab"}
                </a>
            </p>
        </footer>
    }
}
