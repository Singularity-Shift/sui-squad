use anyhow::Result;
use sui_squad_core::commands::bot_commands::{Command, LoginState};
use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::{Message, Update},
};

use crate::middleware::auth::auth;

use super::{answer::answer, handlers::handle_login};

pub fn handler_tree() -> Handler<'static, DependencyMap, Result<()>, DpHandlerDescription> {
    Update::filter_message().enter_dialogue::<Message, InMemStorage<LoginState>, LoginState>()
        .branch(dptree::entry().filter_async(auth).filter_command::<Command>().endpoint(answer))
        .branch(
            dptree::entry()
                .branch(dptree::case![LoginState::Login].endpoint(handle_login)),
        )
}

