//! # Art Testing library
//!
//! A library for modeling artistic concepts.

//these are re exports to make this lib easier to use
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

///Modules that work with the colors
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
        Grey,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1,c2){
            (PrimaryColor::Red,PrimaryColor::Yellow)
            | (PrimaryColor::Yellow,PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Red,PrimaryColor::Blue) 
            | (PrimaryColor::Blue,PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Blue,PrimaryColor::Yellow)
            |(PrimaryColor::Yellow,PrimaryColor::Blue) => SecondaryColor::Green,
            _ => SecondaryColor::Grey,
        }
    }
}