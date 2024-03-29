
/*Attribute
--
name  : String;     // Name of the Attribute
desc  : String;     // A short description
value : u8;         // Current value of Attribute
max   : u8;         // Max allowed value
min   : u8;          // Min allowed value

increment() -> bool // +1 to Attribute if under max
decrement() -> bool // -1 to Attribute if above min
 */

/// Attribute composes the base unit of an attribute slider.
pub struct Attribute
{
    name  : String,
    desc  : String,
    value : u8,
    max   : u8,
    min   : u8,
}

impl Attribute
{
    /// increment checks to see if the attribute is already at max.
    /// If it is not, we increment the value; if it is, we do not.
    /// We return whether or not the increment was successful.
    pub fn increment(&mut self) -> bool
    {
        let mut ret_val = true;
        if self.value == self.max
        {
            ret_val = false;
        }
        else
        {
            self.value = self.value + 1;
        }

        ret_val
    }

    /// decrement checks to see if the attribute is already at min.
    /// If it is not, we decrement the value; if it is, we do not.
    /// We return whether or not the decrement was successful.
    pub fn decrement(&mut self) -> bool
    {
        let mut ret_val = true;

        if self.value == self.min
        {
            ret_val = false;
        }
        else
        {
            self.value = self.value - 1;
        }

        ret_val
    }


    /// new is a simple constructor for the Attribute struct.
    pub fn new(name  : String,
               desc  : String,
               value : u8,
               max   : u8,
               min   : u8) -> Attribute
    {
        Attribute
        {
            name,
            desc,
            value,
            max,
            min
        }
    }
}

