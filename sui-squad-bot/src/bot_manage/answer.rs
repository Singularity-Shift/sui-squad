use anyhow::Result;
use grammers_client::{types::Message, InputMessage};
use sui_squad_core::commands::bot_commands::Command;

use crate::bot_manage::{
    handlers::{handle_fund, handle_prompt},
    handler_tree::BotContext
};

pub async fn answer(
    ctx: &BotContext,
    msg: &Message,
    cmd: Command,
) -> Result<()> {
    let chat = msg.chat();
    
    match cmd {
        Command::Help => {
            let help_text = get_help_text();
            ctx.client
                .send_message(chat, InputMessage::text(help_text))
                .await?;
        },
        Command::Fund => {
            handle_fund(ctx.clone(), msg.clone()).await?;
        },
        Command::Prompt(prompt_text) => {
            handle_prompt(ctx.clone(), msg.clone(), prompt_text).await?;
        },
        Command::P(prompt_text) => {
            handle_prompt(ctx.clone(), msg.clone(), prompt_text).await?; 
        },
        Command::PromptExamples => {
            let examples_text = get_prompt_examples_text();
            ctx.client
                .send_message(chat, InputMessage::text(examples_text))
                .await?;
        },
    };
    Ok(())
}

pub fn get_help_text() -> &'static str {
    "These commands are supported:

/prompt <text> - Send a prompt to the AI assistant
/p <text> - Send a prompt to the AI assistant (short alias for /prompt)  
/promptexamples - Show Squard prompt examples
/help - Display this help message
/fund - Fund your account"
}

pub fn get_prompt_examples_text() -> &'static str {
    "Here are some example prompts you can use:

💰 Wallet & Balance:
- /prompt \"What's my wallet address?\" or /p \"What's my wallet address?\"
- /prompt \"Show my balance\" or /p \"Show my balance\"
- /prompt \"Check my SUI balance\" or /p \"Check my SUI balance\"
- /prompt \"How much do I have?\" or /p \"How much do I have?\"

💸 Transactions:
- /prompt \"Send 10 SUI to @username\" or /p \"Send 10 SUI to @username\"
- /prompt \"Withdraw 5 SUI\" or /p \"Withdraw 5 SUI\"
- /prompt \"Send 100 SUI to everyone\" or /p \"Send 100 SUI to everyone\"

❓ General:
- /prompt \"What can you help me with?\" or /p \"What can you help me with?\"
- /prompt \"Explain how this bot works\" or /p \"Explain how this bot works\"

💡 Tip: Use /p as a shortcut for /prompt!"
}
