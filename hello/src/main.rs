
///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;

extern crate hello_lib;

///Optionally oad each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greetings::{default_greeting, french, spanish};
use hello_lib::greeting_from_lib;

use crate::how_you_hold_data_for_operations::{
    derived::user_defined::Comp, primitives::scalar_types::literals::compare,
};
use how_you_hold_data_for_operations::primitives::scalar_types::literals::run;

use how_you_hold_data_for_operations::primitives::compound_types::array::multiplier;
fn main() {
    println!("Hello, world!");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", greeting_from_lib());

    // how_you_hold_data_for_operations::primitives::scalar_types::literals::run();
    run();
    run();

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    multiplier(&[
        arr[0] as f64,
        arr[1] as f64,
        arr[2] as f64,
        arr[3] as f64,
        arr[4] as f64,
    ]);

    println!("{}", compare(10, 20, Comp::Greater));
    println!("{}", compare(10, 20, Comp::Lesser));
    println!("{}", compare(10, 20, Comp::Equals))

    // // Variables can be type annotated
    // let logical = true;

    // let a_float = 1.0; //Regular annotation
    // let an_integer = 5i32;

    // // Or a default will be used
    // let default_float = 3.0;
    // let default_integer = 7;
    hello_lib::run9();
}
