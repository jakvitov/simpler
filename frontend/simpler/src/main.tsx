import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import 'bootstrap/dist/css/bootstrap.min.css';
import '../index.css'
import MainHeader from "./components/layout/MainHeader.tsx";
import MainNavBar from "./components/layout/MainNavBar.tsx";
import BottomNavBar from "./components/layout/BottomNavBar.tsx";
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <MainHeader />
      <MainNavBar />
      <BottomNavBar />
  </StrictMode>,
)
