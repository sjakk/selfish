mod uterm;
mod consoles;
mod text;


pub type FontCharType = u16;

pub mod prelude{
    pub use crate::text::*;
    pub use crate::consoles::*;
    pub use crate::FontCharType;
}
