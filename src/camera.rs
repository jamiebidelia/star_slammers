extern crate pancurses;
use crate::creature;
use crate::tile_map;

pub struct Camera
{
    x_pos : i32,  // Position of the left-hand boundary of the camera.
    y_pos : i32,  // Position of the right-hand boundary of the camera.
    
    // Camera positions must be signed to support centering
    // Maps that are smaller than the camera's size.
}

impl Camera
{
    pub fn new() -> Camera
    {
        Camera {x_pos : 0, y_pos : 0}
    }
    
    pub fn get_x_pos(&self) -> &i32
    {
        &self.x_pos
    }

    pub fn set_x_pos(&mut self, new_x_pos : i32)
    {
        self.x_pos = new_x_pos;
    }

    pub fn get_y_pos(&self) -> &i32
    {
        &self.y_pos
    }

    pub fn set_y_pos(&mut self, new_y_pos : i32)
    {
        self.y_pos = new_y_pos;
    }
}// End Camera Implementation.


pub fn update_camera(game_camera : &mut Camera,
                     game_window : &pancurses::Window,
                     actor       : &creature::Creature,
                     game_map    : &tile_map::TileMap)
{
    // The camera's position is the top-left corner of the viewable screen.

    // Find the camera bounds.
    let map_start_x = game_window.get_beg_x();
    let map_start_y = game_window.get_beg_y();

    let map_end_x = get_map_end_x(&game_window);
    let map_end_y = get_map_end_y(&game_window);

    let camera_length  = map_end_x - map_start_x;
    let camera_height  = map_end_y - map_start_y;

    let camera_midpoint_x = camera_length  / 2;
    let camera_midpoint_y = camera_height  / 2;

    let camera_offset_x = *actor.get_x_pos() as i32 - camera_midpoint_x;
    let camera_offset_y = *actor.get_y_pos() as i32 + camera_midpoint_y;

    let mut small_x = false;
    let mut small_y = false;

    // If the map is not as long as our camera's length, then center the map in our horizontal.
    if camera_length >= *game_map.get_x_size() as i32
    {
        game_camera.set_x_pos( (*game_map.get_x_size() / 2) as i32 - camera_midpoint_x );
        small_x = true;
    }

    // If the map is not as tall as our camera's height, then center the map in our vertical.
    if camera_height >= *game_map.get_y_size() as i32
    {
        game_camera.set_y_pos( (*game_map.get_y_size() / 2) as i32 + camera_midpoint_y );
        small_y = true;
    }
    
    // Only track the camera along axes where there is more scrolling to do.
    if !small_x
    {
        // Camera offset is within x bounds, use the offset.
        if (camera_offset_x >= 0) &&
            (camera_offset_x + camera_length < *game_map.get_x_size() as i32)
        {
            game_camera.set_x_pos(camera_offset_x);
        }
        // Camera offset is too far left, clamp to the leftend of the map.
        else if camera_offset_x < 0
        {
            game_camera.set_x_pos(0);
        }
        // Camera offset is too far right, clamp to the right end of the map.
        else
        {
            // We subtract the camera length and add 1 because the full camera length
            // Would take us into the map border.
            game_camera.set_x_pos( *game_map.get_x_size() as i32 - camera_length + 1 );
        }

    } // End if not small_x

    if !small_y
    {
        // Camera offset is within y bounds, use the offset.
        if (camera_offset_y - camera_height >= 0) &&
            (camera_offset_y < *game_map.get_y_size() as i32)
        {
            game_camera.set_y_pos(camera_offset_y);
        }
        // Camera offset is too far down, clamp to the bottom end of the map.
        else if camera_offset_y - camera_height < 0
        {
            game_camera.set_y_pos(camera_height - 2);
        }
        // Camera offset is too far up, clamp to the top end of the map.
        else
        {
            game_camera.set_y_pos(*game_map.get_y_size() as i32 - 1); // Subtract 1 here because y_size is one past the max height of the map.
        }
    } // End if not small_y
}


fn get_map_end_x(game_window : &pancurses::Window) -> i32
{
    const MAP_X_PROPORTION : f32 = 0.60;
    
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let map_start_x = start_x;
    
    // Compute the ending point for map X, and make it an Integer.
    let map_end_x = (end_x - map_start_x) as f32;
    let map_end_x = MAP_X_PROPORTION * map_end_x;

    map_end_x.floor() as i32
}

fn get_map_end_y(game_window : &pancurses::Window) -> i32
{
    const MAP_Y_PROPORTION : f32 = 0.75;
    
    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();

    let map_start_y = start_y;

    // Compute the ending point for map Y, and make it an Integer.
    let map_end_y = (end_y - map_start_y) as f32;
    let map_end_y = MAP_Y_PROPORTION * map_end_y;

    map_end_y.floor() as i32
}

fn get_console_start_x(game_window : &pancurses::Window) -> i32
{
    game_window.get_beg_x() + 1
}

fn get_console_end_x(game_window : &pancurses::Window) -> i32
{
    get_map_end_x(game_window)
}

fn get_console_start_y(game_window : &pancurses::Window) -> i32
{
    get_map_end_y(game_window)
}

fn get_console_end_y(game_window : &pancurses::Window) -> i32
{
    game_window.get_max_y() - 2
}


