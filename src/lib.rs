use cobapi::{
    Event,
    SystemEvent,
};

mod structs_types;
use structs_types::*;

use std::sync::OnceLock;
use engage_il2cpp::GameVariableManager;
use engage_il2cpp::app::persondata::{IPersonDataMethods, PersonData};
use engage_il2cpp::app::jobdata::{IJobDataMethods, JobData};
use engage_il2cpp::app::structdata_1::IStructData_1Methods;
use engage_il2cpp::{List_1Ext};
use engage_il2cpp::app::capabilitybase_1::ICapabilityBase_1Methods;

static BACKUP_PERSON_DATA_STATS: OnceLock<PersonDataStats> = OnceLock::new();

extern "C" fn my_system_event_listener(event: &Event<SystemEvent>) 
{
    if let Event::Args(ev) = event 
    {
        match ev 
        {
            SystemEvent::GamedataLoaded => check_and_validate_person_data(),
            SystemEvent::SaveLoaded { ty, slot_id } => on_save_data_loaded(ty, slot_id),
            // This syntax means you do not intend to deal with the other events and will do nothing if they are received.
            _ => (),
        }
    }
}

fn print_stat_block(label: &str, s: &PersonDataStats)
{
    let lbl = ["HP", "Str", "Mag", "Skl", "Spd", "Lck", "Def", "Res", "Phys"];
    println!("--- {} ---", label);
    println!("  Bases  : {}", (0..9).map(|i| format!("{}={}", lbl[i], s.bases[i])).collect::<Vec<_>>().join(", ") );
    println!("  Caps   : {}", (0..9).map(|i| format!("{}={}", lbl[i], s.caps[i])).collect::<Vec<_>>().join(", ") );
    println!("  Growths: {}", (0..9).map(|i| format!("{}={}", lbl[i], s.growths[i])).collect::<Vec<_>>().join(", ") );
}

pub fn on_save_data_loaded( save_type: &i32, slot_id: &i32 )
{
    if *save_type <= 1  { return; } // Only care about actual save files being loaded

    println!("Save type #{} being loaded from slot #{}", save_type, slot_id);
    let boon_type = return_as_boon_bane_type(GameVariableManager::get_number("alear_boon_type"));
    let bane_type = return_as_boon_bane_type(GameVariableManager::get_number("alear_bane_type"));

    println!("Current boon type is {:?} and bane type is {:?}", boon_type, bane_type);

    let backup = BACKUP_PERSON_DATA_STATS.get().unwrap();
    let boon = get_boon_stats_by_type(boon_type);
    let bane = get_bane_stats_by_type(bane_type);

    print_stat_block("BACKUP", backup);
    print_stat_block("BOON",   boon);
    print_stat_block("BANE",   bane);

    let combined = combine_stats(boon, backup, bane);
    print_stat_block("COMBINED (boon + backup - bane)", &combined);

    if let Some(person) = PersonData::get_list().iter().find(|p| p.is_hero())
    {
        let bases   = person.get_offset_l();
        let caps    = person.get_limit();
        let growths = person.get_grow();
    
        bases.set_hp(combined.bases[0]);
        bases.set_str(combined.bases[1]);
        bases.set_magic(combined.bases[2]);
        bases.set_tech(combined.bases[3]);
        bases.set_quick(combined.bases[4]);
        bases.set_luck(combined.bases[5]);
        bases.set_def(combined.bases[6]);
        bases.set_mdef(combined.bases[7]);
        bases.set_phys(combined.bases[8]);
    
        caps.set_hp(combined.caps[0]);
        caps.set_str(combined.caps[1]);
        caps.set_magic(combined.caps[2]);
        caps.set_tech(combined.caps[3]);
        caps.set_quick(combined.caps[4]);
        caps.set_luck(combined.caps[5]);
        caps.set_def(combined.caps[6]);
        caps.set_mdef(combined.caps[7]);
        caps.set_phys(combined.caps[8]);
    
        growths.set_hp(combined.growths[0]);
        growths.set_str(combined.growths[1]);
        growths.set_magic(combined.growths[2]);
        growths.set_tech(combined.growths[3]);
        growths.set_quick(combined.growths[4]);
        growths.set_luck(combined.growths[5]);
        growths.set_def(combined.growths[6]);
        growths.set_mdef(combined.growths[7]);
        growths.set_phys(combined.growths[8]);
    }
}

pub fn check_and_validate_person_data() 
{
    println!("=================\nChecking PersonData...\n=================");
    
    if let Some(person) = PersonData::get_list().iter().find(|person| person.is_hero()) 
    {
        let pid = person.get_pid();
        let name = person.get_name();

        let growths = person.get_grow();
        let bases = person.get_offset_l();
        let caps = person.get_limit();

        let highest_str_mag_grow = std::cmp::max(growths.get_str(), growths.get_magic());
        let highest_str_mag_base = std::cmp::max(bases.get_str(), bases.get_magic());
        let highest_str_mag_cap = std::cmp::max(caps.get_str(), caps.get_magic());
        
        let stats = PersonDataStats 
        {
            bases: [bases.get_hp(), highest_str_mag_base, highest_str_mag_base, bases.get_tech(), bases.get_quick(), bases.get_luck(), bases.get_def(), bases.get_mdef(), bases.get_phys()],
            caps: [caps.get_hp(), highest_str_mag_cap, highest_str_mag_cap, caps.get_tech(), caps.get_quick(), caps.get_luck(), caps.get_def(), caps.get_mdef(), caps.get_phys()],
            growths: [growths.get_hp(), highest_str_mag_grow, highest_str_mag_grow, growths.get_tech(), growths.get_quick(), growths.get_luck(), growths.get_def(), growths.get_mdef(), growths.get_phys()],
        };
    
        BACKUP_PERSON_DATA_STATS.set(stats).unwrap_or_else(|_| 
        {
            println!("Stats already populated!");
        });

        println!("PersonData entry - PID: {}, Name: {}", pid, name);
    }

    // Divine Dragon.into()
    for job in JobData::get_list().iter().filter(|job| 
    {
        let jid = job.get_jid().to_rust_string();
        jid == "JID_神竜ノ子" 
        || jid == "JID_神竜ノ王" 
        || jid == "JID_M000_神竜ノ子"
    }) 
    {
        let jid = job.get_jid();
        let name = job.get_name();

        let growths = job.get_diff_grow();
        let bases = job.get_base();
        let caps = job.get_limit();

        let highest_str_mag_grow = std::cmp::max(growths.get_str(), growths.get_magic());
        growths.set_str(highest_str_mag_grow);
        growths.set_magic(highest_str_mag_grow);

        let highest_str_mag_base = std::cmp::max(bases.get_str(), bases.get_magic());
        bases.set_str(highest_str_mag_base);
        bases.set_magic(highest_str_mag_base);

        let highest_str_mag_cap = std::cmp::max(caps.get_str(), caps.get_magic());
        caps.set_str(highest_str_mag_cap);
        caps.set_magic(highest_str_mag_cap);

        println!("JobData entry - JID: {}, Name: {}", jid, name);
    }
}


#[skyline::main(name = "AlearBoonBane")]
pub fn main() {
    println!("Hello from skyline plugin AlearBoonBane");
    cobapi::register_system_event_handler(my_system_event_listener);
}
