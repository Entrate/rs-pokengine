//! Plays one battle with both sides using Showdown's default choice, then prints the log.
//!
//! Run with `cargo run --release --example battle`.
use engine::{Battle, ids::SideId, log::TextLog, state::choices::RequestKind};

const P1: &str = "Volbeat||SitrusBerry|Prankster|tailwind,thunderwave,encore,lunge||85,85,85,85,85,85|M|||82|,,,,,Water]Rampardos||ChoiceScarf|SheerForce|rockslide,firepunch,headsmash,stompingtantrum||85,85,85,85,85,85|M|||87|,,,,,Rock]Regirock||Leftovers|ClearBody|irondefense,bodypress,thunderwave,stoneedge||85,85,85,85,85,85|N|||82|,,,,,Fighting]Medicham||LifeOrb|PurePower|zenheadbutt,protect,bulletpunch,closecombat||85,85,85,85,85,85|M|||86|,,,,,Fire]Spiritomb||SitrusBerry|Infiltrator|snarl,trickroom,willowisp,foulplay||85,,85,85,85,|F|,0,,,,0||89|,,,,,Steel]Chimecho||SitrusBerry|Levitate|psychic,helpinghand,icywind,protect||85,,85,85,85,85|M|,0,,,,||94|,,,,,Steel";
const P2: &str = "Deoxys||LifeOrb|Pressure|protect,psychoboost,extremespeed,superpower||85,85,85,85,85,85|N|||79|,,,,,Ghost]Honchkrow||LifeOrb|Moxie|bravebird,heatwave,suckerpunch,protect||85,85,85,85,85,85|F|||85|,,,,,Dark]Zangoose||ToxicOrb|ToxicBoost|protect,closecombat,facade,knockoff||85,85,85,85,85,85|M|||87|,,,,,Normal]Meowstic|MeowsticF|LifeOrb|Competitive|protect,psychic,alluringvoice,darkpulse||85,,85,85,85,85|F|,0,,,,||89|,,,,,Dark]Klawf||ChoiceScarf|Regenerator|crabhammer,rockslide,highhorsepower,knockoff||85,85,85,85,85,85|M|||91|,,,,,Ground]Flygon||LoadedDice|Levitate|scaleshot,earthpower,tailwind,protect||85,85,85,85,85,85|M|||83|,,,,,Steel";

fn main() {
    let mut battle = Battle::with_log([1, 2, 3, 4], P1, P2, TextLog::default()).expect("valid teams");
    battle.start().expect("battle starts");

    while battle.outcome().is_none() {
        for side in 0..2 {
            if matches!(battle.side_request_kind(SideId(side as u8)), RequestKind::Move | RequestKind::Switch) {
                // Any Showdown choice string works here, e.g. "move 1 1, move 2 2" or "switch 3".
                battle.choose(side, "default").expect("legal choice");
            }
        }
    }

    for line in &battle.log.entries {
        println!("{line}");
    }
    let outcome = battle.outcome().unwrap();
    println!("winner: {:?} after {} turns", outcome.winner, outcome.turns);
}
