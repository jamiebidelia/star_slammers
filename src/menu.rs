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

use crate::menu;
use crate::mode;


pub struct MenuItem<'a>
{
    text     : &'a str,
    visible  : bool,
    action   : fn(cur_mode   : &mut mode::Mode,
                  cur_cursor : &mut menu::MenuItem)
}


impl MenuItem<'_>
{
    pub fn get_text(&self) -> &str
    {
        self.text
    }

    pub fn get_visible(&self) -> bool
    {
        self.visible
    }

    pub fn do_action(&self,
                     cur_mode   : &mut mode::Mode,
                     cur_cursor : &mut menu::MenuItem)
    {
        (self.action)(cur_mode, cur_cursor);
    }
}


impl std::cmp::PartialEq for MenuItem<'_>
{
    fn eq(&self, other : &MenuItem<'_>) -> bool
    {
        return
            if self.text == other.text &&
            self.visible == other.visible
        {
            true
        }
        else
        {
            false
        };
            
    }
}




pub fn create_menu(in_text    : &str,
                   in_visible : bool,
                   in_action  : fn(cur_mode   : &mut mode::Mode,
                                   cur_cursor : &mut menu::MenuItem))
                   -> MenuItem
{
    MenuItem
    {
        text    : in_text,
        visible : in_visible,
        action  : in_action
    }
}


pub fn null_action(cur_mode   : &mut mode::Mode,
                   cur_cursor : &mut menu::MenuItem)
{
    
}

pub fn create_main_menu() -> Vec<MenuItem<'static>>
{
    let new_game  = create_menu("New Game",
                                true,
                                null_action);

    let load_game = create_menu("Load Game",
                                true,
                                null_action);

    let resume    = create_menu("Resume",
                                true,
                                null_action);

    let quit      = create_menu("Quit",
                                true,
                                null_action);

    let main_menu = vec!(new_game, load_game, resume, quit);

    main_menu
}
