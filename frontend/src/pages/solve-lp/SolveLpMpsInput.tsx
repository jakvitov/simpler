import MainHeader from "../../components/ui/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve-input/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve-input/SolverAlgorithmRadial.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import MPSInput from "../../components/layout/mps/MpsInput.tsx";
import {useEffect, useState} from "react";
import ConfirmButton from "../../components/ui/ConfirmButton.tsx";
import type {OptimisationTarget, SolveLpRequest, SolverMethods} from "../../api/solver/solveLpTypes.ts";
import {fetchSolveBasicSimplex} from "../../api/solver/basic/basicSimplexSolveApi.ts";
import {get, set} from "idb-keyval";
import {
    LAST_MPS_INPUT_DATA,
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX,
    SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX,
} from "../../utils/storageConstants.ts";
import {hashStringSHA256} from "../../utils/hash.ts";
import {useNavigate} from "react-router-dom";

function SolveLpMpsInput() {

    const [mpsInput, setMpsInput] = useState("Enter your MPS code here...")
    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")
    const [optimisationTarget, setOptimisationTarget] = useState<OptimisationTarget>("MIN")

    const navigate = useNavigate()

    useEffect(() => {
        const previousInput: string|null = localStorage.getItem(LAST_MPS_INPUT_DATA);
        if (previousInput !== null) {
            setMpsInput(previousInput)
        }
    }, []);

    const handleSolveBasicSimplex = async(request: SolveLpRequest) => {
        try {
            const requestHash =  await hashStringSHA256(JSON.stringify(request))
            const response = await fetchSolveBasicSimplex(request)

            if (await get(SOLVE_LP_DATA_PREFIX + requestHash) === undefined) {
                await set(SOLVE_LP_DATA_PREFIX + requestHash, JSON.stringify(request))
            }

            if (response.success) {
                if (await get(SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX + requestHash) === undefined) {
                    await set(SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX + requestHash, JSON.stringify(response))
                }
                navigate(`/solve-lp/results/basic-simplex/${requestHash}`)
            } else {
                if (await get(SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX + requestHash) === undefined) {
                    await set(SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX + requestHash, JSON.stringify(response))
                }
                navigate(`/solve-lp/results/error/${requestHash}`)
            }


        } catch (error) {
            console.error(error)
        }
    }

    const handleSolveMpsButtonClick = () => {

        const request: SolveLpRequest = {
            data: mpsInput,
            optimisationTarget: optimisationTarget,
            method: solverMethod
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
