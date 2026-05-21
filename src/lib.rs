use cobapi::{
    Event,
    SystemEvent,
};

mod structs_types;
use engage_il2cpp::system::collections::generic::{IList_1Methods, List_1};
use engage_il2cpp::unity_engine::component::IComponentMethods;
use engage_il2cpp::unity_engine::gameobject::IGameObjectMethods;
use engage_il2cpp::unity_engine::transform::ITransformMethods;
use structs_types::*;
use unity2::OptionalMethod;

use engage_il2cpp::app::{BasicMenu_Result, IBasicMenuItem, IBasicMenuMethods, IMainMenuSequence, IMainMenuSequence_LanguageSettingMenuSequence_Menu, IMainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent, IMainMenuSequence_MenuSequenceBase, IMainMenuSequence_MenuSequenceBaseMethods, ISingletonProcInst_1Methods, MainMenuSequence_Label, MainMenuSequence_LanguageSettingMenuSequence_Menu, MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog, MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent, MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent, MainMenuSequence_LanguageSettingMenuSequence_Menu_MessMenuItem, MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, MainMenuSequence_NetworkServiceSelectMenuSequence_Menu, Mess};
use engage_il2cpp::app::pad::Pad;
use engage_il2cpp::app::gamesound::GameSound;
use engage_il2cpp::combat::character::Character;
use engage_il2cpp::tm_pro::tmp_text::ITMP_TextMethods;
use engage_il2cpp::app::procinst::{IProcInst, IProcInstMethods, ProcInst};
use engage_il2cpp::app::procvoidfunction::ProcVoidFunction;
use engage_il2cpp::app::procvoidmethod::ProcVoidMethod;
use unity2::{Cast, FromIlInstance, Il2CppString, IlInstance, SystemObject};

use std::sync::{atomic::{AtomicI32, Ordering}, OnceLock};
use engage_il2cpp::ext::{GameVariableManager, ProcVoidMethodExt};
use engage_il2cpp::app::{jobdata::{IJobDataMethods, JobData}, persondata::{IPersonDataMethods, PersonData}};
use engage_il2cpp::app::structdata_1::IStructData_1Methods;
use engage_il2cpp::{List_1Ext, ProcVoidFunctionExt};
use engage_il2cpp::app::capabilitybase_1::ICapabilityBase_1Methods;
use engage_il2cpp::app::mainmenusequence::MainMenuSequence_NetworkServiceSelectMenuSequence;
use engage_il2cpp::app::proc::Proc;
use engage_il2cpp::app::titlebar::{ITitleBarMethods, TitleBar};
use engage_il2cpp::app::mainmenusequence::MainMenuSequence;
use engage_il2cpp::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence;
use engage_il2cpp::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem;

static BACKUP_PERSON_DATA_STATS: OnceLock<PersonDataStats> = OnceLock::new();
static BOON_TYPE: AtomicI32 = AtomicI32::new(BoonBaneType::Hp as i32);
static BANE_TYPE: AtomicI32 = AtomicI32::new(BoonBaneType::Str as i32);

static MID_BOON_ARRAY: [&str; 8] = [
    "MID_BOON_HP",
    "MID_BOON_STR",
    "MID_BOON_MAG",
    "MID_BOON_SKL",
    "MID_BOON_SPD",
    "MID_BOON_LCK",
    "MID_BOON_DEF",
    "MID_BOON_RES"
];
static MID_BANE_ARRAY: [&str; 8] = [
    "MID_BANE_HP",
    "MID_BANE_STR",
    "MID_BANE_MAG",
    "MID_BANE_SKL",
    "MID_BANE_SPD",
    "MID_BANE_LCK",
    "MID_BANE_DEF",
    "MID_BANE_RES"
];

extern "C" fn my_system_event_listener(event: &Event<SystemEvent>) 
{
    if let Event::Args(ev) = event 
    {
        match ev 
        {
            SystemEvent::GamedataLoaded => check_and_validate_person_data(),
            SystemEvent::SaveLoaded { ty, slot_id } => on_save_data_loaded(ty, slot_id),
            SystemEvent::ProcInstBind { proc, parent: _ } => {
                if (*proc.borrow()).m_hash_code() == -1912552174 {
                    println!("Entering MainMenuSequence");
                }
            }
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

    BOON_TYPE.store(boon_type as i32, Ordering::Relaxed);
    BANE_TYPE.store(bane_type as i32, Ordering::Relaxed);

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

pub extern "C" fn boon_bane_set_title_bar(_proc: ProcInst, _method_info: OptionalMethod) 
{
    let boon_name = Mess::get("MID_BOON_TITLE");
    let bane_name = Mess::get("MID_BANE_TITLE");
    Mess::set_argument_2(0, boon_name.to_string());
    Mess::set_argument_2(1, bane_name.to_string());
    // let title = Mess::get("MID_HERE");
    TitleBar::get_instance().open_header(Mess::get("MID_BOON_BANE_TITLE"), Il2CppString::null(), "KHID_汎用");
}

#[skyline::hook(offset = 0x02285890)]
pub fn App_GameSaveDataUtil__Write(_super: u64, save_type: i32, slot_id: i32, result_header_callback: u64, method_info: OptionalMethod)
{
    if save_type <= 1 { return; } // Only care about actual save files being written

    println!("Save type #{} being written to slot #{}", save_type, slot_id);
    GameVariableManager::set_number("alear_boon_type", BOON_TYPE.load(Ordering::Relaxed));
    GameVariableManager::set_number("alear_bane_type", BANE_TYPE.load(Ordering::Relaxed));

    println!("Current boon type is {:?} and bane type is {:?}", BOON_TYPE.load(Ordering::Relaxed), BANE_TYPE.load(Ordering::Relaxed));

    call_original!(_super, save_type, slot_id, result_header_callback, method_info);
}

#[unity2::callback]
pub extern "C" fn boon_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    Mess::get("MID_BOON_TITLE")
}

#[unity2::callback]
pub extern "C" fn boon_get_param_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    let menu = this.m_menu().try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu>().unwrap();
    
    let boon_index = menu.m_lang_voice_index();
    let boon_old_index = menu.m_lang_voice_index_old();

    Mess::get(*MID_BOON_ARRAY.get(boon_index as usize).unwrap_or(&""))
}

#[unity2::callback]
pub extern "C" fn bane_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    Mess::get("MID_BANE_TITLE")
}

