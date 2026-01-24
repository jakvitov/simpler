import {Route, Routes } from "react-router-dom";
import Home from "./pages/Home.tsx";
import About from "./pages/About.tsx";
import VerifyMps from "./pages/VerifyMps.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Home/>}/>
            <Route path="/about" element={<About/>} />
            <Route path="/verify-mps" element={<VerifyMps/>} />
        </Routes>
    )
}

export default App