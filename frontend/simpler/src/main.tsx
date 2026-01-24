import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import 'bootstrap/dist/css/bootstrap.min.css';
import '../index.css'
import MainHeader from "./components/ui/MainHeader.tsx";
import MainNavBar from "./components/layout/MainNavBar.tsx";
import BottomNavBar from "./components/layout/BottomNavBar.tsx";
import AboutBottomNavBar from "./components/ui/AboutBottomNavBar.tsx";
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MainHeader />
      <MainNavBar />
      <AboutBottomNavBar />
      <BottomNavBar />

  </StrictMode>,
)
