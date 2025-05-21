use anyhow::Result;
use sui_squad_core::commands::bot_commands::{Command, LoginState};
use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt, dialogue::InMemStorage},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::{Message, Update},
};

use super::{answer::answer, handlers::handle_login};

pub fn handler_tree() -> Handler<'static, DependencyMap, Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
        .branch(
            dptree::entry()
                .enter_dialogue::<Message, InMemStorage<LoginState>, LoginState>()
                .branch(dptree::case![LoginState::Login].endpoint(handle_login)),
        )
}
