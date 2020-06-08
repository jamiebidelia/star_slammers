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

pub enum MenuAction
{
    CursorUp,
    CursorDown,
    Select,
    Resize,
    Invalid
}

pub fn do_action(game_mode    : &mut mode::Mode,
                 menu_action  : &MenuAction,
                 menu_cursor  : &mut usize,
                 menu         : &Vec<menu::MenuItem>)
{

    match menu_action
    {
        MenuAction::CursorUp   =>
        {
            if *menu_cursor > 0
            {
                *menu_cursor -= 1;
            }
            else
            {
                let max_field = menu.len() - 1 as usize;
                *menu_cursor = max_field;
            }
        }
        
        MenuAction::CursorDown =>
        {
            let max_field = (menu.len() - 1) as usize;

            if *menu_cursor < max_field
            {
                *menu_cursor += 1;
            }
            else
            {
                *menu_cursor = 0;
            }
        }
        MenuAction::Select     =>
        {
            menu[*menu_cursor].do_action(game_mode, menu_cursor);
        }
        MenuAction::Resize     => {} // Do nothing.
        MenuAction::Invalid    => {} // Do nothing.
    }
}
