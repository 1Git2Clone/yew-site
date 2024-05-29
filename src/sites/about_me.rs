use super::*;

#[function_component(AboutMe)]
pub fn details() -> Html {
    wrap_site(html! {
        <>
            <h1>{ "My name is Ivan" }</h1>

            <h2>{"Project lore"}</h2>

            <p>
                {"I'm mainly a back-end developer but with "}
                <a href="https://gitlab.com/1k2s/yew-site" target="_blank">
                    {"this project"}
                </a>
                {" I decided to challenge myself and try out something new."}

                <br />

                {"Let's just say... It was a bit of a pain to get this working to a
                somewhat decent level. Not to mention that a lot of functionality like
                closing the gallery preview images with esc and/or clicking outside
                of the image isn't implemented because I'm not exactly sure how to
                do it. Anyway."}

                <br />
            </p>

            <h2>{"Hobbies"}</h2>

            <ul class={"hobbies"}>
                <li>
                {"Linux (imagine having an OS as a hobby)"}
                </li>
                <li>
                {"Programming (and I mainly do it in Rust)"}
                </li>
                <li>
                {"Anime (I don't watch it a lot but I like it)"}
                </li>
                <li>
                {"Sleeping- okay that's not a hobby-"}
                </li>
            </ul>

            <h2>{"Other projects"}</h2>
            <ul class={"other-projects"}>
                <li>
                    <a href="https://gitlab.com/1k2s/serenity-discord-bot"
                        target="_blank"
                    >
                        {"Discord bot"}
                    </a>
                    {" "}
                    <img src="/public/skill-icons/Rust.svg" />
                </li>
                <li>
                    <a href="https://gitlab.com/1k2s/discord-interactions-bot"
                        target="_blank"
                    >
                        {"Discord bot"}
                    </a>
                    {" "}
                    <img src="/public/skill-icons/TypeScript.svg" />
                </li>
                <li>
                    <a href="https://gitlab.com/1k2s/leetcode-trees-rs"
                        target="_blank"
                    >
                        {"LeetCode mini library"}
                    </a>
                    {" "}
                    <img src="/public/skill-icons/Rust.svg" />
                </li>
                <li>
                    <a href="https://gitlab.com/1k2s/hunger-games-website"
                        target="_blank"
                    >
                        {"Hunger Games Website"}
                    </a>
                    {" "}
                    <img src="/public/skill-icons/HTML.svg" />
                    {" "}
                    <img src="/public/skill-icons/CSS.svg" />
                    {" "}
                    <img src="/public/skill-icons/TypeScript.svg" />
                    <br />
                    <br />
                    {"- hosted on "}
                    <a href="https://1k2s.gitlab.io/hunger-games-website/"
                        target="_blank"
                    >
                        {"GitLab Pages"}
                    </a>
                </li>
                <li>
                    {"More on my "}
                    <a href="https://gitlab.com/1Kill2Steal" target="_blank">
                        {"GitLab"}
                    </a>
                    {" and "}
                    <a href="https://codeberg.org/1Kill2Steal" target="_blank">
                        {"Codeberg"}
                    </a>
                    {" profiles."}
                </li>
            </ul>
        </>
    })
}
