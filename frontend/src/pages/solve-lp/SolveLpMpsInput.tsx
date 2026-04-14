import MainHeader from "../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve-input/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve-input/SolverAlgorithmRadial.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import MPSInput from "../../components/layout/mps/MpsInput.tsx";
import {useEffect, useState} from "react";
import ConfirmButton from "../../components/ui/general/ConfirmButton.tsx";
import type {
    OptimisationTarget,
    SolveLpRequest,
    SolverConfiguration,
    SolverMethods
} from "../../api/solver/solveLpTypes.ts";
import {fetchSolveBasicSimplex} from "../../api/solver/basic/basicSimplexSolveApi.ts";
import {get, set} from "idb-keyval";
import {
    LAST_MPS_INPUT_DATA,
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX,
    SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX, SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX,
    SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX, SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX, SOLVER_CONFIGURATION_KEY,
} from "../../utils/storageConstants.ts";
import {hashStringSHA256} from "../../utils/hash.ts";
import {useNavigate} from "react-router-dom";
import {fetchSolveTwoPhaseSimplex} from "../../api/solver/two-phase/twoPhaseSimplexSolveApi.ts";
import {fetchSolveRevisedSimplex} from "../../api/solver/revised/revisedSimlexSolveApi.ts";
import {fetchSolveMultiplicativeSimplex} from "../../api/solver/multiplicative/multiplicativeSimplexSolveApi.ts";

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

    const handleSolveTwoPhaseSimplex = async(request: SolveLpRequest) => {
        try {
            const requestHash =  await hashStringSHA256(JSON.stringify(request))
            const response = await fetchSolveTwoPhaseSimplex(request)

            if (await get(SOLVE_LP_DATA_PREFIX + requestHash) === undefined) {
                await set(SOLVE_LP_DATA_PREFIX + requestHash, JSON.stringify(request))
            }

            if (response.success) {
                if (await get(SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX + requestHash) === undefined) {
                    await set(SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX + requestHash, JSON.stringify(response))
                }
                navigate(`/solve-lp/results/two-phase-simplex/${requestHash}`)
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

    const handleSolveRevisedSimplex = async(request: SolveLpRequest) => {
        try {
            const requestHash =  await hashStringSHA256(JSON.stringify(request))
            const response = await fetchSolveRevisedSimplex(request)

            if (await get(SOLVE_LP_DATA_PREFIX + requestHash) === undefined) {
                await set(SOLVE_LP_DATA_PREFIX + requestHash, JSON.stringify(request))
            }

            if (response.success) {
                if (await get(SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX + requestHash) === undefined) {
                    await set(SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX + requestHash, JSON.stringify(response))
                }
                navigate(`/solve-lp/results/revised-simplex/${requestHash}`)
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

    const handleSolveMultiplicativeSimplex = async(request: SolveLpRequest) => {
        try {
            const requestHash =  await hashStringSHA256(JSON.stringify(request))
            const response = await fetchSolveMultiplicativeSimplex(request)

            if (await get(SOLVE_LP_DATA_PREFIX + requestHash) === undefined) {
                await set(SOLVE_LP_DATA_PREFIX + requestHash, JSON.stringify(request))
            }

            if (response.success) {
                if (await get(SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX + requestHash) === undefined) {
                    await set(SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX + requestHash, JSON.stringify(response))
                }
                navigate(`/solve-lp/results/multiplicative-simplex/${requestHash}`)
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

        //Null when not set in settings, backend will use defaults
        const solverConfigurationStr: string|null = localStorage.getItem(SOLVER_CONFIGURATION_KEY);
        let solverConfiguration: SolverConfiguration|null = null
        if (solverConfigurationStr != null) {
            solverConfiguration = JSON.parse(solverConfigurationStr)
        }

        const request: SolveLpRequest = {
            data: mpsInput,
            optimisationTarget: optimisationTarget,
            method: solverMethod,
            solverConfiguration: solverConfiguration
        }

        switch (solverMethod) {
            case "BASIC_SIMPLEX":
                handleSolveBasicSimplex(request);
                return;
            case "TWO_PHASE":
                handleSolveTwoPhaseSimplex(request);
                return;
            case "REVISED":
                handleSolveRevisedSimplex(request);
                return;
            case "MULTIPLICATIVE":
                handleSolveMultiplicativeSimplex(request);
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
