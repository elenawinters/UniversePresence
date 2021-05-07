// extern crate lazy_static;
use rustcord::{Rustcord, RichPresenceBuilder, RichPresence, EventHandlers};

use lazy_static;
// use discord_rpc_client::Client;
use std::{ffi::NulError, str::FromStr, thread, time::Duration, sync::{Mutex, RwLock}};
use crate::zone::{get_region, get_localization};

pub struct Handlers; // we don't use this right now
impl EventHandlers for Handlers {}

// const APP_ID: u64 = 715320038428639313;
const APP_ID: &str = "679062437076402178"; // this is Wincent's app id. too lazy to set up everything myself

// let mut discord: Client::new(715320038428639313);
// lazy_static! {
//     static ref discord: Mutex<Client> = Client::new(APP_ID);
// }

lazy_static! {
    pub static ref CLIENT: Result<Rustcord, NulError> = Rustcord::init::<Handlers>(APP_ID, true, None); // I can't get this to work with the "?"
    pub static ref REGION: RwLock<String> = RwLock::new(get_region());
    pub static ref DETAILS: RwLock<String> = RwLock::new(get_localization(&""));
    // pub static ref STATE: String = String::from("");
    // pub static ref STATE: String = String::from("");
    // pub static ref STATE: String = String::from("");
    // pub static ref STATE: String = String::from("");
    // pub static ref STATE: String = String::from("");

}

pub fn run_discord_rpc() { // roundabout way to get the rustcord obj
    // let set_region = REGION.write().unwrap();
    // *set_region = get_region();
    // let test = REGION.lock().unwrap()

    info!("Building presence");
    let presence = RichPresenceBuilder::new()
        .details(
            &format!("Exploring {}", get_localization("ZoneTable_0_DisplayDescription"))
        )
        .state("Selecting character")
        .large_image_key("select")
        .large_image_text("Character select")
        .build();

    info!("We built the presence");
    update_presence(presence);

    thread::spawn(|| {
        loop {
            run_callbacks();
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

pub fn update_presence(presence: RichPresence) {
    &CLIENT.as_ref().unwrap().update_presence(presence);
}

pub fn run_callbacks() {
    &CLIENT.as_ref().unwrap().run_callbacks();
}
