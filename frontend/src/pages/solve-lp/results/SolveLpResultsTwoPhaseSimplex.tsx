import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import {get} from "idb-keyval";
import {
    SOLVE_LP_DATA_PREFIX,
    SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX
} from "../../../utils/storageConstants.ts";
import MainHeader from "../../../components/ui/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import type {SolveLpTwoPhaseSimplexResponseDto} from "../../../api/solver/two-phase/twoPhaseSimplexSolveTypes.ts";
import SolveLpTwoPhaseSimplexResponseElement
    from "../../../components/layout/solution/two-phase/SolveLpTwoPhaseSimplexResponseElement.tsx";
function SolveLpResultsTwoPhaseSimplex() {
    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverResults, setSolverRestults] = useState<SolveLpTwoPhaseSimplexResponseDto|null>(null)

    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_TWO_PHASE_SIMPLEX_PREFIX + key).then(i => JSON.parse(i) as SolveLpTwoPhaseSimplexResponseDto).then(setSolverRestults)
    }, [])

    if (solverInput === null || solverResults === null) {

    } else {
        if (solverResults.solutionStatus === "SOLVED") {
            return (<>
                    <div className={"page-content"}>
                        <MainHeader />
                        <MainNavBar />
                        <PageContentHeader value="LP solved ✅"></PageContentHeader>
                        <Container>
                            <Container style={{ backgroundColor: '#F5F5F5'}}>
                                <SolveLpTwoPhaseSimplexResponseElement solveLpTwoPhaseSimplexResponseDto={solverResults} />
                            </Container>
                        </Container>
                    </div>
                    <BottomNavBar />
                </>
            )
        } else if (solverResults.solutionStatus === "UNBOUNDED") {
            return (<>
                    <div className={"page-content"}>
                        <MainHeader />
                        <MainNavBar />
                        <PageContentHeader value="LP solution unbounded ♾️"></PageContentHeader>
                        <Container>
                            <Container style={{ backgroundColor: '#F5F5F5'}}>
                                <SolveLpTwoPhaseSimplexResponseElement solveLpTwoPhaseSimplexResponseDto={solverResults} />
                            </Container>
                        </Container>
                    </div>
                    <BottomNavBar />
                </>
            )
        } else if (solverResults.solutionStatus === "MAX_ITERATIONS") {
            return (<>
                    <div className={"page-content"}>
                        <MainHeader />
                        <MainNavBar />
                        <PageContentHeader value="LP sol️ution exceeded max iterations ⚠️"></PageContentHeader>
                        <Container>
                            <Container style={{ backgroundColor: '#F5F5F5'}}>
                                <SolveLpTwoPhaseSimplexResponseElement solveLpTwoPhaseSimplexResponseDto={solverResults} />
                            </Container>
                        </Container>
                    </div>
                    <BottomNavBar />
                </>
            )
        }
        else if (solverResults.solutionStatus === "CYCLE") {
            return (<>
                    <div className={"page-content"}>
                        <MainHeader />
                        <MainNavBar />
                        <PageContentHeader value="LP solution includes possible cycle 🔄️"></PageContentHeader>
                        <Container>
                            <Container style={{ backgroundColor: '#F5F5F5'}}>
                                <SolveLpTwoPhaseSimplexResponseElement solveLpTwoPhaseSimplexResponseDto={solverResults} />
                            </Container>
                        </Container>
                    </div>
                    <BottomNavBar />
                </>
            )
        }
    }
}

export default SolveLpResultsTwoPhaseSimplex