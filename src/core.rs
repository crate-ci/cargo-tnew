use std::fmt;
use std::path::Path;

use crate::util::{CargoResult, GlobalContext};

pub struct Edition(&'static str);

impl Edition {
    pub const LATEST_STABLE: Self = Self("2024");
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Workspace;

impl Workspace {
    pub fn new(manifest_path: &Path, gctx: &GlobalContext) -> CargoResult<Self> {
        let mut metadata = cargo_metadata::MetadataCommand::new();
        metadata.manifest_path(manifest_path);
        metadata.current_dir(gctx.cwd());
        metadata.no_deps();
        metadata.exec()?;
        Ok(Self)
    }
}
