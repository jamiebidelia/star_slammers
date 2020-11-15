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
//! This module handles input into the system by writeable text box.

extern crate pancurses;


/// Key_Type checks if the key pressed was a valid key to add to a TextWriter, or
/// if it signals done (enter) or cancel (esc).
enum Key_Type
{
    BACKSPACE_PRESSED,
    KEY_PRESSED,
    ENTER_PRESSED,
    ESC_PRESSED,
    KEY_UNKNOWN
}

/// TextWriter handles the data and logic for entering text into boxes
/// using the keyboard.
pub struct TextWriter
{
    max_len       : u8;
    text_field    : Vec<char>;
    prompt        : String;
    allow_nums    : bool;
    allow_chars   : bool;
    allow_special : bool;
    start_x       : u8;
    start_y       : u8;
}

impl TextWriter
{

    /// run fills in a TextWriter with the initial data and
    /// runs the processing loop.
    pub fn run(max_len       : u8;
               prompt        : String;
               allow_nums    : bool;
               allow_chars   : bool;
               allow_special : bool;
               start_x       : u8;
               start_y       : u8;) -> Option<String>
    {
        let textWriter : TextWriter =
        {
            max_len,
            text_field: Vec::new(),
            prompt,
            allow_nums,
            allow_chars,
            allow_special,
            start_x,
            start_y
        };

        // Run the processing loop and return the results.
         textWriter.input_loop()
        
    }

    /// is_alpha returns true if the passed char is alphabetical,
    /// false otherwise.
    fn is_alpha(key : &char) -> bool
    {
        match key
        {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' |
            'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
            'v' | 'w' | 'x' | 'y' | 'z' | => true,

            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' |
            'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' |
            'V' | 'W' | 'X' | 'Y' | 'Z' | => true,

            _ => false,
        }
    }

    /// is_num returns true if the passed char is a numeral,
    /// false otherwise.
    fn is_num(key : &char) -> bool
    {
        match key
        {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,

            _ => false,
        }
    }

    /// is_special returns true if the passed char is an ASCII symbol,
    /// false otherwise.
    fn is_special(key : &char) -> bool
    {
        match key
        {
            '`' | '~' | '!' | '@' | '#' | '$'  | '%'  | '^' | '&' | '*' |
            '(' | ')' | '-' | '_' | '=' | '+'  | '\\' | '|' | '[' | '{' |
            ']' | '}' | ';' | ':' | '\'' | '"' | ','  | '<' | '.' | '>' |
            '/' | '?' => true,
            
            _ => false,
        }
    }
    
    pub fn input_key(&mut self, game_window : &pancurses::Window) -> Key_Type
    {
        let input = game_window.getch();
        
        let mut result = KEY_PRESSED;

        let ESC : char = '\x1B'; 

        match input()
        {
            KeyBackspace =>
                if text_field.len() > 0
                {
                    text_field.remove(len-1);
                    result = BACKSPACE_PRESSED;
                }
            
            KeyEnter =>
                result = ENTER_PRESSED;

            Character(key) =>
            {
                if is_num(key) && allow_nums
                {
                    text_field.push(key);
                    result == KEY_PRESSED;
                }
                else if is_alpha(key) && allow_chars
                {
                    text_field.push(key);
                    result == KEY_PRESSED;
                }
                else if is_special(key) && allow_special
                {
                    text_field.push(key);
                    result == KEY_PRESSED;
                }
                else if key == ESC
                {
                    result == ESC_PRESSED;
                }
                else
                {
                    result == KEY_UNKNOWN;
                }
            }
        }
    }

    /// input_loop populates the string vector by listening for inputs.
    /// It returns Some(String) if successful, and None if ESC was pressed.
    pub fn input_loop(&mut self,
                      game_window : &pancurses::Window) -> Option<String>
    {
        let mut done   : bool           = false;
        let mut result : Option<String> = None;

        // Run until either enter or esc is pressed, then return the results if
        // the key was enter.  If it was ESC, just give None.
        while !done
        {
            let result = input_key(game_window);
            
            if result == ENTER_PRESSED
            {
                // Move the string out of the TextField,
                // as the struct will die soon.
                result = Some(text_field);
                done   = true;
            }
            else if result == ESC_PRESSED
            {
                // Abort here.
                done = true;
            }
        }
        // Return the result back (Either None or Some(String)).
        result
    }

    pub fn draw(&self, game_window : &pancurses::Window)
    {
    }
}
