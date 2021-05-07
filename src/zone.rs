use std::{fs::read_to_string, io::Read, str::from_utf8};

use crate::discord::REGION;

// const CLIENT_REGION: usize = 0x0 + 1;
// const CLIENT_LOCALE: usize = 0x0 + 1;

// DisplayDescription locations: 015a30e0, 015a3108, 015b07ec
// 00c8bbfb for context. Ghidra addresses for all

// decided to just write my own code to do the thing. Seems easier. If I wanted to get other data I'd have to call, but the zoneID is easy to get
pub fn get_zone_name(zoneid: u16) -> String {
    return String::from("test");
}

pub fn get_region() -> String {  // this code is borrowed from ui_scaler mod by lcdr
    // info!("this is being logged from get_region!");
    let lnv = read_to_string("boot.cfg").expect("could not open boot.cfg");

    for name_value in lnv.split(",") {
		let name_value = name_value.trim();
		let (name, value) = name_value.split_at(name_value.find(":").expect(&format!("malformed boot.cfg entry, colon missing: {}", name_value))+1);

		if name == "LOCALE=0:" {
			let region: String = value.parse().expect(&format!("could not parse boot.cfg LOCALE value as float: {}", value));
			return region;
		}
	}

    return String::from("en_US");  // default
}

// impl From<io::Error> for LocalizationError {
//     fn from(e: io::Error) -> LocalizationError {
        
//     }
// }

pub fn get_localization(item: &str) -> String {  // this error handling is cursed
    let lnv = read_to_string("locale/locale.xml").expect("could not open locale.xml");
    let doc = roxmltree::Document::parse(&lnv).unwrap();

    let elem = doc.descendants().find(|n| n.attribute("id") == Some(item)).unwrap();
    let loc = elem.children().find(|n| n.attribute("locale") == Some(&REGION.read().unwrap())).unwrap();

    // let elem = doc.descendants().find(|n| n.attribute("id") == Some(item)).unwrap();
    // let loc = elem.children().find(|n| n.attribute("locale") == Some(&REGION.read().unwrap())).unwrap();
    return String::from(loc.text().unwrap());

    info!("{:?}", loc.text());
    info!("{}", loc.text().unwrap());
    // for x in elem.children().find() {
    //     info!("{}", x.attribute(name))
    // }
    // info!("{}", elem.children());
    // assert!(elem.has_tag_name("rect"));
    

    return String::from(item);
}


fn create_localization(item: &str) -> String { // this will add any needed entries to locale.xml. they are only relevant to this mod
    return String::from("Venture Explorer");
}

// pub enum ZoneID { // this should be u16
//     VentureExplorerCinematic,
//     VentureExplorer = 1000,
//     ReturnToVentureExplorer,
//     AvantGardens = 1100,
//     AvantGardensSurvival,
//     SpiderQueenBattle,
//     BlockYard = 1150,
//     AvantGrove,
//     NimbusStation = 1200,
//     PetCove,
//     VertigoLoopRacetrack = 1203,
//     BattleOfNimbusStation,
//     NimbusRock = 1250,
//     NimbusIsle,
//     FrostBurgh = 1260,
//     GnarledForest = 1300,
//     CanyonCove = 1302,
//     KeelhaulCanyon,
//     ChanteyShantey = 1350,
//     ForbiddenValley = 1400,
//     ForbiddenValleyDragon = 1402,
//     DragonmawChasm,
//     RavenBluff = 1450,
//     Starbase3001 = 1600,
//     DeepFreeze,
//     RobotCity,
//     MoonBase,
//     Portabello = 1604,
//     LegoClub = 1700,
//     CruxPrime = 1800,
//     NexusTower = 1900,
//     Ninjago = 2000,
//     FrakjawBattle,
//     NimbusStationWinterRacetrack = 1261,
//     DefaultTest = 58001
// }