pub fn draw_screen(game_window      : &pancurses::Window,
               game_camera          : &Camera,
               game_map             : &tile_map::TileMap,
               creatures_on_map     : &Vec<creature::Creature>,
               mut console_buffer   : &mut Vec<String>)
{
    
    // The max values it gives are not printable.  So we need to
    // Subtract 1 from each to reach our last index.  Be mindful of that.
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();

    let map_start_x = start_x;
    let map_start_y = start_y;
 
    let map_end_x = get_map_end_x(&game_window);
    let map_end_y = get_map_end_y(&game_window);

    if pancurses::can_change_color()
    {
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
        game_window.color_set(1);
    }
    
    
    // Clear and redraw the screen.
    //game_window.clear();
    game_window.erase();
    game_window.refresh();

    // Redraw the HUD elements console:
    draw_console(&game_window, &console_buffer);


    // Draw the borders for the User Interface.
    for y in start_y .. end_y
    {
        for x in start_x .. end_x
 	{

            // The if below draws the map border.
            if y == map_end_y || x == map_end_x
            {
                game_window.mvprintw(y, x, "x");
            }
            
            // The if below draws the game border.
	    if (y == start_y || y == end_y-1) || (x == start_x || x == end_x-1)  
	    {
		game_window.mvprintw(y, x, "x");
            }
	}
    }

    // Draw the camera's view of the Tile Map.

    // In the screen coordinates, y counts up as we move down the screen.
    // We need grab the drawable area of the screen.

    const CAMERA_OFFSET : i32 = 1;
    
    let camera_screen_start_x = map_start_x + CAMERA_OFFSET;
    let camera_screen_start_y = map_start_y + CAMERA_OFFSET;

    let camera_screen_stop_x = map_end_x;
    let camera_screen_stop_y = map_end_y;

    let camera_len    = (camera_screen_stop_x - camera_screen_start_x) as u32;
    let camera_height = (camera_screen_stop_y - camera_screen_start_y) as u32;
    
    //camera position is map position format.

    // y and x are the coordinates in screen space that we will draw to.
    // To find out what to draw there, we have to translate to map coordinates.
    for y in camera_screen_start_y .. camera_screen_stop_y
    {

        // if the camera is at position 20,20, and the camera screen starts at y = 1,
        // then we need to subtract. 20 - 1 = 19. So we need to
        // The next map position is 21.  And the camera will be at y = 2.
        // then 21 - 2 = 19.
        
        // As y goes up in screen coordinates, our position moves down in map coordinates.
        let map_y = -1 * y + CAMERA_OFFSET + *game_camera.get_y_pos();
        
        for x in camera_screen_start_x .. camera_screen_stop_x
        {
            // As x goes up in screen coordinates, our position moves right in map coordinates.
            let map_x = *game_camera.get_x_pos() + x - CAMERA_OFFSET;


            // FIXME:  Make sure that we do not attempt to draw if the drawing index is negative etc.
            
            
            // Make sure that if the camera view looks outside the
            // game map, we don't attempt to draw game tiles there.
            let mut ok_to_draw = true;

            // If the map coordinates we have are outside the bounds of the map,
            // Do not attempt to draw there.
            if map_x < 0 || map_x >= *game_map.get_x_size() as i32
            {
                ok_to_draw = false;
            }

            if map_y < 0 || map_y >= *game_map.get_y_size() as i32
            {
                ok_to_draw = false;
            }

            if ok_to_draw
            {
                game_window.mvprintw(y,
                                     x,
                                     game_map.get_map()[map_x as usize]
                                     [map_y as usize].
                                     get_image().to_string());
            }
        }
    }


            //Pretend creature is at x 28, y 28.
            //Camera is at x 24, y 24.
            //Creature is at x28 y 20.
            //Creature is offset from x by 4, and should be 4 spaces right (add 4).
            //Creature is offset from y by 4, and should be 4 spaces up (subtract 4)
            //Both of these positions should be offset by the map offset.


            //y pos is 5.
            //cy is 10.
            // 5 + (10 - 5);

    // Draw all creatures that are visible to the camera.
    for creature in creatures_on_map
    {


        let screen_x : i32 = *creature.get_x_pos() as i32 - *game_camera.get_x_pos() as i32 + CAMERA_OFFSET;
        let screen_y : i32 = *game_camera.get_y_pos() as i32 - *creature.get_y_pos() as i32 + CAMERA_OFFSET;

        if (screen_x >= camera_screen_start_x) &&
           (screen_x <= camera_screen_stop_x) &&
           (screen_y >= camera_screen_start_y) &&
           (screen_y <= camera_screen_stop_y)
        {
            game_window.mvprintw(screen_y, screen_x, creature.get_image().to_string());
            
        }

    } // End for creature in creatures_on_map.
            
}


pub fn draw_console(game_window    : &pancurses::Window,
                console_buffer : &Vec<String>)
{
    let mut cur_message_line = get_console_end_y(game_window);
    for s in console_buffer.iter().rev()
    {
        game_window.mvprintw(cur_message_line,
                             get_console_start_x(game_window),
                             s);
        
        cur_message_line = cur_message_line - 1;
        if cur_message_line == get_console_start_y(game_window)
        {
            break;
        }
    }
}
