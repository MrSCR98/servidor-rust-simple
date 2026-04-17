// Importaciones de librerías estándar y externas
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::env;
use std::fs;
use std::i32;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::ipc::Channel;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

// Importación específica para Windows (ocultar ventanas de consola)
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Obtiene la ruta base de la aplicación y concatena una subruta
/// Ejemplo: obtener_ruta_aplicacion("SERVER") devuelve "C:/ruta/al/ejecutable/RUST SERVER/SERVER"
fn obtener_ruta_aplicacion(sub_ruta: &str) -> PathBuf {
    // Obtenemos la ruta del ejecutable actual
    let ruta_ejecutable = env::current_exe().expect("No se pudo obtener el ejecutable");
    // Obtenemos el directorio padre (carpeta donde está el .exe)
    let directorio_raiz = ruta_ejecutable
        .parent()
        .expect("No se pudo obtener directorio padre");
    // Concatenamos la carpeta principal "RUST SERVER" y la subruta solicitada
    directorio_raiz.join("RUST SERVER").join(sub_ruta)
}

/// Descarga un archivo desde una URL usando PowerShell de Windows
/// Parámetros:
///   - url_origen: Dirección web del archivo a descargar
///   - ruta_destino: Ubicación donde se guardará el archivo localmente
async fn descargar_archivo(url_origen: &str, ruta_destino: &PathBuf) -> Result<(), String> {
    // Construimos el comando de PowerShell para descargar el archivo
    let script_powershell = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
        url_origen,
        ruta_destino.to_str().unwrap()
    );

    // Ejecutamos PowerShell sin mostrar ventana (creation_flags oculta la consola)
    let salida_comando = Command::new("powershell")
        .args(&["-Command", &script_powershell])
        .creation_flags(0x08000000) // FLAG: CREATE_NO_WINDOW (sin ventana visible)
        .output()
        .map_err(|error| format!("Error descargando: {}", error))?;

    // Verificamos si el comando se ejecutó correctamente
    if !salida_comando.status.success() {
        return Err(format!(
            "PowerShell error: {}",
            String::from_utf8_lossy(&salida_comando.stderr)
        ));
    }

    Ok(())
}

/// Verifica si SteamCMD está instalado comprobando la existencia del ejecutable
fn comprobar_steamcmd_instalado() -> bool {
    obtener_ruta_aplicacion("STEAMCMD/steamcmd.exe").exists()
}

/// Verifica si el servidor de Rust está instalado comprobando el ejecutable principal
fn comprobar_rust_instalado() -> bool {
    obtener_ruta_aplicacion("SERVER/RustDedicated.exe").exists()
}

