import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import {get} from "idb-keyval";
import {
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX,
} from "../../../utils/storageConstants.ts";
import MainHeader from "../../../components/ui/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import CommonErrorBoundary from "../../../components/ui/error/CommonErrorBoundary.tsx";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import type {SolveLpRevisedSimlexResponseDto} from "../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import SolveLpRevisedSimlexResponseElement
    from "../../../components/layout/solution/revised/SolveLpRevisedSimlexResponseElement.tsx";

function SolveLpResultsRevisedSimplex() {
    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverResults, setSolverRestults] = useState<SolveLpRevisedSimlexResponseDto|null>(null)

    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_REVISED_SIMPLEX_PREFIX + key).then(i => JSON.parse(i) as SolveLpRevisedSimlexResponseDto).then(setSolverRestults)
    }, [])

    if (solverInput === null || solverResults === null) {

    } else {
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
                    <Container>
                        <Container style={{ backgroundColor: '#F5F5F5'}}>
                            <CommonErrorBoundary>
                                <SolveLpRevisedSimlexResponseElement solveLpRevisedSimlexResponseDto={solverResults} />
                            </CommonErrorBoundary>
                        </Container>
                    </Container>
                </div>
                <BottomNavBar />
            </>
        )
    }
}

export default SolveLpResultsRevisedSimplex