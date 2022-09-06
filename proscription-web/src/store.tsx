import { atom } from 'jotai'

const socketStore = atom<WebSocket | null>(null)

export { socketStore }
