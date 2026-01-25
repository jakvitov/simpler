import {Route, Routes } from "react-router-dom";
import Home from "./pages/Home.tsx";
import About from "./pages/About.tsx";
import VerifyMps from "./pages/VerifyMps.tsx";
import MpsVerificationResults from "./pages/MpsVerificationResults.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Home/>}/>
            <Route path="/about" element={<About/>} />
            <Route path="/verify-mps" element={<VerifyMps/>} />
            <Route path="/mps-verification-results/:key" element={<MpsVerificationResults />} />
        </Routes>
    )
}

export default App