use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Homepage,
    #[at("/about-me")]
    AboutMe,

    #[at("/blog")]
    Blog,
    #[at("/blog/understanding-big-o-notation")]
    BlogUnderstandingBigONotation,

    #[at("/projects")]
    Projects,
    #[at("/projects/serenity-discord-bot")]
    ProjectSerenityDiscordBot,
    #[at("/projects/discord-interactions-bot")]
    ProjectDiscordInteractionsBot,
    #[at("/projects/counting-blinks")]
    ProjectCountingBlinks,
    #[at("/projects/leetcode-trees-rs")]
    ProjectLeetCodeTrees,
    #[at("/projects/reactix")]
    ProjectReactix,
    #[at("/projects/currency-conversion")]
    ProjectCurrencyConversion,

    #[at("/gallery")]
    GalleryBase,
    #[at("/gallery/:page")]
    Gallery { page: u32 },

    #[not_found]
    #[at("/404")]
    NotFound,
}
