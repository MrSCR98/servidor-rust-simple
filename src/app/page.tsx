'use client'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Channel, invoke } from '@tauri-apps/api/core'
import {
  Download,
  FileText,
  FolderOpen,
  Gamepad2,
  Loader2,
  Play,
  Square,
  Terminal,
  Trash2,
  Wrench,
} from 'lucide-react'
import { useState } from 'react'
import { toast } from 'sonner'

type Operation =
  | 'steam'
  | 'rust'
  | 'oxide'
  | 'create_bat'
  | 'open_plugins'
  | 'start_server'
  | 'stop_server'
  | 'delete'
  | null

export default function RustServerManager() {
  const [currentOp, setCurrentOp] = useState<Operation>(null)
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)

  const isLoading = (op: Operation) => currentOp === op
  const isProcessing = currentOp !== null

  const createEstadoChannel = () => {
    const channel = new Channel<string>()
    channel.onmessage = (message) => {
      if (message.toLowerCase().includes('error')) {
        toast.error(message)
      } else if (
        message.toLowerCase().includes('listo') ||
        message.includes('✅')
      ) {
        toast.success(message)
      } else {
        toast.info(message, { icon: <Terminal className="h-4 w-4" /> })
      }
    }
    return channel
  }

  const handleInstalarSteam = async () => {
    if (isProcessing) return
    setCurrentOp('steam')
    toast.info('Iniciando instalación de SteamCMD...')

    try {
      await invoke('instalar_steam', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleInstalarRust = async () => {
    if (isProcessing) return
    setCurrentOp('rust')
    toast.info('Iniciando descarga de Rust (esto tarda varios minutos)...')

    try {
      await invoke('instalar_rust', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleInstalarOxide = async () => {
    if (isProcessing) return
    setCurrentOp('oxide')
    toast.info('Instalando Oxide...')

    try {
      await invoke('instalar_oxide', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleEliminarConfirmado = async () => {
    if (isProcessing) return
    setShowDeleteDialog(false)
    setCurrentOp('delete')
    toast.info('Eliminando archivos...')

    try {
      await invoke('eliminar_todo', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleCrearIniciador = async () => {
    if (isProcessing) return
    setCurrentOp('create_bat')
    toast.info('Creando archivo iniciador...')

    try {
      await invoke('crear_iniciador_servidor', {
        setEstado: createEstadoChannel(),
      })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleAbrirPlugins = async () => {
    if (isProcessing) return
    setCurrentOp('open_plugins')
    toast.info('Abriendo carpeta de plugins...')

    try {
      await invoke('abrir_carpeta_plugins', {
        setEstado: createEstadoChannel(),
      })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleIniciarServidor = async () => {
    if (isProcessing) return
    setCurrentOp('start_server')
    toast.info('Iniciando servidor...')

    try {
      await invoke('iniciar_servidor', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  const handleApagarServidor = async () => {
    if (isProcessing) return
    setCurrentOp('stop_server')
    toast.info('Apagando servidor...')

    try {
      await invoke('apagar_servidor', { setEstado: createEstadoChannel() })
    } catch (error) {
      toast.error(`Error: ${error}`)
    } finally {
      setCurrentOp(null)
    }
  }

  return (
    <div className="flex flex-col items-center min-h-[calc(100vh-2.5rem-2.5rem)] p-8 gap-8 justify-center">
      <h1 className="text-[2.75rem] leading-10 font-extrabold text-amber-500 [text-shadow:0_0_8px_rgb(247_174_0/0.4)]">
        Gestor de Servidor Rust
      </h1>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full max-w-2xl">
        <Button
          onClick={handleInstalarSteam}
          disabled={isProcessing}
          className="h-16 text-lg bg-amber-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-amber-500 hover:bg-[rgb(247,174,0)] hover:shadow-[rgb(247,174,0)] text-white hover:text-white hover:cursor-pointer border-[rgb(247,174,0)]"
          variant="outline"
        >
          {isLoading('steam') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <Download className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          1. Instalar SteamCMD
        </Button>

        <Button
          onClick={handleInstalarRust}
          disabled={isProcessing}
          className="h-16 text-lg bg-amber-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-amber-500 hover:bg-[rgb(247,174,0)] hover:shadow-[rgb(247,174,0)] text-white hover:text-white hover:cursor-pointer border-[rgb(247,174,0)]"
          variant="outline"
        >
          {isLoading('rust') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <Gamepad2 className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          2. Instalar/Actualizar Rust
        </Button>

        <Button
          onClick={handleInstalarOxide}
          disabled={isProcessing}
          className="h-16 text-lg bg-amber-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-amber-500 hover:bg-[rgb(247,174,0)] hover:shadow-[rgb(247,174,0)] text-white hover:text-white hover:cursor-pointer border-[rgb(247,174,0)]"
          variant="outline"
        >
          {isLoading('oxide') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <Wrench className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          3. Instalar Oxide
        </Button>

        <AlertDialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
          <AlertDialogTrigger asChild>
            <Button
              disabled={isProcessing}
              className="hover:cursor-pointer h-16 text-lg bg-red-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-red-500 hover:bg-[rgb(246,77,82)] hover:shadow-[rgb(246,77,82)] text-white"
              variant="destructive"
            >
              {isLoading('delete') ? (
                <Loader2
                  className="mr-2 h-5 w-5 animate-spin"
                  strokeWidth={3}
                />
              ) : (
                <Trash2 className="mr-2 h-5 w-5" strokeWidth={3} />
              )}
              Eliminar Todo
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>¿Eliminar todo?</AlertDialogTitle>
              <AlertDialogDescription>
                Esta acción eliminará permanentemente SteamCMD, el servidor de
                Rust y Oxide. No se puede deshacer.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel className="hover:cursor-pointer">
                Cancelar
              </AlertDialogCancel>
              <AlertDialogAction
                onClick={handleEliminarConfirmado}
                className="bg-red-600 hover:bg-red-700 hover:cursor-pointer"
              >
                Eliminar definitivamente
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>

        <Separator className="col-span-1 sm:col-span-2 my-1 bg-amber-500 shadow-[0_0_8px_0px] shadow-[rgb(247,174,0)]/70 mx-20 data-horizontal:w-auto h-0.75! rounded-[100%]" />

        <Button
          onClick={handleCrearIniciador}
          disabled={isProcessing}
          className="h-16 text-lg bg-amber-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-amber-500 hover:bg-[rgb(247,174,0)] hover:shadow-[rgb(247,174,0)] text-white hover:text-white hover:cursor-pointer border-[rgb(247,174,0)]"
          variant="outline"
        >
          {isLoading('create_bat') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <FileText className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          4. Crear Iniciador
        </Button>

        <Button
          onClick={handleAbrirPlugins}
          disabled={isProcessing}
          className="h-16 text-lg bg-amber-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-amber-500 hover:bg-[rgb(247,174,0)] hover:shadow-[rgb(247,174,0)] text-white hover:text-white hover:cursor-pointer border-[rgb(247,174,0)]"
          variant="outline"
        >
          {isLoading('open_plugins') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <FolderOpen className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          Abrir Plugins
        </Button>

        <Separator className="col-span-1 sm:col-span-2 my-1 bg-amber-500 shadow-[0_0_8px_0px] shadow-[rgb(247,174,0)]/70 mx-40 data-horizontal:w-auto h-0.75! rounded-[100%]" />

        <Button
          onClick={handleIniciarServidor}
          disabled={isProcessing}
          className="h-16 text-lg bg-green-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-green-500 hover:bg-[rgb(49,216,105)] hover:shadow-[rgb(49,216,105)] hover:cursor-pointer"
        >
          {isLoading('start_server') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <Play className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          5. Iniciar Servidor
        </Button>

        <Button
          onClick={handleApagarServidor}
          disabled={isProcessing}
          className="hover:cursor-pointer h-16 text-lg bg-red-500 transition-all ease-in-out duration-200 rounded-[50px] shadow-[0_0_16px_-6px] shadow-red-500 hover:bg-[rgb(246,77,82)] hover:shadow-[rgb(246,77,82)] text-white"
          variant="destructive"
        >
          {isLoading('stop_server') ? (
            <Loader2 className="mr-2 h-5 w-5 animate-spin" strokeWidth={3} />
          ) : (
            <Square className="mr-2 h-5 w-5" strokeWidth={3} />
          )}
          6. Apagar Servidor
        </Button>
      </div>
    </div>
  )
}
