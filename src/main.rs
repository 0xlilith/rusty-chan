extern crate serenity;

use base64::{encode, decode};
use std::fmt::Write;  

use std::collections::HashSet;
use std::collections::HashMap;

use serenity::client::{Client, Context, EventHandler};
use serenity::{
  async_trait,
  framework::standard::{
      help_commands,
      macros::{command, group, help,},
      Args,
      CommandGroup,
      CommandResult,
      HelpOptions,
      StandardFramework,
  },
  model::{
      channel::{Message},
      gateway::Ready,
      id::UserId,
  },
  
};


use std::io::prelude::*;
use std::fs::File;

#[help]
#[individual_command_tip = "✨**WELCOME TO THE HELP MENU OF RUSTY CHAN**✨\n
 **BOT PREFIX** - `+` "]
#[strikethrough_commands_tip_in_guild = " "]
#[command_not_found_text = "INVALID COMMAND NAME : `{}`"]
async fn help(
  context: &Context,
  msg: &Message,
  args: Args,
  help_options: &'static HelpOptions,
  groups: &[&'static CommandGroup],
  owners: HashSet<UserId>,
) -> CommandResult {
  let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
  Ok(())
}

#[group]
#[description = "**GENERAL COMMANDS FOR THE BOT**"]
#[commands(cat, test, db64, eb64, ip)]
struct GeneralCommands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is ready",ready.user.name)
  }
}

#[tokio::main]
async fn main() {
  let framework = StandardFramework::new()
      .configure(|c| c.prefix("+")) 
      .help(&HELP)
      .group(&GENERALCOMMANDS_GROUP);

  let mut file = File::open(".token").expect( "Error reading the file");
  let mut token = String::new();
  file.read_to_string(&mut token).expect("Token file not found");

  let mut client = Client::builder(token)
      .event_handler(Handler)
      .framework(framework)
      .await
      .expect("Error creating client");

  // start listening for events by starting a single shard
  if let Err(why) = client.start().await {
      println!("An error occurred while running the client: {:?}", why);
  }
}

#[command]
#[description = "**JUST TO TEST THE BOT**"]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, "Working...").await?;
  
  Ok(())
}

#[command]
#[description = "**MAKE THE BOT MEOW**"]
async fn cat(ctx: &Context,  msg: &Message) -> CommandResult {
  msg.reply_ping(&ctx, "`ME>^.^<OW`").await?;

  Ok(())
}

#[command]
#[description = "**DECODE BASE64 | usage: +db64 <string>**"]
async fn db64(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let store = args.rest();
  let byte = match decode(store) {
    Ok(byte) => byte,
    Err(err) => {
      eprintln!("Failed to decode user's arguments: {}", err);
      return Ok(());
    },
  };
 
  let string = std::str::from_utf8(&byte)?;
  let newstring = &string.replace("@everyone", "`fuck you`").replace("@here", "`fuck you`");
  msg.channel_id.say(&ctx.http, newstring).await?;

  Ok(())
}

#[command]
#[description = "**ENCODE BASE64 | usage: +eb64 <string>**"]
async fn eb64(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let store = encode(args.rest());
  msg.channel_id.say(&ctx.http, store).await?;

  Ok(())
}


#[command]
#[description = "**FIND INFO OF ANY IP ADDRESS**"]
async fn ip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let ipaddr = args.rest();
  let link = format!("{}{}{}","http://ipinfo.io/",ipaddr,"/json");

  let res = reqwest::get(link)
        .await?
        .json::<HashMap<String, String>>()
        .await?;


  let content = res.iter().fold(String::new(), |mut acc, (key, value)| {
    let _ = writeln!(&mut acc, "{}: \"{}\"", key, value);
    acc
  });  

  let temp = format!("{}{}{}","```hs\n",content,"```");

  msg.channel_id.say(&ctx.http, temp).await?;
  Ok(())
}
