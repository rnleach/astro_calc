#[macro_use] 
extern crate lazy_static;

// Public export modules
pub mod astro_time;

// Internal use only modules
mod util; // Utilities

// Library wide symbols to export
pub use util::AstroAlgorithmsError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
