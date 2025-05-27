use anyhow::Result;
use squard_connect::client::squard_connect::SquardConnect;
use sui_squad_core::{ai::ResponsesClient, commands::bot_commands::{Command, LoginState}};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message, utils::command::BotCommands, Bot};

use super::handlers::{handle_get_balance, handle_get_wallet_address, handle_prompt};

pub async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    responses_client: ResponsesClient,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::GetWallet => handle_get_wallet_address(bot, msg, dialogue, squard_connect_client).await?,
        Command::GetBalance(_token) => handle_get_balance(bot, msg, dialogue, squard_connect_client).await?,
        Command::Prompt(prompt_text) => handle_prompt(bot, msg, prompt_text, responses_client).await?,
        Command::PromptExamples => bot.send_message(msg.chat.id, "Dummy response: Examples:\n- /prompt What is the weather?\n- /prompt Summarize this article...").await?,
    };
    Ok(())
}
