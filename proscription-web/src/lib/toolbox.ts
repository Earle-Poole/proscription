export const openSocket = (url: string) => {
  const socket = new WebSocket(url)

  return socket
}
