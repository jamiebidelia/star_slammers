mod rng
{

    struct PseudoRandom
    {
        seed      : u32,
        constant  : u32,
    }

    struct DiceRoll
    {
        num_dice  : u32,
        num_sides : u32,
        modifier  : i32,
    }

    impl PseudoRandom
    {
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

        fn roll_dice(&mut self, dr : &DiceRoll) -> i32
        {
            let mut total : i32 = 0;

            for i in 0 .. dr.num_dice
            {
                total = total + self.rand_in_range(1, dr.num_sides) as i32;
            }

            return total + dr.modifier;
        }
    }
}
