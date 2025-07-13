// TODO: jsut make some build script that autogenerates this file

mod home;
pub use home::FateDecider;

mod auth;
pub use auth::{Login, Register};

mod not_found;
pub use not_found::PageNotFound;
