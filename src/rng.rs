#![doc(html_no_source)]


mod rng
{
    //! The rng module handles all data structures and methods for generating random numbers.
    
    /// Pseudorandom represents a random number generator.  It currently implements a Linear Congruence algorithm for generating random numbers.
    struct PseudoRandom
    {
        seed      : u32,
        constant  : u32,
    }

    /// DiceRoll represents the semantics of rolling a pool of dice.  It can represent a pool such at "4d6+5" well, but cannot represent complex pools like "4d6+1d4"
    struct DiceRoll
    {
        num_dice  : u32,
        num_sides : u32,
        modifier  : i32,
    }

    impl PseudoRandom
    {
        /// rand_in_range takes in bounds, and produces a random number between them.
        /// use of this function modifies the seed.
        fn rand_in_range(&mut self, lower_bound : u32, upper_bound : u32) -> u32
        {
            //If the bounds were 4-5, upper-lower is only 1.  We have to add 1 to get an inclusive range of 2 numbers.
            //Also, we have to pad by 1 to account for the modulus being one value short of that range.
            let mod_value = upper_bound - lower_bound + 2;

            //The random number will be between 0 and mod_value - 1.
            self.seed = (self.seed + self.constant) % mod_value;

            //Shift the random number up to the lower bound to make sure we get the correct range.
            return self.seed + lower_bound;
        }

        /// roll_dice takes in a DiceRoll structure, and produces a total outcome in the form of an i32.
        /// use of this function modifies the seed.
        fn roll_dice(&mut self, dr : &DiceRoll) -> i32
        {
            let mut total : i32 = 0;

            for i in 0 .. dr.num_dice
            {
                total = total + self.rand_in_range(1, dr.num_sides) as i32;
            }

            return total + dr.modifier;
        }

        /// roll_skill_check takes in three stats to use, and a defense to roll against.  It produces a boolean determining if the skill_check succeeded or failed.
        /// use of this function modifies the seed.
        fn roll_skill_check(&mut self, stat1 : u32, stat2 : u32, stat3 : u32, defense : u32) -> bool
        {
            let target_num = stat1 + stat2 + stat3 - defense;
            let mut ret_val = false;

            if target_num > 0
            {
                if self.rand_in_range(1, 100) <= (target_num as u32)
                {ret_val = true;}
            }


            return ret_val;
        }
    }
}
