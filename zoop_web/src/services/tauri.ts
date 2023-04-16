export const isClient = typeof window !== 'undefined'
export const isTauriClient = isClient && '__TAURI__' in window
