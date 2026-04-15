import {Route, Routes} from "react-router-dom";
import Home from "./pages/Home.tsx";
import About from "./pages/About.tsx";
import VerifyMps from "./pages/verify-mps/VerifyMps.tsx";
import MpsVerificationResults from "./pages/verify-mps/MpsVerificationResults.tsx";
import SolveLpMpsInput from "./pages/solve-lp/SolveLpMpsInput.tsx";
import SolveLpInteractiveInput from "./pages/solve-lp/SolveLpInteractiveInput.tsx";
import SolveLpLayout from "./pages/solve-lp/SolveLpLayout.tsx";
import Settings from "./pages/Settings.tsx";
import VerifyMpsLayout from "./pages/verify-mps/VerifyMpsLayout.tsx";
import SolveLpResultsBasicSimplex from "./pages/solve-lp/results/SolveLpResultsBasicSimplex.tsx";
import SolveLpResultsError from "./pages/solve-lp/results/SolveLpResultsError.tsx";
import SolveLpResultsTwoPhaseSimplex from "./pages/solve-lp/results/SolveLpResultsTwoPhaseSimplex.tsx";
import ReportBug from "./pages/bugreport/ReportBug.tsx";
import SolveLpResultsRevisedSimplex from "./pages/solve-lp/results/SolveLpResultsRevisedSimplex.tsx";
import SolveLpResultsMultiplicativeSimplex from "./pages/solve-lp/results/SolveLpResultsMultiplicativeSimplex.tsx";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Home/>}/>
            <Route path="/about" element={<About/>} />
            <Route path="/settings" element={<Settings/>}/>

            <Route path="/verify-mps" element={<VerifyMpsLayout/>} >
                <Route index element={<VerifyMps/>}/>
                <Route path="results/:key" element={<MpsVerificationResults />} />
            </Route>

            <Route path={"/report-bug"} element={<ReportBug />} />

            <Route path="/solve-lp" element={<SolveLpLayout/>}>
                <Route index element={<SolveLpMpsInput/>}/>
                <Route path="mps" element={<SolveLpMpsInput/>}/>
                <Route path="interactive" element={<SolveLpInteractiveInput/>}/>

                <Route path="results/basic-simplex/:key" element={<SolveLpResultsBasicSimplex/>}></Route>
                <Route path="results/two-phase-simplex/:key" element={<SolveLpResultsTwoPhaseSimplex/>}></Route>
                <Route path="results/revised-simplex/:key" element={<SolveLpResultsRevisedSimplex/>}></Route>
                <Route path="results/multiplicative-simplex/:key" element={<SolveLpResultsMultiplicativeSimplex />}></Route>
                <Route path="results/error/:key" element={<SolveLpResultsError/>}></Route>
            </Route>
        </Routes>
    )
}

export default App