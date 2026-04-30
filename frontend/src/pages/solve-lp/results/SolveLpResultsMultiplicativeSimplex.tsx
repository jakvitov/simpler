import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import {get} from "idb-keyval";
import {
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX,
} from "../../../utils/storageConstants.ts";
import type {
    SolveLpMultiplicativeSimplexResponseDto
} from "../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import MainHeader from "../../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/general/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import CommonErrorBoundary from "../../../components/ui/error/CommonErrorBoundary.tsx";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import SolveLpMultiplicativeSimplexResponseElement
    from "../../../components/layout/solution/multiplicative/SolveLpMultiplicativeSimplexResponseElement.tsx";

function SolveLpResultsMultiplicativeSimplex() {
    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverResults, setSolverRestults] = useState<SolveLpMultiplicativeSimplexResponseDto|null>(null)

    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_MULTIPLICATIVE_SIMPLEX_PREFIX + key).then(i => JSON.parse(i) as SolveLpMultiplicativeSimplexResponseDto).then(setSolverRestults)
    }, [])

    if (solverInput == null || solverResults == null) { /* empty */ } else {
        let pageHeaderText;
        switch (solverResults.solutionStatus) {
            case "SOLVED": pageHeaderText = "LP solved ✅"; break;
            case "UNBOUNDED": pageHeaderText = "LP solution unbounded ♾️";break;
            case "CYCLE": pageHeaderText = "LP solution includes possible cycle 🔄️"; break;
            case "MAX_ITERATIONS": pageHeaderText = "LP sol️ution exceeded max iterations ⚠️";break;
        }
        return (<>
                <div className={"page-content"}>
                    <MainHeader />
                    <MainNavBar />
                    <PageContentHeader value={pageHeaderText}></PageContentHeader>
                    <Container style={{ overflowX: "auto" }}>
                        <Container style={{ backgroundColor: '#F5F5F5', minWidth: "max-content"}}>
                            <CommonErrorBoundary>
                                <SolveLpMultiplicativeSimplexResponseElement solveLpMultiplicativeSimplexResponseElementProps={solverResults} />
                            </CommonErrorBoundary>
                        </Container>
                    </Container>
                </div>
                <BottomNavBar />
            </>
        )
    }
}

export default SolveLpResultsMultiplicativeSimplex;