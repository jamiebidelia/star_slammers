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

use crate::menu;
use crate::menuaction;


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
   
  
}

pub fn draw_menu(game_window  : &pancurses::Window,
                 menu_cursor  : &usize)
{
   let start_x = game_window.get_beg_x();
   let end_x   = game_window.get_max_x();

   let start_y = game_window.get_beg_y();
   let end_y   = game_window.get_max_y();

   // Indent 40% from the left.
   let menu_x_beginning = start_x + (((end_x - start_x) as f32 * 0.4) as i32);
   
   // Indent 50% from the top.
   let menu_y_beginning = start_y + (((end_y - start_y) as f32 * 0.5) as i32);
   
   draw_menu_children(game_window,
                      menu_cursor,
                      menu_y_beginning,
                      menu_x_beginning);
   
}

fn draw_menu_children(game_window   : &pancurses::Window,
                      menu_cursor   : &usize,
                      y_pos         : i32,
                      x_pos         : i32)
{

    let cursor = "-->";
    let mut y_scroller = y_pos;

    let main_menu = menu::create_main_menu();
    
    if (main_menu.len()) != 0
    {
        for menu_item in &main_menu
        {
            game_window.mvprintw(y_scroller, x_pos, menu_item.get_text());

            
            let menu_pointer = &main_menu[*menu_cursor];
            
            if menu_item == menu_pointer
            {
                game_window.mvprintw(y_scroller, x_pos - 4 , cursor);
            }
            
            y_scroller += 3;
            
        }
    }
}


pub fn process_keyboard(game_window : &pancurses::Window) ->
    menuaction::MenuAction
{
    let mut menu_action = menuaction::MenuAction::Invalid;
    
    match game_window.getch()
    {
        Some(pancurses::Input::KeyResize) =>
        {
            menu_action = menuaction::MenuAction::Resize;
        }
        Some(pancurses::Input::KeyUp) =>
        {
            menu_action = menuaction::MenuAction::CursorUp;
        }
        Some(pancurses::Input::KeyDown) =>
        {
            menu_action = menuaction::MenuAction::CursorDown;
        }
        Some(pancurses::Input::Character('\n'))=>
        {
            menu_action = menuaction::MenuAction::Select;
        }
	     Some(_) =>
        {
            menu_action = menuaction::MenuAction::Invalid;
	     }
	     None => () // Do nothing.
    }


    
    
    menu_action // Return menu_action
}
