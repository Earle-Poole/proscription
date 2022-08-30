import Header from '@molecules/Header'
import { FC, PropsWithChildren } from 'react'

interface MainLayoutProps extends PropsWithChildren {
  children: React.ReactNode
}

const MainLayout: FC<MainLayoutProps> = ({ children }) => {
  return (
    <>
      <Header />
      {children}
    </>
  )
}

export default MainLayout
