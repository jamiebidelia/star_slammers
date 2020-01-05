
const CONSOLE_BUFFER_MAX_SIZE : usize = 256;

pub fn post_to_console(text           : String,
                   game_window    : &pancurses::Window,
                   console_buffer : &mut Vec<String>)
{
    let mut the_string : String = " * ".to_string();
    the_string.push_str(text.as_str());

    console_buffer.push(the_string);
}
