use std::sync::Mutex;

#[cfg(not(feature = "once_cell"))]
static GLOBAL_FILE_RESET: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();
#[cfg(feature = "once_cell")]
static GLOBAL_FILE_RESET: once_cell::sync::OnceCell<Mutex<bool>> = once_cell::sync::OnceCell::new();

#[cfg(not(feature = "once_cell"))]
static SEPARATE_FILES_RESET: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();
#[cfg(feature = "once_cell")]
static SEPARATE_FILES_RESET: once_cell::sync::OnceCell<Mutex<bool>> =
    once_cell::sync::OnceCell::new();
