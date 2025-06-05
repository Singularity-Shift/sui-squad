use anyhow::Result;
use sui_squad_core::commands::bot_commands::{Command, LoginState};
use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt, dialogue::InMemStorage},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::{Message, Update},
};

use crate::middleware::{auth::auth, user::check_user};

use super::answer::answer;

pub fn handler_tree() -> Handler<'static, DependencyMap, Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<LoginState>, LoginState>()
        .branch(
            dptree::entry()
                .filter_async(auth)
                .filter_async(check_user)
                .filter_command::<Command>()
                .endpoint(answer),
        )
}
