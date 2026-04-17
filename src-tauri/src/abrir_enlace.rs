use std::process::Command;

// Esta es la única función que necesitas, no hay funciones adicionales
#[tauri::command]
pub fn abrir_enlace(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd")
        .args(&["/C", "start", &url])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("No se pudo abrir el enlace".into())
    }
}
