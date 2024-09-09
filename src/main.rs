use std::io::{stdin, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};



fn main()
{
    enable_raw_mode().unwrap();
    for b in stdin().bytes().into_iter()
    {
        let b = b.unwrap();   
        let c = b as char;
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        }
        else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }
        println!("{c}");
        if c == 'q'
        {
            disable_raw_mode().unwrap();
            break;
        }
    }
}