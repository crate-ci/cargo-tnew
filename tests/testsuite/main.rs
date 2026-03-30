mod cargo_tnew;
mod utils;

pub mod prelude {
    pub use crate::utils::CargoCommandExt;
    pub use crate::utils::CargoProjectExt;
    pub use cargo_test_support::prelude::*;
}
