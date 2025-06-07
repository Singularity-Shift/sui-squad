use anyhow::Result;
use sui_squad_core::commands::bot_commands::{Command, LoginState};
use teloxide::{
    Bot,
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt, dialogue::InMemStorage},
    dptree::{self, Handler},
    prelude::{DependencyMap, Requester},
    types::{Message, Update},
};

use crate::middleware::{auth::auth, user::check_user};

use super::answer::answer;

async fn handle_unauthenticated(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        "👋 Welcome to SUI Squad! 

To use commands like `/p` or `/prompt`, you need to authenticate first.

Please use `/login` to authenticate, or `/fund` to set up your account with Google.",
    )
    .await?;
    Ok(())
}

pub fn handler_tree() -> Handler<'static, DependencyMap, Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<LoginState>, LoginState>()
        .branch(
            // 1. Branch for authenticated users
            dptree::entry()
                .filter_async(auth)
                .filter_async(check_user)
                .filter_command::<Command>()
                .endpoint(answer),
        )
        .branch(
            // 2. Branch for public commands for new users
            dptree::entry()
                .filter_command::<Command>()
                .filter(|cmd: Command| {
                    matches!(cmd, Command::Login | Command::Fund | Command::Help)
                })
                .endpoint(answer),
        )
        .branch(
            // 3. Fallback for unauthenticated users trying protected commands
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_unauthenticated),
        )
}
