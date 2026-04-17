'use client'

import { getCurrentWindow } from '@tauri-apps/api/window'
import { Copy, Minus, Square, X } from 'lucide-react'
import Image from 'next/image'
import { useEffect, useState } from 'react'

export default function MenuWindows() {
  const [maximized, setMaximized] = useState(false)

  useEffect(() => {
    const win = getCurrentWindow()
    let unlisten: (() => void) | undefined

    const init = async () => {
      setMaximized(await win.isMaximized())

      unlisten = await win.onResized(async () => {
        setMaximized(await win.isMaximized())
      })
    }

    void init()

    return () => {
      unlisten?.()
    }
  }, [])

  const handleMinimize = () => {
    getCurrentWindow().minimize()
  }

  const handleMaximize = async () => {
    const win = getCurrentWindow()
    const isMax = await win.isMaximized()

    if (isMax) {
      await win.unmaximize()
      setMaximized(false)
    } else {
      await win.maximize()
      setMaximized(true)
    }
  }

  const handleClose = () => {
    getCurrentWindow().close()
  }

  return (
    <div
      className="drag flex h-10 items-center justify-between select-none bg-amber-500 text-white"
      onDoubleClick={handleMaximize}
    >
      <div className="pl-4 flex items-center gap-2">
        <Image
          src="SCR98-RUST.avif"
          alt="SCR98-RUST"
          width={64}
          height={64}
          className="w-6 h-6 rounded-full object-cover"
        />
        <span className="text-sm font-semibold text-white">
          Servidor Rust Simple
        </span>
        <span className="text-[10px] px-2 py-0.5 rounded-full bg-amber-400/60 text-white font-medium">
          SCR98
        </span>
      </div>

      <div className="flex h-full">
        <button
          onClick={handleMinimize}
          className="no-drag flex h-full w-11 items-center justify-center transition-colors hover:bg-amber-400/60 active:bg-amber-400/40 text-white"
        >
          <Minus size={14} strokeWidth={4} />
        </button>

        <button
          onClick={handleMaximize}
          className="no-drag flex h-full w-11 items-center justify-center transition-colors hover:bg-amber-400/60 active:bg-amber-400/40 text-white"
        >
          {maximized ? (
            <Copy size={12} strokeWidth={4} />
          ) : (
            <Square size={12} strokeWidth={4} />
          )}
        </button>

        <button
          onClick={handleClose}
          className="no-drag flex h-full w-11 items-center justify-center transition-colors hover:bg-red-600 active:bg-red-700 text-white"
        >
          <X size={14} strokeWidth={4} />
        </button>
      </div>
    </div>
  )
}
