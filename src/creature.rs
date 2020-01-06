#![doc(html_no_source)]

use crate::item;
use crate::rng;
use crate::skill;

// //! This module encompasses a creature:  the player, NPCs, and monsters qualify as creatures.


/// The Creature represents all entities that can move and act.
pub struct Creature
{
    name                  : String,
    x_pos                 : u32,
    y_pos                 : u32,

    creativity            : u32,
    focus                 : u32,
    memory                : u32,
    
    max_health            : u32,
    current_health        : u32,

    max_fatigue           : u32,
    current_fatigue       : u32,

    max_sanity            : u32,
    current_sanity        : u32,
    
    current_evasion       : u32,
    current_endurance     : u32,
    current_nullification : u32,
    
    experience_have       : u32,
    image                 : char,
    player_control        : bool,
    inventory             : Vec<item::Item>
}

impl Creature
{
    pub fn new() -> Creature
    {
        Creature
        {
            
            name                  : "Unknown".to_string(),
            x_pos                 : 0,
            y_pos                 : 0,

            creativity            : 0,
            focus                 : 0,
            memory                : 0,
            
            max_health            : 0,
            current_health        : 0,
            
            max_fatigue           : 0,
            current_fatigue       : 0,
            
            max_sanity            : 0,
            current_sanity        : 0,
            
            current_evasion       : 0,
            current_endurance     : 0,
            current_nullification : 0,
            
            experience_have       : 0,
            image                 : '?',
            player_control        : false,
            inventory             : Vec::new(),
        }
    } // End new

    pub fn get_name(&self) -> &String
    {
        &self.name
    }
    
    pub fn set_name(&mut self, new_name : String)
    {
        self.name = new_name;
    }
    
    pub fn get_x_pos(&self) -> &u32
    {
        &self.x_pos
    }

    pub fn set_x_pos(&mut self, new_x_pos : u32)
    {
        self.x_pos = new_x_pos;
    }

    pub fn get_y_pos(&self) -> &u32
    {
        &self.y_pos
    }

    pub fn set_y_pos(&mut self, new_y_pos : u32)
    {
        self.y_pos = new_y_pos;
    }

    pub fn get_image(&self) -> &char
    {
        &self.image
    }

    pub fn set_image(&mut self, new_image : char)
    {
        self.image = new_image;
    }

    pub fn get_player_control(&self) -> &bool
    {
        &self.player_control
    }

    pub fn set_player_control(&mut self, new_player_control : bool)
    {
        self.player_control = new_player_control;
    }

/*    pub fn make_skill_check(&self, target : &mut Creature, the_skill : skill::Skill, the_rng : &mut rng::PseudoRandom) -> bool
    {
        return false;
    }
  */  
} // End Creature Implementation.
