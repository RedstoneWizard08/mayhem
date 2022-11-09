pub mod index;
pub mod login;
pub mod register;

pub use index::index as handle_index;
pub use login::login as handle_login;
pub use register::register as handle_register;
