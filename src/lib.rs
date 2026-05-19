use cobapi::{
    Event,
    SystemEvent,
};

use engage_il2cpp::unity_engine::vector3::Vector3;


mod structs_types;
use engage_il2cpp::system::collections::generic::{IList_1Methods, List_1};
use engage_il2cpp::unity_engine::{IComponentMethods, IGameObjectMethods, IRectTransformMethods, ITransformMethods, RectTransform, Vector2};
use structs_types::*;

use engage_il2cpp::app::{BasicMenu_Result, IBasicMenuItem, IBasicMenuItemMethods, IBasicMenuMethods, IMainMenuSequence, IMainMenuSequence_LanguageSettingMenuSequenceMethods, IMainMenuSequence_MenuSequenceBase, IMainMenuSequence_MenuSequenceBaseMethods, ISingletonProcInst_1Methods, MainMenuSequence_Label, MainMenuSequence_LanguageSettingMenuSequence_Menu, MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog, MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent, MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, MainMenuSequence_NetworkServiceSelectMenuSequence_Menu};
use engage_il2cpp::app::procinst::{IProcInst, IProcInstMethods, ProcInst};
use engage_il2cpp::app::procvoidfunction::ProcVoidFunction ;
use engage_il2cpp::app::procvoidmethod::ProcVoidMethod;
use engage_il2cpp::app::scriptutil::ScriptUtil;
use unity2::{Array, Cast, Il2CppString, IlInstance, OptionalMethod, SystemObject};

use std::sync::OnceLock;
use engage_il2cpp::ext::{GameVariableManager, ProcVoidMethodExt};
use engage_il2cpp::app::{jobdata::{IJobDataMethods, JobData}, persondata::{IPersonDataMethods, PersonData}};
use engage_il2cpp::app::structdata_1::IStructData_1Methods;
use engage_il2cpp::{List_1Ext, ProcVoidFunctionExt};
use engage_il2cpp::app::capabilitybase_1::ICapabilityBase_1Methods;
use engage_il2cpp::app::mainmenusequence::{ MainMenuSequence_NetworkServiceSelectMenuSequence};
use engage_il2cpp::app::proc::Proc;
use engage_il2cpp::app::titlebar::{ITitleBarMethods, TitleBar};
use engage_il2cpp::app::mainmenusequence::MainMenuSequence;
use engage_il2cpp::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence;
use engage_il2cpp::app::mainmenusequence::MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem;

static BACKUP_PERSON_DATA_STATS: OnceLock<PersonDataStats> = OnceLock::new();

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

pub extern "C" fn boon_bane_set_title_bar(_proc: ProcInst, _method_info: OptionalMethod) {
    TitleBar::get_instance().open_header("Boon and Banes", Il2CppString::null(), "KHID_汎用");
}

#[unity2::callback]
pub extern "C" fn boon_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    "Boon".into()
}

#[unity2::callback]
pub extern "C" fn boon_a_call_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    // MainMenuSequence_LanguageSettingMenuSequence_Menu_ConfirmDialog::create_bind(this.m_menu());
    MainMenuSequence::get_instance().set_m_next_sequence(MainMenuSequence_Label::final_confirm());
    BasicMenu_Result::close_decide()
}

#[unity2::callback]
pub extern "C" fn boon_b_call_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuItem, _method_info: OptionalMethod) -> BasicMenu_Result {
    // MainMenuSequence::get_instance().set_m_next_sequence(MainMenuSequence_Label::grow_mode_select());
    MainMenuSequence_NetworkServiceSelectMenuSequence_Menu::return_sequence();
    BasicMenu_Result::close_cancel()
}

#[unity2::callback]
pub extern "C" fn bane_get_name(this: MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem, _method_info: OptionalMethod) -> Il2CppString {
    "Bane".into()
}

