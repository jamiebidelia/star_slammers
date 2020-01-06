#![doc(html_no_source)]
//! This module handles skill interpretation and use.

/// A skill's target values can be Creativity, Focus, or Memory. Error is for validation.
pub enum SkillTarget
{
    Creativity,
    Focus,
    Memory,
    Error
}

/// A skill's defence target can be Health, Fatigue, Sanity, or None.  Error is for valdiation.
pub enum DefenseTarget
{
    Health,
    Fatigue,
    Sanity,
    Error
}

/// As effects are added to the game, they must be added to this enumeration.
pub enum Effect
{
    None
}

/// The Skill packages together all attributes necessary to parse or use a skill.
pub struct Skill
{
    name        : String,
    range       : u32,
    skill1      : SkillTarget,
    skill2      : SkillTarget,
    skill3      : SkillTarget,
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
            name        : "N/A",
            range       : 0,
            skill1      : SkillTarget::None,
            skill2      : SkillTarget::None,
            skill3      : SkillTarget::None,
            defense     : DefenseTarget::None,
            description : "N/A",
            effect      : None
        }
    }
}


