/*
   _____ _                _____ _
  / ____| |              / ____| |                                        _
 | (___ | |_ __ _ _ __  | (___ | | __ _ _ __ ___  _ __ ___   ___ _ __ ___(_)
  \___ \| __/ _` | '__|  \___ \| |/ _` | '_ ` _ \| '_ ` _ \ / _ \ '__/ __|
  ____) | || (_| | |     ____) | | (_| | | | | | | | | | | |  __/ |  \__ \_
 |_____/ \__\__,_|_|    |_____/|_|\__,_|_| |_| |_|_| |_| |_|\___|_|  |___(_)
  _____        __ _       _ _                     _                 _
 |_   _|      / _(_)     (_) |           /\      | |               | |
   | |  _ __ | |_ _ _ __  _| |_ ___     /  \   __| |_   _____ _ __ | |_ _   _ _ __ ___
   | | | '_ \|  _| | '_ \| | __/ _ \   / /\ \ / _` \ \ / / _ \ '_ \| __| | | | '__/ _ \
  _| |_| | | | | | | | | | | ||  __/  / ____ \ (_| |\ V /  __/ | | | |_| |_| | | |  __/
 |_____|_| |_|_| |_|_| |_|_|\__\___| /_/    \_\__,_| \_/ \___|_| |_|\__|\__,_|_|  \___|
*/

#![doc(html_no_source)]
#![allow(non_snake_case)]

use crate::item;
use crate::rng;
use crate::skill;

// //! This module encompasses a creature:  the player, NPCs, and monsters qualify as creatures.

/// The Creature represents all entities that can move and act.
pub struct Creature {
    name: String,
    x_pos: u32,
    y_pos: u32,

    creativity: u32,
    focus: u32,
    memory: u32,

    max_health: u32,
    current_health: u32,

    max_fatigue: u32,
    current_fatigue: u32,

    max_sanity: u32,
    current_sanity: u32,

    evasion: u32,
    endurance: u32,
    nullification: u32,

    experience_have: u32,
    image: char,
    player_control: bool,
    inventory: Vec<item::Item>,
}

impl Creature {
    pub fn new() -> Creature {
        Creature {
            name: "Unknown".to_string(),
            x_pos: 0,
            y_pos: 0,

            creativity: 0,
            focus: 0,
            memory: 0,

            max_health: 0,
            current_health: 0,

            max_fatigue: 0,
            current_fatigue: 0,

            max_sanity: 0,
            current_sanity: 0,

            evasion: 0,
            endurance: 0,
            nullification: 0,

            experience_have: 0,
            image: '?',
            player_control: false,
            inventory: Vec::new(),
        }
    } // End new

    pub fn Default_Player() -> Creature {
        Creature {
            name: "Default".to_string(),
            x_pos: 0,
            y_pos: 0,

            creativity: 20,
            focus: 20,
            memory: 20,

            max_health: 60,
            current_health: 60,

            max_fatigue: 60,
            current_fatigue: 60,

            max_sanity: 60,
            current_sanity: 60,

            evasion: 10,
            endurance: 10,
            nullification: 10,

            experience_have: 0,
            image: '@',
            player_control: true,
            inventory: Vec::new(),
        }
    } // End Default_Player

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn get_x_pos(&self) -> &u32 {
        &self.x_pos
    }

    pub fn set_x_pos(&mut self, new_x_pos: u32) {
        self.x_pos = new_x_pos;
    }

    pub fn get_y_pos(&self) -> &u32 {
        &self.y_pos
    }

    pub fn set_y_pos(&mut self, new_y_pos: u32) {
        self.y_pos = new_y_pos;
    }

    pub fn get_image(&self) -> &char {
        &self.image
    }

    pub fn set_image(&mut self, new_image: char) {
        self.image = new_image;
    }

    pub fn get_player_control(&self) -> &bool {
        &self.player_control
    }

    pub fn set_player_control(&mut self, new_player_control: bool) {
        self.player_control = new_player_control;
    }

    pub fn get_evasion(&self) -> &u32 {
        return &self.evasion;
    }
    pub fn set_evasion(&mut self, new_evasion: u32) {
        self.evasion = new_evasion;
    }
    pub fn get_endurance(&self) -> &u32 {
        return &self.endurance;
    }
    pub fn set_endurnace(&mut self, new_endurance: u32) {
        self.endurance = new_endurance;
    }
    pub fn get_nullification(&self) -> &u32 {
        return &self.nullification;
    }
    pub fn set_nullification(&mut self, new_nullification: u32) {
        self.nullification = new_nullification;
    }

    pub fn use_skill(
        &self,
        target: &mut Creature,
        the_skill: skill::Skill,
        the_rng: &mut rng::PseudoRandom,
    ) -> bool {
        let mut stat1 = 0;
        let mut stat2 = 0;
        let mut stat3 = 0;
        let mut defense = 0;

        match the_skill.GetStat1() {
            skill::StatTarget::Creativity => {
                stat1 = self.creativity;
            }
            skill::StatTarget::Focus => {
                stat1 = self.focus;
            }
            skill::StatTarget::Memory => {
                stat1 = self.memory;
            }
            skill::StatTarget::Error => {
                panic!("Error in use_skill:  stat target 1 is ERROR");
            }
        }

        match the_skill.GetStat2() {
            skill::StatTarget::Creativity => {
                stat2 = self.creativity;
            }
            skill::StatTarget::Focus => {
                stat2 = self.focus;
            }
            skill::StatTarget::Memory => {
                stat2 = self.memory;
            }
            skill::StatTarget::Error => {
                panic!("Error in use_skill:  stat target 2 is ERROR");
            }
        }

        match the_skill.GetStat3() {
            skill::StatTarget::Creativity => {
                stat3 = self.creativity;
            }
            skill::StatTarget::Focus => {
                stat3 = self.focus;
            }
            skill::StatTarget::Memory => {
                stat3 = self.memory;
            }
            skill::StatTarget::Error => {
                panic!("Error in use_skill:  stat target 3 is ERROR");
            }
        }

        match the_skill.GetDefense() {
            skill::DefenseTarget::Evasion => {
                defense = *target.get_evasion();
            }
            skill::DefenseTarget::Endurance => {
                defense = *target.get_endurance();
            }
            skill::DefenseTarget::Nullification => {
                defense = *target.get_nullification();
            }
            skill::DefenseTarget::None => {
                defense = 0;
            }
            skill::DefenseTarget::Error => {
                panic!("Error in use_skill:  defense target is ERROR");
            }
        }

        return the_rng.roll_skill_check(stat1, stat2, stat3, defense);
    }
} // End Creature Implementation.
