use crate::item;

pub struct Creature
{
    name             : String,
    x_pos            : u32,
    y_pos            : u32,

    max_health       : u32,
    current_health   : u32,

    max_mana         : u32,
    current_mana     : u32,

    max_stamina      : u32,
    current_stamina  : u32,
    
    armor_class      : u32,
    
    strength         : u32,
    dexterity        : u32,
    constitution     : u32,
    intelligence     : u32,
    wisdom           : u32,
    charisma         : u32,

    experience_have  : u32,
    experience_worth : u32,

    image            : char,

    player_control   : bool,
    
    inventory        : Vec<item::Item>
}

impl Creature
{
    pub fn new() -> Creature
    {
        Creature
        {
            name             : "Unknown".to_string(),
            
            x_pos            : 0,
            y_pos            : 0,

            max_health       : 0,
            current_health   : 0,
        
            max_mana         : 0,
            current_mana     : 0,

            max_stamina      : 0,
            current_stamina  : 0,
        
            armor_class      : 0,
        
            strength         : 0,
            dexterity        : 0,
            constitution     : 0,
            intelligence     : 0,
            wisdom           : 0,
            charisma         : 0,
        
            experience_have  : 0,
            experience_worth : 0,

            image            : '?',
        
            inventory        : Vec::new(),

            player_control   : false,
        }
    } // End new.

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

    pub fn get_max_health(&self) -> &u32
    {
        &self.max_health
    }

    pub fn set_max_health(&mut self, new_max_health : u32)
    {
        self.max_health = new_max_health;
    }

    pub fn get_current_health(&self) -> &u32
    {
        &self.current_health
    }

    pub fn set_current_health(&mut self, new_current_health : u32)
    {
        self.current_health = new_current_health;
    }

    pub fn get_max_mana(&self) -> &u32
    {
        &self.max_mana
    }

    pub fn set_max_mana(&mut self, new_max_mana : u32)
    {
        self.max_mana = new_max_mana;
    }

    pub fn get_current_mana(&self) -> &u32
    {
        &self.current_mana
    }

    pub fn set_current_mana(&mut self, new_current_mana : u32)
    {
        self.current_mana = new_current_mana;
    }

    pub fn get_max_stamina(&self) -> &u32
    {
        &self.max_stamina
    }

    pub fn set_max_stamina(&mut self, new_max_stamina : u32)
    {
        self.max_stamina = new_max_stamina;
    }

    pub fn get_current_stamina(&self) -> &u32
    {
        &self.current_stamina
    }

    pub fn set_current_stamina(&mut self, new_current_stamina : u32)
    {
        self.current_stamina = new_current_stamina;
    }

    pub fn get_armor_class(&self) -> &u32
    {
        &self.armor_class
    }

    pub fn set_armor_class(&mut self, new_armor_class : u32)
    {
        self.armor_class = new_armor_class;
    }

    pub fn get_strength(&self) -> &u32
    {
        &self.strength
    }

    pub fn set_strength(&mut self, new_strength : u32)
    {
        self.strength = new_strength;
    }

    pub fn get_dexterity(&self) -> &u32
    {
        &self.dexterity
    }

    pub fn set_dexterity(&mut self, new_dexterity : u32)
    {
        self.dexterity = new_dexterity;
    }

    pub fn get_constitution(&self) -> &u32
    {
        &self.constitution
    }

    pub fn set_constitution(&mut self, new_constitution : u32)
    {
        self.constitution = new_constitution;
    }

    pub fn get_intelligence(&self) -> &u32
    {
        &self.intelligence
    }

    pub fn set_intelligence(&mut self, new_intelligence : u32)
    {
        self.intelligence = new_intelligence;
    }

    pub fn get_wisdom(&self) -> &u32
    {
        &self.wisdom
    }

    pub fn set_wisdom(&mut self, new_wisdom : u32)
    {
        self.wisdom = new_wisdom;
    }

    pub fn get_charisma(&self) -> &u32
    {
        &self.charisma
    }

    pub fn set_charisma(&mut self, new_charisma : u32)
    {
        self.charisma = new_charisma;
    }

    pub fn get_experience_have(&self) -> &u32
    {
        &self.experience_have
    }

    pub fn set_experience_have(&mut self, new_experience_have : u32)
    {
        self.experience_have = new_experience_have;
    }

    pub fn get_experience_worth(&self) -> &u32
    {
        &self.experience_worth
    }

    pub fn set_experience_worth(&mut self, new_experience_worth : u32)
    {
        self.experience_worth = new_experience_worth;
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

    pub fn get_inventory_handle (&mut self) -> &mut Vec<item::Item>
    {
        &mut self.inventory
    }
    
} // End Creature Implementation.
