mod server_off;
mod server_checking_progress;
mod server_ok;

pub use server_checking_progress::*;
pub use server_off::*;
pub use server_ok::login::*;
pub use server_ok::register::*;

pub const MINIMAL_USERNAME_LENGTH: usize = 4;
pub const MINIMAL_PASSWORD_LENGTH: usize = 4;