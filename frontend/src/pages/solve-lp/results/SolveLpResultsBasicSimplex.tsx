import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {SOLVE_LP_DATA_PREFIX, SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX} from "../../../utils/storageConstants.ts";
import {get} from "idb-keyval";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import type {SolveLpBasicSimplexResponseDto} from "../../../api/solver/basic/basicSimplexSolveTypes.ts";
import MainHeader from "../../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/general/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import SolveLpBasicSimplexResponseElement
    from "../../../components/layout/solution/basic/SolveLpBasicSimplexResponseElement.tsx";
import CommonErrorBoundary from "../../../components/ui/error/CommonErrorBoundary.tsx";

function SolveLpResultsBasicSimplex() {
    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverResults, setSolverRestults] = useState<SolveLpBasicSimplexResponseDto|null>(null)

    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX + key).then(i => JSON.parse(i) as SolveLpBasicSimplexResponseDto).then(setSolverRestults)
    }, [])

    if (solverInput === null || solverResults === null) {

    } else {
        let pageHeaderText
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
                                <SolveLpBasicSimplexResponseElement solveLpBasicSimplexResponseDto={solverResults}/>
                            </CommonErrorBoundary>
                        </Container>
                    </Container>
                </div>
                <BottomNavBar />
            </>
        )
    }
}

export default SolveLpResultsBasicSimplex;