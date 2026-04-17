mod abrir_enlace;
mod rust_server_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            rust_server_manager::instalar_steam,
            rust_server_manager::instalar_rust,
            rust_server_manager::instalar_oxide,
            rust_server_manager::eliminar_todo,
            rust_server_manager::crear_iniciador_servidor,
            rust_server_manager::abrir_carpeta_plugins,
            rust_server_manager::iniciar_servidor,
            rust_server_manager::apagar_servidor,
            abrir_enlace::abrir_enlace,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
