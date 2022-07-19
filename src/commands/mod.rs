use serenity::framework::standard::macros::group;

mod ping;
use ping::*;
mod kick;
use kick::*;


#[group]
#[commands(ping)]
pub struct General;



#[group]
#[commands(kick)]
pub struct Owners;