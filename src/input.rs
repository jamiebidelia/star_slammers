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


extern crate pancurses;

use crate::action;


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
