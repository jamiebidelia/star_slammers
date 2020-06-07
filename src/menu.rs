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

extern crate lazy_static;


pub struct MenuNode<'a>
{
    children : &'a Vec<&'a MenuNode<'a>>,
    text     : &'a str,
    visible  : bool
}

impl MenuNode<'_>
{
   pub fn get_children(&self)->&Vec<&MenuNode>
   {
      &self.children
   }
   
   pub fn get_text(&self) -> &str
   {
      self.text
   }
}

impl std::cmp::PartialEq for MenuNode<'_>
{
    fn eq(&self, other : &MenuNode<'_>) -> bool
    {
        return
        if self.text == other.text
        {
            true
        }
        else
        {
            false
        };
            
    }
}

const fn init_menu<'a>(the_children : &'a Vec<&'a MenuNode<'a>>,
                       the_text     : &'a str,
                       is_visible   : bool) -> MenuNode<'a>
{
    MenuNode
    {
      children     : the_children,
      text         : the_text,
      visible      : is_visible,
    }
                              
}

// This code here is INCREDIBLY ugly.  All it does is uses lazy_static to
// Let us have global static menus for the main menu.  Since the children
// Are vectors, we have to initialize them at runtime instead of compile time.
// But with Rust, I know of no other way to have globally available data.

// This is probably a problem caused by ignorance and not a langauge problem.
lazy_static::lazy_static!
{
    static ref NEW_GAME_CHILDREN : Vec<&'static MenuNode<'static>> = vec!();
    static ref NEW_GAME_MENU     : MenuNode<'static> = init_menu(&NEW_GAME_CHILDREN,
                                                        "New Game",
                                                        true);
    static ref LOAD_CHILDREN     : Vec<&'static MenuNode<'static>> = vec!();
    static ref LOAD_MENU         : MenuNode<'static> = init_menu(&LOAD_CHILDREN,
                                                        "Load Game",
                                                        true);
    static ref RESUME_CHILDREN   : Vec<&'static MenuNode<'static>> = vec!();
    static ref RESUME_MENU       : MenuNode<'static> = init_menu(&RESUME_CHILDREN,
                                                        "Resume Game",
                                                        true);
    static ref QUIT_CHILDREN     : Vec<&'static MenuNode<'static>> = vec!();
    static ref QUIT_MENU         : MenuNode<'static> = init_menu(&QUIT_CHILDREN,
                                                        "Quit",
                                                        true);

    static ref MAIN_CHILDREN : Vec<&'static MenuNode<'static>> =
        vec!(&NEW_GAME_MENU,
             &LOAD_MENU,
             &RESUME_MENU,
             &QUIT_MENU);

    pub static ref MAIN_MENU   : MenuNode<'static> = init_menu(&MAIN_CHILDREN,
                                                      "Main Menu",
                                                      false);
    
}

