use super::*;

pub mod contents;
pub mod projects_nav;
pub mod utils;

// Re-exports for convenience sake.
use {contents::*, utils::*};

#[function_component(ProjectSerenityDiscordBot)]
pub fn serenity_discord_bot() -> Html {
    wrap_project_subsite(serenity_discord_bot_contents())
}

#[function_component(ProjectDiscordInteractionsBot)]
pub fn discord_interactions_bot() -> Html {
    wrap_project_subsite(discord_interactions_bot_contents())
}

#[function_component(ProjectCountingBlinks)]
pub fn counting_blinks() -> Html {
    wrap_project_subsite(counting_blinks_contents())
}

#[function_component(ProjectLeetCodeTrees)]
pub fn leetcode_trees() -> Html {
    wrap_project_subsite(leetcode_trees_contents())
}
