use super::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let project_item = |tag: &'static str, target: Route, text: &'static str| -> Html {
        html! {
            <div tag={tag} class={"project-item"}>
                <p>
                    <Link<Route> to={target}>
                        {text}
                    </Link<Route>>
                </p>
            </div>
        }
    };
    wrap_site(html! {
        <>
            <h1>{ "Projects" }</h1>
            <div tag={"projects-list"} class={"projects-list"}>
                {project_item("serenity_discord_bot", Route::ProjectSerenityDiscordBot, "Serenity discord bot")}
                {project_item("discord_interactions_bot", Route::ProjectDiscordInteractionsBot, "Discord Interactions Bot")}
                {project_item("counting_blinks", Route::ProjectCountingBlinks, "Counting Blinks")}
                {project_item("leetcode_trees", Route::ProjectLeetCodeTrees, "LeetCode Trees")}
            </div>
        </>
    })
}
