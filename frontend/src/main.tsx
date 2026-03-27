import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import 'bootstrap/dist/css/bootstrap.min.css';
import '../index.css'
import 'katex/dist/katex.min.css';

import App from "./App.tsx";
import { BrowserRouter } from "react-router-dom";
import TopLevelErrorBoundary from "./components/ui/error/TopLevelErrorBoundary.tsx";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
        <TopLevelErrorBoundary>
            <App/>
        </TopLevelErrorBoundary>
    </BrowserRouter>
  </StrictMode>,
)
