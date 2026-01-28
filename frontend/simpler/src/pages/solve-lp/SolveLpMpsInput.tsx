import MainHeader from "../../components/ui/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve/SolverAlgorithmRadial.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import MPSInput from "../../components/layout/mps/MpsInput.tsx";
import {useState} from "react";
import ConfirmButton from "../../components/ui/ConfirmButton.tsx";
import type SolveLpRequest from "../../api/solver/solveLpTypes.ts";
import type {OptimisationTarget, SolverMethods} from "../../api/solver/solveLpTypes.ts";
import {fetchSolveBasicSimplex} from "../../api/solver/basic/basicSimplexSolveApi.ts";
import {set} from "idb-keyval";
import {SOLVE_LP_DATA_PREFIX, SOLVE_LP_SOLUTION_PREFIX} from "../../utils/storageConstants.ts";
import {hashStringSHA256} from "../../utils/hash.ts";
import {useNavigate} from "react-router-dom";

function SolveLpMpsInput() {

    const [mpsInput, setMpsInput] = useState("Enter your MPS code here...")
    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")
    const [optimisationTarget, setOptimisationTarget] = useState<OptimisationTarget>("MIN")

    const navigate = useNavigate()

    const handleSolveBasicSimplex = async(request: SolveLpRequest) => {
        try {
            const requestHash =  await hashStringSHA256(JSON.stringify(request))
            const response = await fetchSolveBasicSimplex(request)
            await set(SOLVE_LP_DATA_PREFIX + requestHash, JSON.stringify(request))
            await set(SOLVE_LP_SOLUTION_PREFIX + requestHash, JSON.stringify(response))
            navigate("/solution/" + requestHash)
        } catch (error) {
            console.error(error)
        }
    }

    const handleSolveMpsButtonClick = () => {

        const request: SolveLpRequest = {
            data: mpsInput,
            optimisationTarget: optimisationTarget,
            solverMethod: solverMethod
        }

        switch (solverMethod) {
            case "BASIC_SIMPLEX":
                handleSolveBasicSimplex(request);
                return;
            case "TWO_PHASE":
                alert("Not implemented")
                return;
            case "REVISED":
                alert("Not implemented");
                return;
            case "MULTIPLICATIVE":
                alert("Not implemented");
                return;
            case "BOUNDS_OPTIMISATION":
                alert("Not implemented");
                return;
            default:
                console.error("Unknown solver method encountered " + solverMethod)
                return;
        }
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
                <MPSInput
                    value={mpsInput}
                    onChange={setMpsInput}
                    rows={12}
                />
                <ConfirmButton onChange={handleSolveMpsButtonClick}></ConfirmButton>
            </Container>
            </div>
            <BottomNavBar />
            </>
    )
}

export default SolveLpMpsInput
