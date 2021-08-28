extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::client::Client;
use serenity::model::channel::Message;

use std::io::prelude::*;
use std::fs::File;
struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {

  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "+test" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Working...").await {
        println!("Error: {:?} ",why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is ready",ready.user.name)
  }
}

#[tokio::main]
async fn main() {

  let mut file = File::open(".token").expect( "Error reading the file");
  let mut token = String::new();
  file.read_to_string(&mut token).expect("Token file not found");

  let mut client = Client::builder(&token).event_handler(Handler).await.expect("Error Creating client");

  if let Err(why) = client.start().await {
    println!("Error: {:?}", why);
  }
}