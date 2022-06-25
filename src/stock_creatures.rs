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

#![allow(non_snake_case)]
//! This module forms the stock creatures that exist in Star Slammers.  They
//! will be replaceable at a future time with drop-in lua files.

use crate::creature;

fn pad(obj : &mut creature::Creature, image : char, name : &str)
{
    obj.set_image(image);
    obj.set_name(name.to_string());
    obj.set_creativity(10);
    obj.set_focus(10);
    obj.set_memory(10);
    obj.set_max_health(10);
    obj.set_current_health(10);
    obj.set_max_fatigue(10);
    obj.set_current_fatigue(10);
    obj.set_max_sanity(10);
    obj.set_current_sanity(10);
    obj.set_evasion(10);
    obj.set_endurance(10);
    obj.set_nullification(10);
    obj.set_experience_have(10);
    obj.set_player_control(false);
    obj.new_inventory();
}


pub fn default_creature_vector() -> Vec<creature::Creature>
{
    // Create the vector.
    let mut creature_vector = Vec::new();

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'a', "Android");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'b', "Brain Dog");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'c', "Creeping Vine");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'd', "Digger");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'e', "Elemental");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'f', "Fortis");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'g', "Giant");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'h', "Human");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'i', "Icklerd");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'j', "Jobber");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'k', "Kobold");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'l', "Luminous Being");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'm', "Mindflayer");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'n', "Nurndulack");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'o', "Orbital Eyes");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'p', "Periscope");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'q', "Quickling");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'r', "Rooter");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 's', "Slave");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 't', "Techmonger");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'u', "Ungulate");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'v', "Vanguard");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'w', "Warrior");
    creature_vector.push(creature);
    
    let mut creature = creature::Creature::new();
    pad(&mut creature, 'x', "Xorn");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'y', "You?");
    creature_vector.push(creature);

    let mut creature = creature::Creature::new();
    pad(&mut creature, 'z', "Zangian");
    creature_vector.push(creature);

    // Return the vector of creatures.
    creature_vector
}
