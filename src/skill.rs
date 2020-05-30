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
//! This module handles skill interpretation and use.

/// A skill's target values can be Creativity, Focus, or Memory. Error is for validation.
pub enum StatTarget
{
    Creativity,
    Focus,
    Memory,
    Error
}

/// A skill's defence target can be Evasion, Endurance, Nullification, or None.  Error is for valdiation.
pub enum DefenseTarget
{
    Evasion,
    Endurance,
    Nullification,
    None,
    Error
}

/// As effects are added to the game, they must be added to this enumeration.
pub enum Effect
{
    None,
    DamageHealth,
    DamageStamina,
    DamageSanity,
    Error
}

/// The Skill packages together all attributes necessary to parse or use a skill.
pub struct Skill
{
    name        : String,
    range       : u32,
    stat1       : StatTarget,
    stat2       : StatTarget,
    stat3       : StatTarget,
    defense     : DefenseTarget,
    description : String,
    effect      : Effect
}

impl Skill
{
    /// new constructs a new Skill with invalid values; they must be populated before this Skill instance can be used!
    pub fn new() -> Skill
    {
        Skill
        {
            name        : "N/A".to_string(),
            range       : 0,
            stat1       : StatTarget::Error,
            stat2       : StatTarget::Error,
            stat3       : StatTarget::Error,
            defense     : DefenseTarget::Error,
            description : "N/A".to_string(),
            effect      : Effect::Error
        }
    }

    pub fn GetName(&self) -> &String
    {
        return &self.name;
    }
    
    pub fn GetRange(&self) -> &u32
    {
        return &self.range;
    }

    pub fn GetStat1(&self) -> &StatTarget
    {
        return &self.stat1;
    }
    
    pub fn GetStat2(&self) -> &StatTarget
    {
        return &self.stat2;
    }
    
    pub fn GetStat3(&self) -> &StatTarget
    {
        return &self.stat3;
    }
    
    pub fn GetDefense(&self) -> &DefenseTarget
    {
        return &self.defense;
    }
    
    pub fn GetDescription(&self) -> &String
    {
        return &self.description;
    }
    
    pub fn GetEffect(&self) -> &Effect
    {
        return &self.effect;
    }

} // end impl Skill



