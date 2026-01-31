import {Route, Routes } from "react-router-dom";
import Home from "./pages/Home.tsx";
import About from "./pages/About.tsx";
import VerifyMps from "./pages/VerifyMps.tsx";
import MpsVerificationResults from "./pages/MpsVerificationResults.tsx";
import SolveLpMpsInput from "./pages/solve-lp/SolveLpMpsInput.tsx";
import SolveLpInteractiveInput from "./pages/solve-lp/SolveLpInteractiveInput.tsx";
import SolveLpLayout from "./pages/solve-lp/SolveLpLayout.tsx";
import Settings from "./pages/Settings.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Home/>}/>
            <Route path="/about" element={<About/>} />
            <Route path="/settings" element={<Settings/>}/>
            <Route path="/verify-mps" element={<VerifyMps/>} />
            <Route path="/mps-verification-results/:key" element={<MpsVerificationResults />} />
            <Route path="/solve-lp" element={<SolveLpLayout/>}>
                <Route index element={<SolveLpMpsInput/>}/>
                <Route path="mps" element={<SolveLpMpsInput/>}/>
                <Route path="interactive" element={<SolveLpInteractiveInput/>}/>
            </Route>
        </Routes>
    )
}

export default App