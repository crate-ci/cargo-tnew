mod cargo_new;
mod utils;

pub mod prelude {
    pub use crate::utils::CargoCommandExt;
    pub use crate::utils::CargoProjectExt;
    pub use cargo_test_support::prelude::*;
    pub use cargo_test_support::snapbox;
}
