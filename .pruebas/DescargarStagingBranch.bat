@echo off
setlocal enabledelayedexpansion

title Descargador Rust Dedicated Server - STAGING

:: Obtenemos la carpeta donde esta el .bat y añadimos "RUST SERVER" igual que en Rust
set "BASE_DIR=%~dp0RUST SERVER"
set "STEAMCMD_EXE=%BASE_DIR%\STEAMCMD\steamcmd.exe"
set "SERVER_DIR=%BASE_DIR%\SERVER"
set "RUST_EXE=%SERVER_DIR%\RustDedicated.exe"

:: 1. Comprobamos si existe SteamCMD en "RUST SERVER\STEAMCMD\steamcmd.exe"
if not exist "%STEAMCMD_EXE%" (
    echo No se encontro SteamCMD en la ruta:
    echo    "%STEAMCMD_EXE%"
    echo.
    echo Ejecuta primero la opcion "Instalar SteamCMD" desde tu aplicacion
    echo o crea la carpeta "RUST SERVER\STEAMCMD" y pon "steamcmd.exe" dentro.
    echo.
    pause
    exit /b 1
)

:: 2. Determinamos si es instalacion nueva o actualizacion
if exist "%RUST_EXE%" (
    set "ACCION=Actualizando"
) else (
    set "ACCION=Instalando"
)

echo !ACCION! servidor de Rust (Staging Branch)...
echo Esto puede tardar 5-10 minutos dependiendo de tu conexion...
echo.

:: 3. Creamos la carpeta SERVER si no existe
if not exist "%SERVER_DIR%" mkdir "%SERVER_DIR%"

echo Descargando archivos del servidor (Staging)...

:: 4. Ejecutamos SteamCMD para descargar la versión Staging
"%STEAMCMD_EXE%" +login anonymous +force_install_dir "%SERVER_DIR%" +app_update 258550 -beta staging validate +quit

:: 5. Comprobación de errores
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error al instalar Rust Staging. Codigo de error: %ERRORLEVEL%
    pause
    exit /b %ERRORLEVEL%
)

echo.
if "!ACCION!"=="Actualizando" (
    echo Servidor de Rust Staging actualizado correctamente.
) else (
    echo Servidor de Rust Staging instalado correctamente.
)

echo.
pause