/// 1. INSTALAR STEAMCMD
/// Descarga e instala SteamCMD en la carpeta correspondiente
#[tauri::command]
pub async fn instalar_steam(set_estado: Channel<String>) -> Result<(), String> {
    // Definimos las rutas necesarias para la instalación
    let directorio_steam = obtener_ruta_aplicacion("STEAMCMD");
    let ruta_archivo_zip = obtener_ruta_aplicacion("steamcmd.zip");

    // Determinamos si es instalación nueva o actualización
    let tipo_operacion = if comprobar_steamcmd_instalado() {
        "Actualizando"
    } else {
        "Instalando"
    };

    // Informamos al usuario del estado actual
    set_estado
        .send(format!("🔧 {} SteamCMD...", tipo_operacion))
        .unwrap();
    set_estado
        .send("📁 Paso 1/4: Preparando carpetas...".to_string())
        .unwrap();

    // Si existe una instalación anterior, la eliminamos para hacer limpieza
    if directorio_steam.exists() {
        fs::remove_dir_all(&directorio_steam).map_err(|error| error.to_string())?;
    }
    // Creamos el directorio de instalación
    fs::create_dir_all(&directorio_steam).map_err(|error| error.to_string())?;

    set_estado
        .send("⬇️ Paso 2/4: Descargando SteamCMD...".to_string())
        .unwrap();

    // Descargamos el archivo ZIP de SteamCMD desde los servidores de Valve
    descargar_archivo(
        "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip",
        &ruta_archivo_zip,
    )
    .await?;

    set_estado
        .send("📦 Paso 3/4: Descomprimiendo archivos...".to_string())
        .unwrap();

    // Descomprimimos el ZIP usando la herramienta tar nativa de Windows
    let resultado_descompresion = Command::new("tar")
        .args(&[
            "-xf",
            ruta_archivo_zip.to_str().unwrap(),
            "-C",
            directorio_steam.to_str().unwrap(),
        ])
        .creation_flags(0x08000000) // Sin ventana visible
        .output()
        .map_err(|error| format!("Error descomprimiendo: {}", error))?;

    // Verificamos que la descompresión fue exitosa
    if !resultado_descompresion.status.success() {
        return Err("Error al descomprimir".to_string());
    }

    // Eliminamos el archivo ZIP temporal para no ocupar espacio
    fs::remove_file(&ruta_archivo_zip).map_err(|error| error.to_string())?;

    set_estado
        .send("⚙️ Paso 4/4: Inicializando SteamCMD...".to_string())
        .unwrap();

    // Ejecutamos SteamCMD una vez para que se actualice e inicialice
    let ruta_ejecutable_steam = directorio_steam.join("steamcmd.exe");
    let _ = Command::new(&ruta_ejecutable_steam)
        .arg("+quit")
        .creation_flags(0x08000000) // Sin ventana visible
        .status()
        .map_err(|error| format!("Error ejecutando: {}", error))?;

    // Verificamos que el ejecutable existe y tiene contenido (tamaño > 0)
    let ejecutable_steam_final = directorio_steam.join("steamcmd.exe");
    if ejecutable_steam_final.exists() {
        let metadatos = fs::metadata(&ejecutable_steam_final).map_err(|error| error.to_string())?;
        if metadatos.len() > 0 {
            // Mensaje final según el tipo de operación realizada
            let mensaje_final = if tipo_operacion == "Actualizando" {
                "✅ SteamCMD actualizado correctamente"
            } else {
                "✅ SteamCMD instalado correctamente"
            };
            set_estado.send(mensaje_final.to_string()).unwrap();
            return Ok(());
        }
    }

    Err("❌ SteamCMD no se instaló correctamente".to_string())
}

