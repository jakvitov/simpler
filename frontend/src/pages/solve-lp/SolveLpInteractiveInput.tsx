import MainHeader from "../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve-input/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve-input/SolverAlgorithmRadial.tsx";
import ConfirmButton from "../../components/ui/general/ConfirmButton.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import {useState} from "react";
import type {OptimisationTarget, SolverMethods} from "../../api/solver/solveLpTypes.ts";

function SolveLpInteractiveInput() {

    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")
    const [optimisationTarget, setOptimisationTarget] = useState<OptimisationTarget>("MIN")


    const handleSolveInteractiveButtonClick = () => {
        alert("Under construction")
    }

    return (
        <>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <Container>
                    <SolverInputType />
                    <SolverAlgorithmRadial
                        onSelectedSolverMethod={setSolverMethod}
                        currentSelectedSolverMethod={solverMethod}
                        onSelectedOptimisationTarget={setOptimisationTarget}
                        currentSelectedOptimisationTarget={optimisationTarget}
                    />
                    <ConfirmButton onChange={handleSolveInteractiveButtonClick}></ConfirmButton>
                </Container>
            </div>
            <BottomNavBar />
        </>
    )
}

export default SolveLpInteractiveInput