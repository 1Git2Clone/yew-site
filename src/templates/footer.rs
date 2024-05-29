use super::*;

use crate::data::footer::*;
use crate::enums::Socials::*;
use crate::structs::ImageElement;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footer_img_container = || String::from("footer-img-container");

    let twitter = SOCIALS_IMAGES.get(&Twitter).unwrap().clone();
    let gitlab = SOCIALS_IMAGES.get(&GitLab).unwrap().clone();
    let codeberg = SOCIALS_IMAGES.get(&Codeberg).unwrap().clone();
    let discord = SOCIALS_IMAGES.get(&Discord).unwrap().clone();

    let set_footer_img = |img_element: ImageElement, class: String, key: Option<String>| -> Html {
        html! {
            <div key={key.unwrap_or(img_element.img_src.clone())} class={class}>
                <a href={img_element.clone().href()} target="_blank">
                    <img class="footer-img"
                        src={img_element.img_src()}
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
                        twitter.clone(),
                        footer_img_container(),
                        Some(twitter.img_src())
                )}
                {set_footer_img(
                        gitlab.clone(),
                        footer_img_container(),
                        Some(gitlab.img_src())
                )}
                {set_footer_img(
                        codeberg.clone(),
                        footer_img_container(),
                        Some(codeberg.img_src())
                )}
                {set_footer_img(
                        discord.clone(),
                        footer_img_container(),
                        Some(discord.img_src())
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