/// 2. INSTALAR/ACTUALIZAR RUST
/// Descarga o actualiza el servidor de Rust usando SteamCMD
#[tauri::command]
pub async fn instalar_rust(set_estado: Channel<String>) -> Result<(), String> {
    // Verificación: SteamCMD debe estar instalado primero
    if !comprobar_steamcmd_instalado() {
        return Err("❌ Primero instala SteamCMD.".to_string());
    }

    // Preparación de rutas y variables
    let ruta_ejecutable_steam = obtener_ruta_aplicacion("STEAMCMD/steamcmd.exe");
    let directorio_servidor = obtener_ruta_aplicacion("SERVER");
    let ruta_ejecutable_rust = directorio_servidor.join("RustDedicated.exe");

    // Determinamos si es instalación nueva o actualización
    let accion_instalacion = if ruta_ejecutable_rust.exists() {
        "Actualizando"
    } else {
        "Instalando"
    };

    set_estado
        .send(format!("🎮 {} servidor de Rust...", accion_instalacion))
        .unwrap();
    set_estado
        .send("⏳ Esto puede tardar 5-10 minutos dependiendo de tu conexión...".to_string())
        .unwrap();

    // Creamos el directorio del servidor si no existe
    fs::create_dir_all(&directorio_servidor).map_err(|error| error.to_string())?;

    set_estado
        .send("🔄 Descargando archivos del servidor...".to_string())
        .unwrap();

    // Ejecutamos SteamCMD para descargar/actualizar Rust (App ID 258550)
    let mut instancia_proceso = Command::new(&ruta_ejecutable_steam)
        .args(&[
            "+login",
            "anonymous",
            "+force_install_dir",
            directorio_servidor.to_str().unwrap(),
            "+app_update",
            "258550", // ID de aplicación de Rust en Steam
            "validate",
            "+quit",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000) // Sin ventana visible
        .spawn()
        .map_err(|error| format!("Error ejecutando steamcmd: {}", error))?;

    // Esperamos a que SteamCMD termine la descarga
    let estado_final = instancia_proceso
        .wait()
        .map_err(|error| error.to_string())?;

    if !estado_final.success() {
        return Err("❌ Error al instalar Rust".to_string());
    }

    // Mensaje según el tipo de operación
    let mensaje_resultado = if accion_instalacion == "Actualizando" {
        "✅ Servidor de Rust actualizado correctamente"
    } else {
        "✅ Servidor de Rust instalado correctamente"
    };
    set_estado.send(mensaje_resultado.to_string()).unwrap();
    Ok(())
}

/// 3. INSTALAR OXIDE
/// Instala el framework Oxide (umod) para soportar plugins
#[tauri::command]
pub async fn instalar_oxide(set_estado: Channel<String>) -> Result<(), String> {
    // Jerarquía de comprobaciones: SteamCMD -> Rust -> Oxide
    if !comprobar_steamcmd_instalado() {
        return Err("❌ Primero instala SteamCMD.".to_string());
    }

    if !comprobar_rust_instalado() {
        return Err("❌ Primero instala Rust.".to_string());
    }

    let directorio_servidor = obtener_ruta_aplicacion("SERVER");
    let ruta_archivo_oxide = obtener_ruta_aplicacion("Oxide.Rust.zip");

    set_estado
        .send("⬇️ Descargando Oxide desde umod.org...".to_string())
        .unwrap();

    // Descargamos Oxide desde umod.org
    descargar_archivo("https://umod.org/games/rust/download", &ruta_archivo_oxide).await?;

    set_estado
        .send("📦 Extrayendo archivos en el servidor...".to_string())
        .unwrap();
    set_estado
        .send("🔌 Instalando Oxide...".to_string())
        .unwrap();

    // Descomprimimos Oxide sobre la carpeta del servidor
    let resultado_extraccion = Command::new("tar")
        .args(&[
            "-xf",
            ruta_archivo_oxide.to_str().unwrap(),
            "-C",
            directorio_servidor.to_str().unwrap(),
        ])
        .creation_flags(0x08000000) // Sin ventana visible
        .output()
        .map_err(|error| format!("Error descomprimiendo Oxide: {}", error))?;

    if !resultado_extraccion.status.success() {
        return Err("❌ Error al instalar Oxide".to_string());
    }

    // Limpiamos el archivo ZIP descargado
    fs::remove_file(&ruta_archivo_oxide).map_err(|error| error.to_string())?;

    set_estado
        .send("✅ Oxide instalado correctamente. El servidor ya soporta plugins!".to_string())
        .unwrap();
    Ok(())
}

/// 4. ELIMINAR TODO
/// Elimina toda la carpeta de instalación del servidor
#[tauri::command]
pub async fn eliminar_todo(set_estado: Channel<String>) -> Result<(), String> {
    let directorio_raiz = obtener_ruta_aplicacion("");

    if directorio_raiz.exists() {
        set_estado
            .send("🗑️ Eliminando todos los archivos...".to_string())
            .unwrap();
        fs::remove_dir_all(&directorio_raiz).map_err(|error| error.to_string())?;
        set_estado
            .send("✅ Todos los archivos han sido eliminados".to_string())
            .unwrap();
    } else {
        set_estado
            .send("⚠️ No hay nada que eliminar".to_string())
            .unwrap();
    }

    Ok(())
}

/// Genera una contraseña aleatoria segura usando caracteres alfanuméricos y símbolos
/// Usa el tiempo actual y el ID del proceso como semilla para el generador
fn generar_contraseña_aleatoria() -> String {
    const CARACTERES_PERMITIDOS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // Semilla basada en tiempo y PID para aleatoriedad
    let mut semilla = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    semilla += process::id() as u64;

    let mut contraseña = String::with_capacity(32);
    for _contador in 0..32 {
        // Generador pseudoaleatorio LCG (Linear Congruential Generator)
        semilla = semilla.wrapping_mul(1103515245).wrapping_add(12345);
        let indice = (semilla % CARACTERES_PERMITIDOS.len() as u64) as usize;
        contraseña.push(CARACTERES_PERMITIDOS[indice] as char);
    }
    contraseña
}

/// Genera un número semilla aleatorio para el mapa del servidor
/// Rango válido: 0 a 2,147,483,647 (i32::MAX)
fn generar_server_seed() -> i32 {
    let mut semilla = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    semilla = semilla.wrapping_add(process::id() as u64);

    // Algoritmo LCG para mejor aleatoriedad
    semilla = semilla.wrapping_mul(1103515245).wrapping_add(12345);

    // Rango válido para seeds de Rust: 0 a i32::MAX (2,147,483,647)
    (semilla % (i32::MAX as u64 + 1)) as i32
}

/// Crea el archivo BAT de inicio del servidor con configuración aleatoria
#[tauri::command]
pub async fn crear_iniciador_servidor(set_estado: Channel<String>) -> Result<(), String> {
    if !comprobar_rust_instalado() {
        return Err("❌ Primero instala Rust".to_string());
    }

    let directorio_servidor = obtener_ruta_aplicacion("SERVER");
    let ruta_archivo_bat = directorio_servidor.join("INICIADOR SERVER.bat");
    let rcon_password = generar_contraseña_aleatoria();
    let server_seed = generar_server_seed();

    // Determinamos si estamos creando o regenerando el archivo
    let tipo_accion = if ruta_archivo_bat.exists() {
        "Actualizando"
    } else {
        "Creando"
    };
    set_estado
        .send(format!("📝 {} INICIADOR SERVER.bat...", tipo_accion))
        .unwrap();

    // Contenido del archivo BAT con todos los parámetros de configuración
    let contenido_script = format!(
        r#"@echo off
RustDedicated.exe -batchmode -nographics ^
+server.hostname "Servidor Rust Simple SCR98" ^
+server.description "Servidor creado con Instalador" ^
+server.headerimage "" ^
+server.url "" ^
+server.ip 0.0.0.0 ^
+server.port 28015 ^
+server.maxplayers 100 ^
+rcon.ip 0.0.0.0 ^
+rcon.port 28016 ^
+rcon.password "{}" ^
+server.identity "default" ^
+server.level "Procedural Map" ^
+server.seed {} +server.worldsize 4000 ^
+server.radiation "True" ^
+bradley.enabled "True" ^
+bradley.respawndelayminutes "60" ^
+bradley.respawndelayvariance "1" ^
+heli.lifetimeminutes "15" ^
+server.stability "True" ^
+decay.upkeep "True" ^
+decay.upkeep_heal_scale "1" ^
+decay.upkeep_inside_decay_scale "0.1" ^
+decay.upkeep_period_minutes "1440" ^
+rcon.web "True" ^
-logfile "servidor-rust-simple-logfile.log" ^"#,
        rcon_password, server_seed
    );

    // Escribimos el archivo en disco
    let mut archivo_salida =
        fs::File::create(&ruta_archivo_bat).map_err(|error| error.to_string())?;
    archivo_salida
        .write_all(contenido_script.as_bytes())
        .map_err(|error| error.to_string())?;

    // Mensaje final según la acción realizada
    if tipo_accion == "Actualizando" {
        set_estado
            .send("✅ INICIADOR SERVER.bat regenerado correctamente".to_string())
            .unwrap();
    } else {
        set_estado
            .send("✅ INICIADOR SERVER.bat creado correctamente".to_string())
            .unwrap();
    }

    Ok(())
}

/// 5. ABRIR CARPETA PLUGINS
/// Abre la carpeta de plugins de Oxide en el explorador de Windows
#[tauri::command]
pub async fn abrir_carpeta_plugins(set_estado: Channel<String>) -> Result<(), String> {
    // Comprobación jerárquica de dependencias
    if !comprobar_steamcmd_instalado() {
        return Err("❌ Primero instala SteamCMD.".to_string());
    }

    if !comprobar_rust_instalado() {
        return Err("❌ Primero instala Rust.".to_string());
    }

    // Verificar que Oxide esté instalado (carpeta oxide existe)
    let directorio_oxide = obtener_ruta_aplicacion("SERVER/oxide");
    if !directorio_oxide.exists() {
        return Err("❌ Primero instala Oxide y ejecuta el servidor al menos una vez para generar las carpetas".to_string());
    }

    // Verificar que la subcarpeta plugins exista
    let directorio_plugins = obtener_ruta_aplicacion("SERVER/oxide/plugins");
    if !directorio_plugins.exists() {
        return Err("❌ La carpeta plugins no existe. Ejecuta el servidor una vez para que Oxide la genere automáticamente".to_string());
    }

    set_estado
        .send("📂 Abriendo carpeta de plugins...".to_string())
        .unwrap();

    // Convertimos la ruta a string para el comando
    let ruta_texto = directorio_plugins.to_str().unwrap();

    // Abrimos el explorador de Windows en esa ubicación
    Command::new("cmd")
        .args(&["/C", "start", "", &format!(r#"{}"#, ruta_texto)])
        .creation_flags(0x08000000) // Sin ventana de consola adicional
        .spawn()
        .map_err(|error| format!("No se pudo abrir el explorador: {}", error))?;

    set_estado
        .send("✅ Carpeta de plugins abierta".to_string())
        .unwrap();
    Ok(())
}

/// 6. INICIAR SERVIDOR
/// Ejecuta el archivo BAT del servidor en una ventana de consola visible
#[tauri::command]
pub async fn iniciar_servidor(set_estado: Channel<String>) -> Result<(), String> {
    // Comprobaciones de dependencias
    if !comprobar_steamcmd_instalado() {
        return Err("❌ Primero instala SteamCMD".to_string());
    }
    if !comprobar_rust_instalado() {
        return Err("❌ Primero instala Rust".to_string());
    }

    // Verificar que exista el archivo de inicio
    let ruta_archivo_bat = obtener_ruta_aplicacion("SERVER/INICIADOR SERVER.bat");
    if !ruta_archivo_bat.exists() {
        return Err("❌ Crea el iniciador primero".to_string());
    }

    let directorio_trabajo = obtener_ruta_aplicacion("SERVER");

    set_estado
        .send("🚀 Iniciando servidor en nueva ventana...".to_string())
        .unwrap();

    // Ejecutamos el BAT directamente (spawn lo deja corriendo independientemente)
    Command::new(&ruta_archivo_bat)
        .current_dir(&directorio_trabajo) // Establecemos el directorio de trabajo
        .spawn()
        .map_err(|error| format!("Error al iniciar: {}", error))?;

    set_estado
        .send("✅ Servidor iniciado en ventana aparte".to_string())
        .unwrap();
    set_estado
        .send("💡 Escribe 'quit' en esa ventana para cerrar, o usa el botón Apagar".to_string())
        .unwrap();

    Ok(())
}

/// 7. APAGAR SERVIDOR
/// Envía comando quit vía WebRCON (WebSocket) para cerrar el servidor limpiamente
#[tauri::command]
pub async fn apagar_servidor(set_estado: Channel<String>) -> Result<(), String> {
    if !comprobar_steamcmd_instalado() {
        return Err("❌ Primero instala SteamCMD".to_string());
    }
    if !comprobar_rust_instalado() {
        return Err("❌ Primero instala Rust".to_string());
    }

    let ruta_archivo_bat = obtener_ruta_aplicacion("SERVER/INICIADOR SERVER.bat");
    if !ruta_archivo_bat.exists() {
        return Err("❌ Crea el iniciador primero".to_string());
    }

    set_estado
        .send("🛑 Conectando vía WebRCON para enviar quit...".to_string())
        .unwrap();

    // Leemos la contraseña RCON del archivo BAT
    let rcon_password = extraer_rcon_password(&ruta_archivo_bat)?;
    let direccion_conexion = "127.0.0.1:28016"; // Localhost, puerto por defecto

    // Intentamos conectar y enviar el comando quit
    match enviar_comando_webrcon_quit(direccion_conexion, &rcon_password, set_estado.clone()).await
    {
        Ok(_) => {
            // set_estado.send("⏳ Comando enviado. El servidor debería cerrarse...".to_string());
            Ok(())
        }
        Err(codigo_error) => {
            // Si el servidor ya estaba apagado, no es un error crítico
            if codigo_error == "SERVIDOR_YA_APAGADO" {
                set_estado
                    .send("⚠️ El servidor ya estaba apagado".to_string())
                    .unwrap();
                return Ok(());
            }

            set_estado
                .send(format!("❌ Error WebRCON: {}", codigo_error))
                .unwrap();
            Err(codigo_error)
        }
    }
}

/// Extrae la contraseña RCON del archivo BAT buscando la línea rcon.password
fn extraer_rcon_password(ruta_archivo: &PathBuf) -> Result<String, String> {
    let contenido = fs::read_to_string(ruta_archivo).map_err(|error| error.to_string())?;

    // Buscamos la línea que contiene la contraseña
    for linea in contenido.lines() {
        if linea.contains("rcon.password") {
            // Extraemos el valor entre comillas dobles
            if let Some(posicion_inicio) = linea.find('"') {
                if let Some(posicion_fin) = linea[posicion_inicio + 1..].find('"') {
                    return Ok(
                        linea[posicion_inicio + 1..posicion_inicio + 1 + posicion_fin].to_string(),
                    );
                }
            }
        }
    }

    Err("No se encontró contraseña RCON en el .bat".to_string())
}

/// Conecta vía WebSocket al servidor y envía el comando quit
/// Parámetros:
///   - direccion: IP:Puerto (ej: "127.0.0.1:28016")
///   - contraseña: Contraseña RCON configurada en el servidor
///   - transmisor: Canal para enviar mensajes de estado al frontend
async fn enviar_comando_webrcon_quit(
    direccion: &str,
    contraseña: &str,
    set_estado: Channel<String>,
) -> Result<(), String> {
    // Construimos la URL WebSocket (ws://) con la contraseña como path
    let url_conexion = format!("ws://{}/{}", direccion, contraseña);
    let url_parseada =
        Url::parse(&url_conexion).map_err(|error| format!("URL inválida: {}", error))?;

    // Intentamos conectar con timeout de 3 segundos
    let resultado_conexion =
        timeout(Duration::from_secs(3), connect_async(url_parseada.as_str())).await;

    let (mut websocket, _) = match resultado_conexion {
        Ok(Ok((ws, respuesta))) => (ws, respuesta),
        Ok(Err(error)) => {
            let mensaje_error = error.to_string();
            // Código 10061 = Connection refused (servidor apagado)
            if mensaje_error.contains("10061") || mensaje_error.contains("connection refused") {
                return Err("SERVIDOR_YA_APAGADO".to_string());
            }
            return Err(format!("Error conectando WebRCON: {}", mensaje_error));
        }
        Err(_) => {
            return Err("Tiempo de espera agotado al conectar".to_string());
        }
    };

    set_estado
        .send("📡 Conectado a WebRCON".to_string())
        .unwrap();

    // Preparamos el mensaje JSON con el comando quit
    let mensaje_json = json!({
        "Identifier": 1,        // ID único del mensaje
        "Message": "quit",      // Comando a ejecutar
        "Name": "WebRcon"       // Identificador del cliente
    });

    // Enviamos el mensaje por el WebSocket
    websocket
        .send(Message::Text(mensaje_json.to_string().into()))
        .await
        .map_err(|error| format!("Error enviando quit: {}", error))?;

    set_estado
        .send("📤 Comando 'quit' enviado".to_string())
        .unwrap();

    // Esperamos respuesta del servidor (máximo 3 segundos)
    let espera_respuesta = timeout(Duration::from_secs(3), websocket.next()).await;

    match espera_respuesta {
        Ok(Some(Ok(_))) => {
            set_estado
                .send("📨 Respuesta recibida del servidor".to_string())
                .unwrap();
        }
        Ok(Some(Err(error))) => {
            set_estado
                .send(format!("⚠️ Error respuesta WebSocket: {}", error))
                .unwrap();
        }
        Ok(None) => {
            set_estado
                .send("⚠️ WebRCON cerró sin respuesta".to_string())
                .unwrap();
        }
        Err(_) => {
            set_estado
                .send("⏱️ Sin respuesta (timeout)".to_string())
                .unwrap();
        }
    }

    // Cierre limpio de la conexión
    set_estado
        .send("✅ Proceso de apagado finalizado".to_string())
        .unwrap();

    let _ = websocket.close(None).await;

    Ok(())
}
