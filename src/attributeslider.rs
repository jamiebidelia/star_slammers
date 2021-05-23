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
#![allow(non_camel_case_types)]

//! This module handles the adding and removing of attributes
//! via key commands (up/down to select, left/right to add or remove).

extern crate pancurses;

/// Attribute composes the base unit of an attribute slider.
pub struct Attribute
{
    name  : String,
    desc  : String,
    value : u8,
    max   : u8,
    min   : u8,
}

impl Attribute
{
    /// increment checks to see if the attribute is already at max.
    /// If it is not, we increment the value; if it is, we do not.
    /// We return whether or not the increment was successful.
    pub fn increment(&mut self) -> bool
    {
        let mut ret_val = true;
        if self.value == self.max
        {
            ret_val = false;
        }
        else
        {
            self.value = self.value + 1;
        }

        ret_val
    }

    /// decrement checks to see if the attribute is already at min.
    /// If it is not, we decrement the value; if it is, we do not.
    /// We return whether or not the decrement was successful.
    pub fn decrement(&mut self) -> bool
    {
        let mut ret_val = true;

        if self.value == self.min
        {
            ret_val = false;
        }
        else
        {
            self.value = self.value - 1;
        }

        ret_val
    }


    /// new is a simple constructor for the Attribute struct.
    pub fn new(name  : String,
               desc  : String,
               value : u8,
               max   : u8,
               min   : u8) -> Attribute
    {
        Attribute
        {
            name,
            desc,
            value,
            max,
            min
        }
    }
}

pub struct AttributeSlider
{
    attributes  : Vec<Attribute>,
    cursor_pos  : u8,
    points_left : u8,
    start_x     : u8,
    start_y     : u8,
}


impl AttributeSlider
{

    fn draw(&self, game_window : &pancurses::Window)
    {
        let CURSOR        = "-->";
        let CURSOR_OFFSET = 4;
        let NUM_OFFSET    = 14;
        
        for i in 0 .. self.attributes.len()
        {

            let val = self.attributes[i].value;
            
            // Print the cursor, if on the correct line.
            if (self.cursor_pos == i as u8)
            {
                game_window.mvprintw((self.start_y + i as u8)       as i32,
                                     (self.start_x - CURSOR_OFFSET) as i32,
                                     CURSOR);
            }

            // Print the attribute's name.
            game_window.mvprintw((self.start_y + i as u8) as i32,
                                 self.start_x as i32,
                                 &self.attributes[i].name);

            // Print the attribute's current value.
            game_window.mvprintw((self.start_y + i as u8)     as i32,
                                 (self.start_x + NUM_OFFSET)  as i32,
                                 val.to_string());
        }

    }

    fn input_loop(&self, game_window : &pancurses::Window)
    {
        //First let's draw what we have.
        self.draw(game_window);
    }
    
    pub fn run(start_x     : u8,
               start_y     : u8,
               game_window : &pancurses::Window) -> (u8, u8, u8)
    {
        let creativity_slider =
            Attribute::new("Creativity".to_string(), //name
                          "How capable the Slammer is of imagining new outcomes and bringing them into this world.".to_string(),
                          23,           // value
                          30,           // max
                          10);          // min


        let focus_slider =
            Attribute::new("Focus".to_string(), //name
                          "How capable the Slammer is of asserting their will without distraction.".to_string(),
                          23,           // value
                          30,           // max
                          10);          // min

        let memory_slider =
            Attribute::new("Memory".to_string(),     //name
                          "How capable the Slammer is of recalling ancient arcana, and the size of their mental workspace.to_string()".to_string(),
                          23,           // value
                          30,           // max
                          10);          // min

        let attribute_vec = vec!(creativity_slider, focus_slider, memory_slider);

        // Construct the slider and move the vector into it.
        let mut slider = AttributeSlider
        {
            attributes  : attribute_vec,
            cursor_pos  : 0,
            points_left : 5,
            start_x     : start_x,
            start_y     : start_y 
        };

        slider.input_loop(game_window);
        
        
        // Return a tuple of the the finished attributes.
        (slider.attributes[0].value,
         slider.attributes[1].value,
         slider.attributes[2].value)
    }
}
