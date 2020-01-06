

/*pub enum SkillTarget
{
    Creativity,
    Focus,
    Memory,
    None
}

pub enum DefenseTarget
{
    Health,
    Fatigue,
    Sanity,
    None
}

pub enum Effect
{
    None
}*/

pub mod skill
{
    pub struct Skill
{
    name        : String,
    range       : u32,
   // skill1      : SkillTarget,
   // skill2      : SkillTarget,
   // skill3      : SkillTarget,
   // defense     : DefenseTarget,
    description : String,
   // effect      : Effect
}

impl Skill
{
    pub fn new() -> Skill
    {
        Skill
        {
            name        : "N/A",
            range       : 0,
        //    skill1      : SkillTarget::None,
        //    skill2      : SkillTarget::None,
       //     skill3      : SkillTarget::None,
       //     defense     : DefenseTarget::None,
            description : "N/A",
     //       effect      : None
        }
    }
}

}
