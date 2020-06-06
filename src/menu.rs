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


pub struct MenuNode<'a>
{
    children : Option<&'a [&'a MenuNode<'a>]>,
    text     : &'a str,
    visible  : bool
}

impl MenuNode<'_>
{
   pub fn get_children(&self)->Option<&'_ [&'_ MenuNode<'_>]>
   {
      self.children
   }
   
   pub fn get_text(&self) -> &str
   {
      self.text
   }
}

const fn init_menu_head<'a>(the_children : &'a [&'a MenuNode<'a>],
                            the_text     : &'a str,
                            is_visible   : bool) -> MenuNode<'a>
{
    MenuNode
    {
      children     : Some(the_children),
      text         : the_text,
      visible      : is_visible,
    }
                              
}

const fn init_menu_node<'a>(the_text   : &'a str,
                            is_visible : bool) -> MenuNode<'a>
{
    MenuNode
    {
      children     : None,
      text         : the_text,
      visible      : is_visible,
    }
}

                                                   
static NEW_GAME_MENU   : MenuNode = init_menu_node("New Game",
                                                   true);
                                                   
static LOAD_MENU       : MenuNode = init_menu_node("Load Game",
                                                   true);

static RESUME_MENU     : MenuNode = init_menu_node("Resume Game",
                                                   true);
                                                   
static QUIT_MENU       : MenuNode = init_menu_node("Quit",
                                                   true);
                                                   
static MAIN_CHILDREN   : [&MenuNode; 4] = [&NEW_GAME_MENU,
                                           &LOAD_MENU,
                                           &RESUME_MENU,
                                           &QUIT_MENU];
                                                   
pub static MAIN_MENU   : MenuNode = init_menu_head(&MAIN_CHILDREN,
                                                   "Main Menu",
                                                   false);