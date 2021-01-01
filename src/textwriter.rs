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

//! This module handles input into the system by writeable text box.

extern crate pancurses;


/// Key_Type checks if the key pressed was a valid key to add to a TextWriter, or
/// if it signals done (enter) or cancel (esc).
#[derive(PartialEq)]
enum KeyType
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
    max_len       : u8,
    text_field    : Vec<char>,
    prompt        : String,
    allow_nums    : bool,
    allow_chars   : bool,
    allow_special : bool,
    start_x       : u8,
    start_y       : u8,
}

impl TextWriter
{

    /// run fills in a TextWriter with the initial data and
    /// runs the processing loop.
    pub fn run(max_len       : u8,
               prompt        : String,
               allow_nums    : bool,
               allow_chars   : bool,
               allow_special : bool,
               start_x       : u8,
               start_y       : u8,
               game_window   : &pancurses::Window,) -> Option<String>
    {
        let mut textWriter = TextWriter
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
        let got_text = textWriter.input_loop(game_window);

        //Return area:  Either 
        if got_text
        {
            Some(textWriter.text_field.into_iter().collect())
        }
        else
        {
            None
        }
    }

    /// is_alpha returns true if the passed char is alphabetical,
    /// false otherwise.
    fn is_alpha(key : &char) -> bool
    {
        match key
        {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' |
            'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
            'v' | 'w' | 'x' | 'y' | 'z' => true,

            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' |
            'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' |
            'V' | 'W' | 'X' | 'Y' | 'Z' => true,

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
    
    pub fn input_key(&mut self, game_window : &pancurses::Window) -> KeyType
    {
        let input = game_window.getch();
        
        let mut result = KeyType::KEY_PRESSED;

        let ESC       : char = '\x1B';
        let BACKSPACE : char = '\x08';

        let ASCII_LOWER : char ='\x20';
        let ASCII_UPPER : char ='\x7E';
       
        match input
        {
            Some(pancurses::Input::KeyBackspace) =>
            {
                result = KeyType::BACKSPACE_PRESSED;
                
                if self.text_field.len() > 0
                {
                    self.text_field.pop();
                }
            }
            Some(pancurses::Input::Character(key)) =>
            {

                game_window.mvprintw(15, 2, key.to_string());
                
                if key == ESC {result = KeyType::ESC_PRESSED;}
                
                if key == BACKSPACE
                {
                    result = KeyType::BACKSPACE_PRESSED;

                    if self.text_field.len() > 0
                    {
                        self.text_field.pop();
                    }
                }

                if key >= ASCII_LOWER && key <= ASCII_UPPER
                {
                    result = KeyType::KEY_PRESSED;


                    if TextWriter::is_alpha(&key) && self.allow_chars &&
                        self.text_field.len() < self.max_len as usize
                    {
                        self.text_field.push(key);
                    }
                    if TextWriter::is_num(&key) && self.allow_nums &&
                        self.text_field.len() < self.max_len as usize
                    {
                        self.text_field.push(key);
                    }
                    else if TextWriter::is_special(&key) && self.allow_special &&
                        self.text_field.len() < self.max_len as usize
                    {
                        self.text_field.push(key);
                    }
                }
            }
            Some(_) =>
            {
                result = KeyType::KEY_UNKNOWN;
            }
            None =>
            {
                result = KeyType::KEY_UNKNOWN;
            }
        }
        
        result
    }

                   

    /// input_loop populates the string vector by listening for inputs.
    /// It returns true if successful, and false if ESC was pressed.
    pub fn input_loop(&mut self,
                      game_window : &pancurses::Window) -> bool
    {
        let mut done   : bool = false;
        let mut result : bool = false;

        let mut count = 0;
        
        game_window.mvprintw(42,
                             2,
                             "                        .");


        
        // Run until either enter or esc is pressed, then return the results if
        // the key was enter.  If it was ESC, just give None.
        while !done
        {
            count = count + 1;
            game_window.mvprintw(39,
                                 2,
                                 "Top of loop.");

            
            // Draw the text box, then wait on input from the user.
            self.draw(game_window);

            game_window.mvprintw(40,
                                 2,
                                 "Before Input_Key.");

            
            let input = self.input_key(game_window);

            game_window.mvprintw(41,
                                 2,
                                 "After Input_Key.");

            match input
            {
                KeyType::ENTER_PRESSED =>
                {
                    result = true;
                    done   = true;
                    
                    game_window.mvprintw(42,
                                         2,
                                         "Enter Key Entered.");
                }

                KeyType::ESC_PRESSED =>
                {
                    // Abort here.
                    result = false;
                    done   = true;
                    game_window.mvprintw(42,
                                         2,
                                         "ESC Key Entered.");
                }

                KeyType::KEY_PRESSED =>
                {
                    game_window.mvprintw(42,
                                         2,
                                         "AlphNumSpec Key Entered.");
                }

                KeyType::BACKSPACE_PRESSED =>
                {
                }

                KeyType::KEY_UNKNOWN =>
                {
                }
            }



            let text : String = self.text_field.iter().collect();
            game_window.mvprintw(44,
                                 2,
                                 text);
            
            game_window.mvprintw(45,
                                 2,
                                 "ITERS:  ".to_owned() + &count.to_string());

            game_window.mvprintw(46,
                                 2,
                                 "Len:  ".to_owned() + &self.text_field.len().to_string());
            
        }
        
        // Return the result back (Either None or Some(String)).
        result
    }

    /// draw draws the prompt and current text_field to the screen.  It also
    /// displays an underscore, _, after the text if there is room
    /// for additional input.
    pub fn draw(&self, game_window : &pancurses::Window)
    {
        //Draw the Prompt;
        game_window.mvprintw(self.start_y as i32,
                             self.start_x as i32,
                             &self.prompt);

        let start_text_x  = self.start_x + self.prompt.len() as u8 + 2;
        let mut step      : usize = 0;
        let mut done      = false;

        while !done
        {
            if step == self.max_len as usize { done = true; }

            if step < self.text_field.len()
            {
                let the_char = &self.text_field[step].to_string();
                
                game_window.mvprintw(self.start_y as i32,
                                (start_text_x + step as u8) as i32,
                                     the_char);
            }
            else
            {
                if step < self.max_len as usize
                {
                    game_window.mvprintw(self.start_y as i32,
                                         (start_text_x + step as u8) as i32,
                                         "_");
                }
                  
            }

            step = step + 1;
        }
    }
}
