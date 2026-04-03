use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use cargo_util_terminal::Shell;

use crate::util::CargoResult;

pub struct GlobalContext {
    shell: RefCell<Shell>,
    cwd: PathBuf,
    env: BTreeMap<OsString, OsString>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        Self {
            shell: RefCell::new(Shell::new()),
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            env: std::env::vars_os().collect(),
        }
    }

    pub fn shell(&self) -> RefMut<'_, Shell> {
        self.shell.borrow_mut()
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub fn get_env_os(&self, key: impl AsRef<OsStr>) -> Option<&OsStr> {
        self.env.get(key.as_ref()).map(OsString::as_os_str)
    }

    pub fn get<T: Default>(&self, _key: &str) -> CargoResult<T> {
        Ok(Default::default())
    }
}
