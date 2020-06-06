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

extern crate pancurses;

use std::io::BufReader;
use std::fs::File;
use std::env;

use crate::menu;
use crate::action;


pub fn draw_title(game_window  : &pancurses::Window)
{
    // The max values it gives are not printable.  So we need to
    // Subtract 1 from each to reach our last index.  Be mindful of that.
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();


    //99 chars / 2 is 44.5  So take center of screen and subtract 44 characters?
    let mut title_x =  ((start_x+end_x) / 2) - 44;
    if title_x < start_x
    {
        title_x = start_x;
    }


    // We want to print the title, but "\" is an escape character.  So we need to use "\\" instead.
    
    let mut title_array =[""; 12];

    title_array[0]  = "  _____ _                _____ _                                                                 ";
    title_array[1]  = " / ____| |              / ____| |                                        _                       ";
    title_array[2]  = "| (___ | |_ __ _ _ __  | (___ | | __ _ _ __ ___  _ __ ___   ___ _ __ ___(_)                      ";
    title_array[3]  = " \\___ \\| __/ _` | '__|  \\___ \\| |/ _` | '_ ` _ \\| '_ ` _ \\ / _ \\ '__/ __|                 ";
    title_array[4]  = " ____) | || (_| | |     ____) | | (_| | | | | | | | | | | |  __/ |  \\__ \\_                     ";
    title_array[5]  = "|_____/ \\__\\__,_|_|    |_____/|_|\\__,_|_| |_| |_|_| |_| |_|\\___|_|  |___(_)                  ";
    title_array[6]  = " _____        __ _       _ _                     _                 _                             ";
    title_array[7]  = "|_   _|      / _(_)     (_) |           /\\      | |               | |                           ";
    title_array[8]  = "  | |  _ __ | |_ _ _ __  _| |_ ___     /  \\   __| |_   _____ _ __ | |_ _   _ _ __ ___           ";
    title_array[9]  = "  | | | '_ \\|  _| | '_ \\| | __/ _ \\   / /\\ \\ / _` \\ \\ / / _ \\ '_ \\| __| | | | '__/ _ \\ ";
    title_array[10] = " _| |_| | | | | | | | | | | ||  __/  / ____ \\ (_| |\\ V /  __/ | | | |_| |_| | | |  __/         ";
    title_array[11] = "|_____|_| |_|_| |_|_| |_|_|\\__\\___| /_/    \\_\\__,_| \\_/ \\___|_| |_|\\__|\\__,_|_|  \\___|  ";
    
    
    // Clear and redraw the screen.
    game_window.erase();
    game_window.refresh();

    if pancurses::can_change_color()
    {
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
        game_window.color_set(1);
    }

    //The title array of is size 12.
    for index in 0..12
    {
        game_window.mvprintw(start_y + index,
                             title_x,
                             title_array[index as usize]);
    }
}

pub fn draw_dedication(game_window : &pancurses::Window)
{
   let start_x = game_window.get_beg_x();
   let end_x   = game_window.get_max_x();

   let start_y = game_window.get_beg_y();
   let end_y   = game_window.get_max_y();

   let dedication_string = "A labor of love by Jamie Bidelia O'Brien, 2020";
   let dedication_length = dedication_string.len();

   // Center the dedication horizontally.
   let dedication_x =
      (start_x + (((end_x - start_x) as f32 * 0.5) as i32)) -
      (dedication_length / 2) as i32;
   
   // Indent 95% from the top.
   let dedication_y =
   start_y + (((end_y - start_y) as f32 * 0.95) as i32);
   
   game_window.mvprintw(dedication_y, dedication_x, dedication_string);
}

pub fn draw_menu(game_window  : &pancurses::Window,
                 menu_pointer : &Option<&menu::MenuNode>)
{
   let start_x = game_window.get_beg_x();
   let end_x   = game_window.get_max_x();

   let start_y = game_window.get_beg_y();
   let end_y   = game_window.get_max_y();

   // Indent 40% from the left.
   let menu_x_beginning = start_x + (((end_x - start_x) as f32 * 0.4) as i32);
   
   // Indent 50% from the top.
   let menu_y_beginning = start_y + (((end_y - start_y) as f32 * 0.5) as i32);

   let draw_menu_pointer = &menu::MAIN_MENU;
   
   draw_menu_children(game_window,
                       draw_menu_pointer,
                       menu_y_beginning,
                       menu_x_beginning);
   
}

fn draw_menu_children(game_window   : &pancurses::Window,
                       menu_pointer : &menu::MenuNode,
                       y_pos        : i32,
                       x_pos        : i32)
{
   let mut y_scroller = y_pos;
   
   let option_children = menu_pointer.get_children();
   match option_children
   {
      None =>
      {
         // Skip printing out the menu if there are no children to print.
      }
      
      Some(menu_vector) => 
      {
         for menu_item in menu_vector
         {
            game_window.mvprintw(y_scroller, x_pos, menu_item.get_text());
            y_scroller += 3;
         }
      }
   }
}

pub fn process_keyboard(game_window : &pancurses::Window) -> action::Action
{

    let mut game_action = action::Action::Invalid;
    
    match game_window.getch()
    {
        Some(pancurses::Input::KeyResize) =>
        {
            game_action = action::Action::Resize;
        }
        Some(pancurses::Input::KeyUp) =>
        {
            game_action = action::Action::MoveUp;
        }
        Some(pancurses::Input::KeyDown) =>
        {
            game_action = action::Action::MoveDown;
        }
        Some(pancurses::Input::KeyLeft) =>
        {
            game_action = action::Action::MoveLeft;
        }
        Some(pancurses::Input::KeyRight) =>
        {
            game_action = action::Action::MoveRight;
        }
        Some(pancurses::Input::Character(' '))=>
        {
            game_action = action::Action::Pass;
        }
        Some(pancurses::Input::Character('i'))=>
        {
            game_action = action::Action::Inventory;
        }
        Some(pancurses::Input::Character(_)) =>
        {
            game_action = action::Action::EndGame;
        }
	     Some(_) =>
        {
            game_action = action::Action::Invalid;
	     }
	     None => () // Do nothing.
    }

    game_action // Return game_action
}
