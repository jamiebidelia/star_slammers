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


pub enum SliderAction
{
    CursorUp,
    CursorDown,
    Increment,
    Decrement,
    Resize,
    Back,
    Done,
    Invalid
}


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
    done        : bool,
}


impl AttributeSlider
{

    fn draw(&self, game_window : &pancurses::Window)
    {
        let CURSOR        = "-->";
        let NO_CURSOR     = "   ";
        let CURSOR_OFFSET = 4;
        let NUM_OFFSET    = 20;
        
        for i in 0 .. self.attributes.len()
        {

            let val = self.attributes[i].value;
            
            // Print the cursor, if on the correct line.
            if self.cursor_pos == i as u8
            {
                game_window.mvprintw((self.start_y + i as u8)       as i32,
                                     (self.start_x - CURSOR_OFFSET) as i32,
                                     CURSOR);
            }
            else
            {
                game_window.mvprintw((self.start_y + i as u8)       as i32,
                                     (self.start_x - CURSOR_OFFSET) as i32,
                                     NO_CURSOR);
                
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

        let REMAINING_Y = self.start_y as i32 +
                          self.attributes.len() as i32 +
                          1;
        
        let POINTS_REM  = "Points Remaining";
        let REMAINING   = self.points_left.to_string();
        
        game_window.mvprintw(REMAINING_Y,
                             self.start_x as i32,
                             POINTS_REM);

        game_window.mvprintw(REMAINING_Y,
                             (self.start_x + NUM_OFFSET) as i32,
                             REMAINING);
    }


    /// process_keyboard transforms keypresess into SliderActions.  It does
    /// no bounds checking or validity checks on if the action can be performed.
    fn process_keyboard(&mut self, game_window : &pancurses::Window) -> SliderAction
    {

        let ENTER : char = '\n';
        let ESC   : char = '\x1B';
        
        let mut the_action = SliderAction::Invalid;

        match game_window.getch()
        {
            Some(pancurses::Input::KeyResize) =>
            {
                the_action = SliderAction::Resize;
            }
            Some(pancurses::Input::KeyUp) =>
            {
                the_action = SliderAction::CursorUp;
            }
            Some(pancurses::Input::KeyDown) =>
            {
                the_action = SliderAction::CursorDown;
            }
            Some(pancurses::Input::KeyRight) =>
            {
                the_action = SliderAction::Increment;
            }
            Some(pancurses::Input::KeyLeft) =>
            {
                the_action = SliderAction::Decrement;
            }
            Some(pancurses::Input::Character(key)) =>
            {
                if key == ESC
                {
                    the_action = SliderAction::Back;
                }
                if key == ENTER
                {
                    the_action = SliderAction::Done;
                }
            }
            Some(_) => {} // Do nothing;
            None    => {} // Do nothing;
        }

        the_action
    }

    /// process_action mutates the AttributeSlider based on the provided
    /// SliderAction.
    fn process_action(&mut self,
                      the_action : SliderAction)
    {
        match the_action
        {
            SliderAction::CursorUp =>
            {
                if self.cursor_pos == 0
                {
                    self.cursor_pos = self.attributes.len() as u8 - 1;
                }
                else
                {
                    self.cursor_pos = self.cursor_pos - 1;
                }
            }
            SliderAction::CursorDown =>
            {
                if self.cursor_pos == self.attributes.len() as u8 - 1
                {
                    self.cursor_pos = 0;
                }
                else
                {
                    self.cursor_pos = self.cursor_pos + 1;
                }
            }
            SliderAction::Increment =>
            {
                if self.points_left > 0 &&
                   self.attributes[self.cursor_pos as usize].increment() == true
                {
                    self.points_left = self.points_left - 1;
                }
            }
            SliderAction::Decrement =>
            {
                if self.attributes[self.cursor_pos as usize].decrement() == true
                {
                    self.points_left = self.points_left + 1;
                }
            }
            SliderAction::Resize =>
            {
                // Do nothing;
            }
            SliderAction::Done =>
            {
                if self.points_left == 0
                {
                    self.done = true;
                }
            }
            SliderAction::Back =>
            {
                // Do nothing;
                // TODO: Make this go back to the Name Entry point.
            }
            SliderAction::Invalid =>
            {
                // Do nothing;
            }
        }
    }
    
    fn input_loop(&mut self, game_window : &pancurses::Window)
    {
        while self.done == false
        {
            // First let's draw what we have.
            self.draw(game_window);
            
            // Then, get a keyboard command from the player.
            let the_action = self.process_keyboard(game_window);

            // Change state based on the action (and move the_action).
            self.process_action(the_action);
        }
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
            start_y     : start_y,
            done        : false
        };

        slider.input_loop(game_window);
        
        
        // Return a tuple of the the finished attributes.
        (slider.attributes[0].value,
         slider.attributes[1].value,
         slider.attributes[2].value)
    }
}
