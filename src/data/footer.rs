use crate::enums::Socials;
use crate::structs::ImageElement;

const DIRECTORY: &str = "/public/socials/";
const EXTENSION: &str = ".svg";

fn set_src(img_name: &'static str) -> String {
    format!("{DIRECTORY}{img_name}{EXTENSION}")
}

pub fn get_socials_images(e: Socials) -> ImageElement {
    use Socials as s;

    match e {
        s::Twitter => ImageElement::from(
            set_src("twitter"),
            String::from("https://twitter.com/1Kill2Steal"),
        ),
        s::GitHub => ImageElement::from(
            set_src("github"),
            String::from("https://github.com/1Git2Clone"),
        ),
        s::GitLab => ImageElement::from(
            set_src("gitlab"),
            String::from("https://gitlab.com/1Kill2Steal"),
        ),
        s::Codeberg => ImageElement::from(
            set_src("codeberg"),
            String::from("https://codeberg.org/1Kill2Steal"),
        ),
        s::Discord => ImageElement::from(
            set_src("discord"),
            String::from("https://discord.gg/d8eyqpK2PN"),
        ),
    }
}