pub extern "C" fn language_create_menu_bind(proc: MainMenuSequence_LanguageSettingMenuSequence, parent: ProcInst, _method_info: OptionalMethod) {
    // let list = List_1::<BasicMenuItem>::new();
    // let item1 = BasicMenuItem::instantiate().unwrap();
    // item1.set_name("test");
    // list.add(item1);
    // let menu_content= BasicDialogContent::new();
    // let basic = BasicDialog::new(list, menu_content);
    // basic.set_text("dialog");
    // basic.create_bind(proc, basic.create_default_desc(), "Dialog");
    // // BasicDialog::create_basic_dialog_bind(proc, list);
    let transform = proc.m_layout_prefab().get_transform();
    let menu_transform = transform.find("Menu");
    let menu_go = menu_transform.get_game_object();

    let menu_content = menu_go.get_component::<MainMenuSequence_LanguageSettingMenuSequence_Menu_MenuContent>();
    let menu_item_list = List_1::new();

    let boon_item = MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem::new();
    let new_class = boon_item.override_class();
    new_class.override_virtual_method("GetName", boon_get_name_method_info());
    new_class.override_virtual_method("ACall", boon_a_call_get_name_method_info());
    new_class.override_virtual_method("BCall", boon_b_call_get_name_method_info());


    let bane_item = MainMenuSequence_LanguageSettingMenuSequence_Menu_VoiceMenuItem::new();
    let new_class = bane_item.override_class();
    new_class.override_virtual_method("GetName", bane_get_name_method_info());
    new_class.override_virtual_method("ACall", boon_a_call_get_name_method_info());
    new_class.override_virtual_method("BCall", boon_b_call_get_name_method_info());


    menu_item_list.add(boon_item.into());
    menu_item_list.add(bane_item.into());

    let menu = MainMenuSequence_LanguageSettingMenuSequence_Menu::new(menu_item_list, menu_content.into());
    let descs = menu.create_default_desc();
    menu.create_bind(proc, descs, Il2CppString::null());
    menu.bind_parent_menu();
    menu.set_show_row_num(2);
    // let menu: MainMenuSequence_LanguageSettingMenuSequence = proc.m_child().try_cast::<MainMenuSequence_LanguageSettingMenuSequence>().unwrap();
    // proc.get_super().try_cast::<MainMenuSequence>().unwrap().set_m_next_sequence(MainMenuSequence_Label::r#continue());
}

pub extern "C" fn boon_bane_create_menu_bind(proc: MainMenuSequence_NetworkServiceSelectMenuSequence, _parent: ProcInst, _method_info: OptionalMethod) {
    // let list = List_1::<BasicMenuItem>::new();
    // let item1 = BasicMenuItem::instantiate().unwrap();
    // item1.set_name("test");
    // list.add(item1);
    // let menu_content= BasicDialogContent::new();
    // let basic = BasicDialog::new(list, menu_content);
    // basic.set_text("dialog");
    // basic.create_bind(proc, basic.create_default_desc(), "Dialog");
    // // BasicDialog::create_basic_dialog_bind(proc, list);
    let language = MainMenuSequence_LanguageSettingMenuSequence::new();

    let descs = language.get_proc_desc();
    descs.set(1, Proc::call_2(ProcVoidMethod::from_fn(proc.as_instance(), boon_bane_set_title_bar).unwrap()));
    descs.set(5, Proc::call(ProcVoidFunction::from_fn(language.as_instance(), language_create_menu_bind).unwrap()));

    language.create_bind(proc, descs, Il2CppString::null());
    println!("Proc current: {}, child: {}, super: {}", proc.get_name(), proc.get_child().get_name(), proc.get_super().get_name());
    // let menu: MainMenuSequence_LanguageSettingMenuSequence = proc.m_child().try_cast::<MainMenuSequence_LanguageSettingMenuSequence>().unwrap();
    // proc.get_super().try_cast::<MainMenuSequence>().unwrap().set_m_next_sequence(MainMenuSequence_Label::r#continue());
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

    for (i, desc) in descs.into_iter().enumerate() {
        println!("Desc #{} - {}", i, desc.get_class().name());
    }

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
    skyline::install_hooks!(network_service_select_hook);
}
