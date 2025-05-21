use anyhow::Result;
use sui_squad_core::{ai::ResponsesClient, commands::bot_commands::Command};
use teloxide::{Bot, prelude::*, types::Message, utils::command::BotCommands};

use super::handlers::handle_prompt;

pub async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    responses_client: ResponsesClient,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::GetWallet => bot.send_message(msg.chat.id, "Your wallet address is 0x123...").await?,
        Command::GetBalance(token) => {
            if token.trim().is_empty() {
                bot.send_message(msg.chat.id, "You have 100 SUI, 50 USDT.").await?
            } else {
                bot.send_message(msg.chat.id, format!("Your balance for {} is 100.", token)).await?
            }
        },
        Command::Prompt(prompt_text) => handle_prompt(bot, msg, prompt_text, responses_client).await?,
        Command::PromptExamples => bot.send_message(msg.chat.id, "Dummy response: Examples:\n- /prompt What is the weather?\n- /prompt Summarize this article...").await?,
    };
    Ok(())
}
