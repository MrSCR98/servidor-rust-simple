'use client'

import { invoke } from '@tauri-apps/api/core'

export function PieDePagina() {
  // Función asíncrona para llamar a 'abrir_enlace' desde Tauri
  const handleAbrirEnlace = async (value: string) => {
    try {
      await invoke('abrir_enlace', { url: value })
      console.log('Enlace a GitHub abierto')
    } catch (error) {
      console.error('Error al abrir enlace:', error)
    }
  }

  return (
    <footer className="w-full mt-auto bg-amber-500 text-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8 flex h-10 items-center justify-center">
        <div className="text-balance text-center text-sm leading-loose text-white md:text-left">
          Creado por{' '}
          <button
            onClick={() => handleAbrirEnlace('https://www.youtube.com/@SCR98')}
            className="font-medium underline underline-offset-4 hover:cursor-pointer"
          >
            SCR98
          </button>
          . Si deseas mejorar la aplicación, el código está disponible en{' '}
          <button
            className="font-medium underline underline-offset-4 hover:cursor-pointer"
            onClick={() =>
              handleAbrirEnlace(
                'https://github.com/MrSCR98/servidor-rust-simple'
              )
            }
          >
            GitHub
          </button>
          .
        </div>
      </div>
    </footer>
  )
}
