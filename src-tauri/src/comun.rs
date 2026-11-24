//! Código que comparten `abrir_enlace` y `rust_server_manager`.

use std::ffi::OsStr;
use std::process::Command;

// Importación específica para Windows (ocultar ventanas de consola)
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// FLAG de Windows: CREATE_NO_WINDOW (sin ventana de consola visible).
#[cfg(target_os = "windows")]
const SIN_VENTANA: u32 = 0x08000000;

/// Crea un `Command` que en Windows no abre ventana de consola.
/// Equivale a encadenar `.creation_flags(SIN_VENTANA)` en cada comando.
pub fn comando_oculto(programa: impl AsRef<OsStr>) -> Command {
    #[allow(unused_mut)]
    let mut comando = Command::new(programa);
    #[cfg(target_os = "windows")]
    comando.creation_flags(SIN_VENTANA);
    comando
}
