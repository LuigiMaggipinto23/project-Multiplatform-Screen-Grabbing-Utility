use std::ops::Index;
use std::str::FromStr;


use std::string;

use egui::TextBuffer;
use global_hotkey::hotkey::HotKey;
use keyboard_types::Modifiers;
use keyboard_types::Code;

#[derive(Debug)]
pub struct CustomizeHotkey{
    id: usize,
    modifier: String,
    code: String,
}

impl Default for CustomizeHotkey{
    fn default() -> Self {
        CustomizeHotkey{
            id: usize::MAX,
            modifier: "modifier".to_string(),
            code:   "Key".to_string(),
        }
    }
}

impl PartialEq for CustomizeHotkey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.modifier == other.modifier && self.code == other.code
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl CustomizeHotkey{
    pub fn new(id: usize,modifier: String,code: String)->Self{
        CustomizeHotkey { id: id, modifier: modifier, code: code }
    }

    pub fn get_modifier(&self) -> String{
        self.modifier.clone()
    }
    pub fn get_code(&self) -> String{
        self.code.clone()
    }


    
}
pub struct Hotkeys{
    hotkeys_vector: Vec<HotKey>,
    hotkeys_strings:   Vec<(String,String)>,
}


impl Hotkeys{
    pub fn new()-> Self{
        Hotkeys{hotkeys_vector: 
            vec![
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyE),  //Exit
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyD),  //Screen
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyS),  //Save
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyC),  //Copy
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyA),  //Save with name
            HotKey::new(Some(Modifiers::CONTROL), Code::KeyG),  //Crop
            ],
            hotkeys_strings: vec![
                ("ctrl".to_string(),"E".to_string()),
                ("ctrl".to_string(),"D".to_string()),
                ("ctrl".to_string(),"S".to_string()),
                ("ctrl".to_string(),"C".to_string()),
                ("ctrl".to_string(),"A".to_string()),
                ("ctrl".to_string(),"G".to_string()),
            ]
        }
      
    }
    
    pub fn get_hotkeys(&self)-> Vec<HotKey>{
        self.hotkeys_vector.clone()
    }
    pub fn get_hotkeys_strings(&self)-> Vec<(String,String)>{
        self.hotkeys_strings.clone()
    }

    pub fn get_hotkey_strings_formatted(&self,id: usize) -> String{
        format!("{} + {}",   self.hotkeys_strings[id].0,  self.hotkeys_strings[id].1)
    }
    pub fn update_hotkey(&mut self, new_hotkey: &CustomizeHotkey, ui: &mut egui::Ui)-> bool{
        
        let mut modifier_name: String = "CONTROL".to_string();
 

    
        match new_hotkey.modifier.as_str(){
            "alt" => {modifier_name = "ALT".to_string()},
            "ctrl" => {modifier_name = "CONTROL".to_string()},
            "shift" => {modifier_name = "SHIFT".to_string()},
            "mac_cmd" => {modifier_name = "CONTROL".to_string()},
            "command" => {modifier_name = "CONTROL".to_string()},
            _ => {}
        }
        let mut hotkey_to_assign = HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::Abort);
        
        if new_hotkey.code.parse::<u32>().is_ok(){
            hotkey_to_assign = HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Digit{}",new_hotkey.code).as_str()).unwrap());
        }else if new_hotkey.code.parse::<char>().is_ok() && new_hotkey.code.parse::<char>().unwrap().is_alphabetic(){
            hotkey_to_assign = HotKey::new(Modifiers::from_name(modifier_name.as_str()), Code::from_str(format!("Key{}",new_hotkey.code).as_str()).unwrap());
        }
        
        if self.hotkeys_vector.contains(&hotkey_to_assign){
            println!("Hotkey selezionata già utilizzata scegline un altra");
            true
        }else{
            println!("aggiorno hotkey");
            self.hotkeys_strings[new_hotkey.id]= (new_hotkey.modifier.clone(),new_hotkey.code.clone());
             *self.hotkeys_vector.get_mut(new_hotkey.id).unwrap() =hotkey_to_assign;
             false
        
        }
 

    }
}   