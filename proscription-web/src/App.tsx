import { useSocket } from '@lib/hooks'
import MainMenu from '@templates/MainMenu'
import { API_URL } from '@constants'

function App() {
  const BACKEND_URL = 'ws://' + API_URL + '/stream'
  const socket = useSocket(BACKEND_URL)
  if (socket) {
    socket.onopen = () => {
      console.log('Socket Opened')
      setInterval(() => socket.send('Hello rust!'), 3000)
    }
    socket.onmessage = (msg) => console.log('onmessage: ', msg.data)
    socket.onerror = (err) => console.error(err)
    socket.onclose = () => console.log('Socket Closed')
  }
  return <MainMenu />
}

export default App
