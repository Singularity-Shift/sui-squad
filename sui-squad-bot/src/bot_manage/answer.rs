use anyhow::Result;
use squard_connect::client::squard_connect::SquardConnect;
use sui_squad_core::{
    ai::ResponsesClient, 
    commands::bot_commands::{Command, LoginState},
    conversation::ConversationCache
};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message, utils::command::BotCommands, Bot};

use super::handlers::{handle_prompt};

pub async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    responses_client: ResponsesClient,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    conversation_cache: ConversationCache,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Prompt(prompt_text) => handle_prompt(
            bot, 
            msg, 
            prompt_text, 
            responses_client, 
            dialogue, 
            squard_connect_client,
            conversation_cache
        ).await?,
        Command::P(prompt_text) => handle_prompt(
            bot, 
            msg, 
            prompt_text, 
            responses_client, 
            dialogue, 
            squard_connect_client,
            conversation_cache
        ).await?,
        Command::PromptExamples => bot.send_message(msg.chat.id, "Here are some example prompts you can use:\n\n💰 Wallet & Balance:\n- /prompt \"What's my wallet address?\" or /p \"What's my wallet address?\"\n- /prompt \"Show my balance\" or /p \"Show my balance\"\n- /prompt \"Check my SUI balance\" or /p \"Check my SUI balance\"\n- /prompt \"How much do I have?\" or /p \"How much do I have?\"\n\n💸 Transactions:\n- /prompt \"Send 10 SUI to @username\" or /p \"Send 10 SUI to @username\"\n- /prompt \"Withdraw 5 SUI\" or /p \"Withdraw 5 SUI\"\n- /prompt \"Send 100 SUI to everyone\" or /p \"Send 100 SUI to everyone\"\n\n❓ General:\n- /prompt \"What can you help me with?\" or /p \"What can you help me with?\"\n- /prompt \"Explain how this bot works\" or /p \"Explain how this bot works\"\n\n💡 Tip: Use /p as a shortcut for /prompt!").await?,
    };
    Ok(())
}
