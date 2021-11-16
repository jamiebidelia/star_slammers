--   _____ _                _____ _                                                      
--  / ____| |              / ____| |                                        _            
-- | (___ | |_ __ _ _ __  | (___ | | __ _ _ __ ___  _ __ ___   ___ _ __ ___(_)           
--  \___ \| __/ _` | '__|  \___ \| |/ _` | '_ ` _ \| '_ ` _ \ / _ \ '__/ __|             
--  ____) | || (_| | |     ____) | | (_| | | | | | | | | | | |  __/ |  \__ \_            
-- |_____/ \__\__,_|_|    |_____/|_|\__,_|_| |_| |_|_| |_| |_|\___|_|  |___(_)           
--  _____        __ _       _ _                     _                 _                  
-- |_   _|      / _(_)     (_) |           /\      | |               | |                 
--   | |  _ __ | |_ _ _ __  _| |_ ___     /  \   __| |_   _____ _ __ | |_ _   _ _ __ ___ 
--   | | | '_ \|  _| | '_ \| | __/ _ \   / /\ \ / _` \ \ / / _ \ '_ \| __| | | | '__/ _ \
--  _| |_| | | | | | | | | | | ||  __/  / ____ \ (_| |\ V /  __/ | | | |_| |_| | | |  __/
-- |_____|_| |_|_| |_|_| |_|_|\__\___| /_/    \_\__,_| \_/ \___|_| |_|\__|\__,_|_|  \___|


-- Constants that specify what each ASCII character means.
FLOOR = "."
WALL  = "0"
ROCK  = "X"

function Construct_Map(x_max, y_max, rock_rate)
   -- Construct the Map Grid as a 2D array filled with 0's.
   local map_grid = {}
   
   for y = 1, y_max do
      map_grid[y] = {}
      for x = 1, x_max do
         
         local rock_roll = math.random(1, 100)
         
         if (rock_roll > rock_rate) then
            map_grid[y][x] = ROCK
         else
            map_grid[y][x] = FLOOR
         end
      end
   end
   
   return map_grid
end

--Print_Map expects map_grid to be a 2d array in the form [y][x].
function Print_Map(map_grid, x_max, y_max)
   
   io.write("Printing Map Grid:\n")
   
   for y = 1, y_max do
      for x = 1, x_max do
         io.write(map_grid[y][x])
      end
      io.write("\n")
   end
end

function Eval_Neighborhood(map_grid, x, y, x_max, y_max)
   --"In cellular automata, the Moore neighborhood is defined on a two-dimensional
   -- square lattice and is composed of a central cell and the eight cells
   -- which surround it" --Wikipedia.
   
   -- If the location given is invalid, give a score of -1 to indicate error
   if (x < 1) or (x > x_max) or (y < 1) or (y > y_max) then
      return -1
   end
   
   local use_left  = true   -- "left" is x-1
   local use_right = true   -- "right" is x+1
   local use_up    = true   -- "up" is y-1
   local use_down  = true   -- "down" is y+1
   
   -- Determine which directions are within the bounds of the map
   if (x - 1) < 1 then
      use_left = false
   end
   
   if (x + 1) > x_max then
      use_right = false
   end
   
   if (y - 1) < 1 then
      use_up = false
   end
   
   if (y + 1) > y_max then
      use_down = false
   end
   
   local score = 0
   
   -- Grab Self Data.
   if (map_grid[y][x] == ROCK) then
      score = score + 1
   end
   
   -- Grab Left.
   if use_left and map_grid[y][x-1] == ROCK then
      score = score + 1
   end
   
   -- Grab Right.
   if use_right and map_grid[y][x+1] == ROCK then
      score = score + 1
   end
   
   -- Grab Up.
   if use_up and map_grid[y-1][x] == ROCK then
      score = score + 1
   end
   
   -- Grab Down.
   if use_down and map_grid[y+1][x] == ROCK then
      score = score + 1
   end
   
   -- Grab Left & Up.
   if use_left and use_up and map_grid[y-1][x-1] == ROCK then
      score = score + 1
   end
   
   -- Grab Left & Down.
   if use_left and use_down and map_grid[y+1][x-1] == ROCK then
      score = score + 1
   end
   
   -- Grab Right & Up.
   if use_right and use_up and map_grid[y-1][x+1] == ROCK then
      score = score + 1
   end
   
   -- Grab Right & Down.
   if use_right and use_down and map_grid[y+1][x+1] == ROCK then
      score = score + 1
   end
   
   return score
   
end

function Eval_Map(map_grid, x_max, y_max)
   -- Evaluate the Map Grid.
   local eval_grid = {}
   
   for y = 1, y_max do
      eval_grid[y] = {}
      for x = 1, x_max do
         eval_grid[y][x] = Eval_Neighborhood(map_grid, x, y, x_max, y_max)
      end
   end
   
   return eval_grid
   
end

function Transform_Map(map_grid, eval_grid, threshold, x_max, y_max)
   
   for y = 1, y_max do
      for x = 1, x_max do
         if eval_grid[y][x] >= threshold then
            map_grid[y][x] = ROCK
         else
            map_grid[y][x] = FLOOR
         end
      end
   end
   
end

function Get_Neighbor_Number(map_grid, x, y, x_max, y_max)
   
   -- If the location given is invalid, give a score of -1 to indicate error
   if (x < 1) or (x > x_max) or (y < 1) or (y > y_max) then
      return -1
   end
   
   local use_left  = true   -- "left" is x-1
   local use_right = true   -- "right" is x+1
   local use_up    = true   -- "up" is y-1
   local use_down  = true   -- "down" is y+1
   
   -- Determine which directions are within the bounds of the map
   if (x - 1) < 1 then
      use_left = false
   end
   
   if (x + 1) > x_max then
      use_right = false
   end
   
   if (y - 1) < 1 then
      use_up = false
   end
   
   if (y + 1) > y_max then
      use_down = false
   end
   
   -- Start with 1 as we are always a neighbor to ourself.
   local neighbor_count = 1 
   
   -- Grab Left.
   if use_left then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Right.
   if use_right then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Up.
   if use_up then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Down.
   if use_down then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Left & Up.
   if use_left and use_up then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Left & Down.
   if use_left and use_down then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Right & Up.
   if use_right and use_up then
      neighbor_count = neighbor_count + 1
   end
   
   -- Grab Right & Down.
   if use_right and use_down then
      neighbor_count = neighbor_count + 1
   end
   
   return neighbor_count
   
end

function Add_Walls(map_grid, x_max, y_max)
   
   local new_grid = {}
   
   for y = 1, y_max do
      new_grid[y] = {}
      for x = 1, x_max do
         
         local score = Eval_Neighborhood(map_grid, x, y, x_max, y_max)
         local neighbor_count = Get_Neighbor_Number(map_grid, x, y, x_max, y_max)
         
         if score < neighbor_count and map_grid[y][x] == ROCK then
            new_grid[y][x] = WALL
         else
            new_grid[y][x] = map_grid[y][x]
         end
      end
   end
   
   return new_grid
   
end

-- Learning to create a cave using cellular automata.

-- Terrain types are:
-- Floor: "."
-- Wall: "0"
-- Rock: "X"

-- Let x_max be the maximum number of columns.
-- Let y_max be the maximum number of rows.
x_max = 50
y_max = 50

-- Let rock_rate be the percent of a tile initially being a rock (0 to 100)
rock_rate = 50

-- Threshold tells us at what evaluation score to turn a tile into Rock.
threshold = 5

-- Cellular Automata Rule Application Iterations: how many times we evaluate and then transform the map.
iterations = 2

--If testing, use a constant seed; otherwise use one based off the clock.
--math.randomseed(10)
math.randomseed(os.time())

-- Construct the Map Grid.
map_grid = Construct_Map(x_max, y_max, rock_rate)


for i = 1, iterations do
   -- Evalute the Map Grid.
   eval_grid = Eval_Map(map_grid, x_max, y_max)
   
   -- Transform the Map Grid based on the results of the Eval Grid
   Transform_Map(map_grid, eval_grid, threshold, x_max, y_max)
end

map_grid = Add_Walls(map_grid, x_max, y_max)

Print_Map(map_grid, x_max, y_max)