#[unity2::callback]
pub extern "C" fn bane_get_param_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    let menu = this.m_menu().try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu>().unwrap();

    let bane_index = menu.m_lang_mess_index();
    let bane_old_index = menu.m_lang_mess_index_old();

    Mess::get(*MID_BANE_ARRAY.get(bane_index as usize).unwrap_or(&""))
}

#[unity2::callback]
pub extern "C" fn menuitem_a_call(_this: MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    MainMenuSequence::get_instance().set_m_next_sequence(MainMenuSequence_Label::final_confirm());
    BasicMenu_Result::close_decide()
}

#[unity2::callback]
pub extern "C" fn menuitem_b_call(_this: MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu::return_sequence();
    BasicMenu_Result::close_cancel()
}

const BOON_BANE_COUNT: i32 = 8;

#[unity2::callback]
pub extern "C" fn boon_key_call(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    let any_left = Pad::any_left();
    let any_right = Pad::any_right();
    let left_pressed = Pad::is_repeat(any_left);
    let right_pressed = Pad::is_repeat(any_right);

    if !left_pressed && !right_pressed {
        return BasicMenu_Result::pass();
    }

    let menu = this.m_menu()
        .try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu>()
        .unwrap();

    let old_index = menu.m_lang_voice_index();
    let mut new_index = old_index;

    if left_pressed { new_index -= 1; }
    if right_pressed { new_index += 1; }

    if new_index < 0 {
        new_index = BOON_BANE_COUNT - 1;
    } else if new_index >= BOON_BANE_COUNT {
        new_index = 0;
    }

    if new_index == BANE_TYPE.load(Ordering::Relaxed) {
        if left_pressed { new_index -= 1; }
        if right_pressed { new_index += 1; }
        if new_index < 0 {
            new_index = BOON_BANE_COUNT - 1;
        } else if new_index >= BOON_BANE_COUNT {
            new_index = 0;
        }
    }

    menu.set_m_lang_voice_index_old(old_index);
    menu.set_m_lang_voice_index(new_index);
    BOON_TYPE.store(new_index, Ordering::Relaxed);

    let content = this.m_menu_item_content()
        .try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent>()
        .unwrap();

    content.m_name_text().set_text(boon_get_name(this, None));
    content.m_param_text().set_text(boon_get_param_name(this, None));

    if old_index != new_index {
        let null_character = <Character as FromIlInstance>::from_il_instance(IlInstance::null());
        GameSound::post_event("Select", null_character);
    }

    BasicMenu_Result::pass()
}

#[unity2::callback]
pub extern "C" fn bane_key_call(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    let any_left = Pad::any_left();
    let any_right = Pad::any_right();
    let left_pressed = Pad::is_repeat(any_left);
    let right_pressed = Pad::is_repeat(any_right);

    if !left_pressed && !right_pressed {
        return BasicMenu_Result::pass();
    }

    let menu = this.m_menu()
        .try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu>()
        .unwrap();

    let old_index = menu.m_lang_mess_index();
    let mut new_index = old_index;

    if left_pressed { new_index -= 1; }
    if right_pressed { new_index += 1; }

    if new_index < 0 {
        new_index = BOON_BANE_COUNT - 1;
    } else if new_index >= BOON_BANE_COUNT {
        new_index = 0;
    }

    if new_index == BOON_TYPE.load(Ordering::Relaxed) {
        if left_pressed { new_index -= 1; }
        if right_pressed { new_index += 1; }
        if new_index < 0 {
            new_index = BOON_BANE_COUNT - 1;
        } else if new_index >= BOON_BANE_COUNT {
            new_index = 0;
        }
    }

    menu.set_m_lang_mess_index_old(old_index);
    menu.set_m_lang_mess_index(new_index);
    BANE_TYPE.store(new_index, Ordering::Relaxed);

    let content = this.m_menu_item_content()
        .try_cast::<MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItemContent>()
        .unwrap();

    content.m_name_text().set_text(bane_get_name(this, None));
    content.m_param_text().set_text(bane_get_param_name(this, None));

    if old_index != new_index {
        let null_character = <Character as FromIlInstance>::from_il_instance(IlInstance::null());
        GameSound::post_event("Select", null_character);
    }

    BasicMenu_Result::pass()
}

