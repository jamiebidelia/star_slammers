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
