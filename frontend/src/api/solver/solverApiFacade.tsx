import type {SolveLpRequest} from "./solveLpTypes.ts";
import {hashStringSHA256} from "../../utils/hash.ts";
import {fetchSolveBasicSimplex} from "./basic/basicSimplexSolveApi.ts";
import {get, set} from "idb-keyval";
import {
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX,
    SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX, SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX,
    SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX, SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX
} from "../../utils/storageConstants.ts";
import {fetchSolveTwoPhaseSimplex} from "./two-phase/twoPhaseSimplexSolveApi.ts";
import {fetchSolveRevisedSimplex} from "./revised/revisedSimlexSolveApi.ts";
import {fetchSolveMultiplicativeSimplex} from "./multiplicative/multiplicativeSimplexSolveApi.ts";
import type {NavigateFunction} from "react-router-dom";

const handleSolveBasicSimplex = async(request: SolveLpRequest, navigate: NavigateFunction) => {
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

const handleSolveTwoPhaseSimplex = async(request: SolveLpRequest, navigate: NavigateFunction) => {
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

const handleSolveRevisedSimplex = async(request: SolveLpRequest, navigate: NavigateFunction) => {
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

const handleSolveMultiplicativeSimplex = async(request: SolveLpRequest, navigate: NavigateFunction) => {
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

export function handleSolveRequestBasedOnSolverMethod(request: SolveLpRequest, navigate: NavigateFunction) {
    switch (request.method) {
        case "BASIC_SIMPLEX":
            handleSolveBasicSimplex(request, navigate);
            return;
        case "TWO_PHASE":
            handleSolveTwoPhaseSimplex(request, navigate);
            return;
        case "REVISED":
            handleSolveRevisedSimplex(request, navigate);
            return;
        case "MULTIPLICATIVE":
            handleSolveMultiplicativeSimplex(request, navigate);
            return;
        case "BOUNDS_OPTIMISATION":
            alert("Not implemented");
            return;
        default:
            console.error("Unknown solver method encountered " + request.method)
            return;
    }
}