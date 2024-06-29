pub mod interpreter;
pub mod lynch;
pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::setup::interpreter::ReadShebang;
}