# <img src="app-icon.png" alt="Logo" height="24"> Servidor Rust Simple SCR98

Una aplicación sencilla para crear, instalar y gestionar tu propio servidor de Rust sin complicaciones.

Olvídate de comandos raros o configuraciones complicadas: con unos pocos clics puedes tener tu servidor funcionando.

---

## 🚀 ¿Qué hace esta aplicación?

- Instala SteamCMD automáticamente
- Descarga y actualiza el servidor de Rust
- Instala Oxide (plugins) opcionalmente
- Genera el iniciador del servidor
- Permite iniciar y apagar el servidor fácilmente
- Acceso rápido a la carpeta de plugins

---

## 🧑‍💻 Uso para cualquier persona (paso a paso)

### 📁 1. Preparar la aplicación

1. Crea una carpeta donde quieras (por ejemplo: `ServidorRust`)
2. Mete dentro el `.exe` de la aplicación
3. Ejecuta el programa

---

### 🎮 2. Usar la aplicación (botones en orden)

La app está diseñada para usarse en este orden:

---

### 1️⃣ Instalar SteamCMD

Pulsa el botón:

👉 **"1. Instalar SteamCMD"**

Esto:

- Descarga SteamCMD
- Lo instala automáticamente
- Lo prepara para usar

⏳ Tarda unos segundos

---

### 2️⃣ Instalar / Actualizar Rust

Pulsa:

👉 **"2. Instalar/Actualizar Rust"**

Esto:

- Descarga el servidor de Rust
- Puede tardar varios minutos (normal)

⏳ Tiempo: 5–10 minutos dependiendo de tu internet

---

### 3️⃣ Instalar Oxide (plugins) (Opcional)

Pulsa:

👉 **"3. Instalar Oxide"**

Esto:

- Instala el sistema de plugins
- Es opcional, solo si quieres usar mods

📥 Puedes descargar plugins desde:
👉 https://umod.org

---

### 4️⃣ Crear Iniciador

Pulsa:

👉 **"4. Crear Iniciador"**

Esto:

- Genera un archivo `.bat`
- Configura automáticamente:
  - Contraseña RCON
  - Seed del mapa
  - Configuración básica

---

### 📂 (Opcional) Plugins

Pulsa:

👉 **"Abrir Plugins"**

Aquí puedes:

- Meter plugins (.cs)
- Gestionar mods del servidor

📥 Plugins disponibles en:
👉 https://umod.org

---

### ▶️ 5️⃣ Iniciar Servidor

Pulsa:

👉 **"5. Iniciar Servidor"**

Esto:

- Abre una ventana nueva
- Inicia el servidor

💡 Para apagar manualmente: escribe `quit` en la ventana  
💡 Para ser admin dentro del servidor usa:
`ownerid TuIDdeSteam`

---

### ⛔ 6️⃣ Apagar Servidor

Pulsa:

👉 **"6. Apagar Servidor"**

Esto:

- Cierra el servidor correctamente
- Evita corrupciones

---

### 🗑️ Eliminar Todo

Botón:

👉 **"Eliminar Todo"**

⚠️ Borra absolutamente todo:

- SteamCMD
- Servidor
- Oxide

---

## ⚙️ Parte técnica (para desarrolladores)

La app está hecha con:

- Next.js (frontend)
- Tauri (backend en Rust)

---

## 📦 Requisitos

Antes de empezar necesitas tener instalados:

- 🟢 Node.js → https://nodejs.org/es
- 🦀 Rust → https://rust-lang.org/es

---

## 📥 Instalar dependencias

```bash
npm install
```

---

## 🧪 Ejecutar en desarrollo

```bash
npm run tauri dev
```

---

## 🏗️ Build de producción

```bash
npm run tauri build
```