pub extern "C" fn language_create_menu_bind(proc: MainMenuSequence_LanguageSettingMenuSequence, _parent: ProcInst, _method_info: OptionalMethod) {
    let transform = proc.m_layout_prefab().get_transform();
    let menu_transform = transform.find("Menu");
    let menu_go = menu_transform.get_game_object();

    let menu_content = menu_go.get_component::<MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent>();
    let menu_item_list = List_1::new();

    let boon_item = MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem::new();
    let new_class = boon_item.override_class();

    new_class.override_virtual_method("GetName", boon_get_name_method_info());
    new_class.override_virtual_method("GetParamName", boon_get_param_name_method_info());
    new_class.override_virtual_method("ACall", menuitem_a_call_method_info());
    new_class.override_virtual_method("BCall", menuitem_b_call_method_info());
    new_class.override_virtual_method("KeyCall", boon_key_call_method_info());

    let bane_item = MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem::new();
    let new_class = bane_item.override_class();

    new_class.override_virtual_method("GetName", bane_get_name_method_info());
    new_class.override_virtual_method("GetParamName", bane_get_param_name_method_info());
    new_class.override_virtual_method("ACall", menuitem_a_call_method_info());
    new_class.override_virtual_method("BCall", menuitem_b_call_method_info());
    new_class.override_virtual_method("KeyCall", bane_key_call_method_info());

    menu_item_list.add(boon_item.into());
    menu_item_list.add(bane_item.into());

    let menu = MainMenuSequence_LanguageSettingMenuSequence_Menu::new(menu_item_list, menu_content.into());
    menu.set_m_lang_voice_index(BOON_TYPE.load(Ordering::Relaxed));
    menu.set_m_lang_mess_index(BANE_TYPE.load(Ordering::Relaxed));
    let descs = menu.create_default_desc();

    menu.create_bind(proc, descs, Il2CppString::null());
    menu.bind_parent_menu();
    menu.set_show_row_num(2);
}

pub extern "C" fn boon_bane_create_menu_bind(proc: MainMenuSequence_NetworkServiceSelectMenuSequence, _parent: ProcInst, _method_info: OptionalMethod) {
    let language = MainMenuSequence_LanguageSettingMenuSequence::new();

    let descs = language.get_proc_desc();
    descs.set(1, Proc::call_2(ProcVoidMethod::from_fn(proc.as_instance(), boon_bane_set_title_bar).unwrap()));
    descs.set(5, Proc::call(ProcVoidFunction::from_fn(language.as_instance(), language_create_menu_bind).unwrap()));

    language.create_bind(proc, descs, Il2CppString::null());
}

#[unity2::hook("App", "MainMenuSequence.NetworkServiceSelectMenuSequence", "CreateBind")]
pub fn network_service_select_hook(parent: MainMenuSequence, method_info: OptionalMethod) {
    // let root = parent.m_history_info().m_window().get(0).m_root_object();
    // let title = Ut::find_child_game_object(root, "Title");
    // let title_text = title.get_component(SystemType::from_il2cpp_type(TextMeshProUGUI::il_type()).unwrap());
    // let casted_text = title_text.try_cast::<TextMeshProUGUI>().unwrap();
    // casted_text.set_text("New text");

    let proc = MainMenuSequence_NetworkServiceSelectMenuSequence::new();
    let descs = proc.get_proc_desc();

    // for (i, desc) in descs.into_iter().enumerate() {
    //     println!("Desc #{} - {}", i, desc.get_class().name());
    // }

    descs.set(1, Proc::call_2(ProcVoidMethod::from_fn(proc.as_instance(), boon_bane_set_title_bar).unwrap()));
    descs.set(5, Proc::call(ProcVoidFunction::from_fn(proc.as_instance(), boon_bane_create_menu_bind).unwrap()));
    
    proc.create_bind(parent, descs, Il2CppString::null());
}

#[skyline::main(name = "alear_boon_bane")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        println!("AlearBoonBane v{}\nLocation: {}\n\n{}", env!("CARGO_PKG_VERSION"), location, msg);

        let err_msg = format!(
            "AlearBoonBane v{}\nLocation: {}\n\n{}\0",
            env!("CARGO_PKG_VERSION"),
            location,
            msg
        );

        skyline::error::show_error(69, "AlearBoonBane has panicked! Press 'Details' for more information.\n\0", err_msg.as_str());
    }));

    println!("Hello from skyline plugin AlearBoonBane");
    cobapi::register_system_event_handler(my_system_event_listener);
    skyline::install_hooks!(network_service_select_hook, App_GameSaveDataUtil__Write);
}
