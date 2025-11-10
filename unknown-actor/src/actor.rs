use crate::message::*;
use kameo::Actor;
use kameo::message::{Context, Message};
use tracing::info;

#[derive(Actor)]
#[actor(name = "UnknownActor")]
pub struct ServerActor;

impl ServerActor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Message<Greet> for ServerActor {
    type Reply = ();

    async fn handle(&mut self, msg: Greet, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        info!("Got greet: {}", msg.0);
    }
}

impl Message<Add> for ServerActor {
    type Reply = u32;

    async fn handle(&mut self, msg: Add, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        msg.0 + msg.1
    }
}

impl Message<Shutdown> for ServerActor {
    type Reply = ();

    async fn handle(&mut self, _: Shutdown, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        ctx.actor_ref().kill();
        ctx.actor_ref().wait_for_shutdown().await;
    }
}

impl Message<String> for ServerActor {
    type Reply = ();

    async fn handle(&mut self, msg: String, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        info!(msg);
    }
}
