import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {SOLVE_LP_DATA_PREFIX, SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX} from "../../../utils/storageConstants.ts";
import {get} from "idb-keyval";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import type {SolveLpBasicSimplexResponseDto} from "../../../api/solver/basic/basicSimplexSolveTypes.ts";
import MainHeader from "../../../components/ui/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import SolveLpBasicSimplexResponseElement
    from "../../../components/layout/solution/basic/SolveLpBasicSimplexResponseElement.tsx";

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

        if (solverResults.solutionStatus === "SOLVED") {
            return (<>
                    <div className={"page-content"}>
                    <MainHeader />
                    <MainNavBar />
                    <PageContentHeader value="LP solved ✅"></PageContentHeader>
                    <Container>
                        <Container style={{ backgroundColor: '#F5F5F5'}}>
                            <SolveLpBasicSimplexResponseElement solveLpBasicSimplexResponseDto={solverResults}/>
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
                                <SolveLpBasicSimplexResponseElement solveLpBasicSimplexResponseDto={solverResults}/>
                            </Container>
                        </Container>
                    </div>
                    <BottomNavBar />
                </>
            )
        }
    }
}

export default SolveLpResultsBasicSimplex