// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green: ColorClassicStruct = ColorClassicStruct {
            name: String::from("green"),
            hex: String::from("#00FF00"),
        };

        // The stack is faster because all free memory is always contiguous.
        // Unlike heap, No list need to keep a list of all the free memory,
        // only one pointer to the current top of the stack.
        // Each byte in the stack tends to be reused very frequently
        // which means it tends to be mapped to the processor’s cache, making it very fast.
        // Therefore, I recommend using stack as long as you don’t need to use heap.

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
