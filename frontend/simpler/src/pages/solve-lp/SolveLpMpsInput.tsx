import MainHeader from "../../components/ui/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve/SolverAlgorithmRadial.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import MPSInput from "../../components/layout/mps/MpsInput.tsx";
import {useState} from "react";
import type {SolverMethods} from "../../api/solver/solveLpTypes.ts";

function SolveLpMpsInput() {

    const [mpsInput, setMpsInput] = useState("Enter your MPS code here...")
    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")

    return (
        <>
            <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
            <Container>
                <SolverInputType />
                <SolverAlgorithmRadial
                    onSelected={setSolverMethod}
                    currentSelected={solverMethod}
                />
                <MPSInput
                    value={mpsInput}
                    onChange={setMpsInput}
                    rows={12}
                />
            </Container>
            </div>
            <BottomNavBar />
            </>
    )
}

export default SolveLpMpsInput
