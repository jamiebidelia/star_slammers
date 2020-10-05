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
//! This module handles the creation of new adventurers.

extern crate pancurses;

use crate::creature;
use crate::menuaction;
use crate::mode;

// The state machine here should be fairly linear.
// We want to proceed sequentially through the choices
enum CharGenStates
{
    WelcomeText,
    EnterName,
    PickPronouns,
    PickStats,
    ConfirmText
}


pub fn draw_state(game_window : &pancurses::Window)
{
   EnterName(game_window);
}

pub fn EnterName(game_window : &pancurses::Window)
{
    // The max values it gives are not printable.  So we need to
    // Subtract 1 from each to reach our last index.  Be mindful of that.
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();

    let center_x = (end_x + start_x) / 2;

    let welcome_dots   = "****************************************";
    let welcome_text   = "* Welcome traveller.  Enter your name. *";
    let center_welcome = std::cmp::max(center_x - (welcome_text.len() / 2) as i32, 0);
    
    let name_field     = "Name:";
    
    // Clear and redraw the screen.
    game_window.erase();
    game_window.refresh();

    if pancurses::can_change_color()
    {
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
        game_window.color_set(1);
    }

    game_window.mvprintw(start_y,     center_welcome, welcome_dots);
    game_window.mvprintw(start_y + 1, center_welcome, welcome_text);
    game_window.mvprintw(start_y + 2, center_welcome, welcome_dots);

    game_window.mvprintw(start_y + 4, center_welcome, name_field);

    let name_entry_start_x = game_window.get_cur_x() + 1;
    let name_entry_y = start_y + 4;
    
    game_window.mvprintw(start_y + 4, name_entry_start_x, "_");

    let name = NameLoop(game_window,
                        name_entry_y,
                        name_entry_start_x);
}

fn NameLoop(game_window : &pancurses::Window,
            y           : i32,
            start_y     : i32) -> String
{
    "Default".to_string()
}


pub fn process_keyboard(game_window : &pancurses::Window) ->
    menuaction::MenuAction
{
    game_window.getch();
    
    menuaction::MenuAction::Invalid
}

pub fn do_action()
{
}
