import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {SOLVE_LP_DATA_PREFIX, SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX} from "../../../utils/storageConstants.ts";
import {get} from "idb-keyval";
import type BasicSimplexSolveResponse from "../../../api/solver/basic/basicSimplexSolveTypes.ts";
import MainHeader from "../../../components/ui/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/PageContentHeader.tsx";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import LpDefinition from "../../../components/layout/solution/LpDefinition.tsx";
import {Container} from "react-bootstrap";
import ProblemSpecificationDetails from "../../../components/ui/solution/ProblemSpecificationDetails.tsx";
import type {SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";

function SolveLpResultsBasicSimplex() {
    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverResults, setSolverRestults] = useState<BasicSimplexSolveResponse|null>(null)

    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_BASIC_SIMPLEX_PREFIX + key).then(i => JSON.parse(i) as BasicSimplexSolveResponse).then(setSolverRestults)
    }, [])

    if (solverInput === null || solverResults === null) {

    } else {
        return (<>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <PageContentHeader value="Basic simplex LP solution"></PageContentHeader>
                <Container>
                    <Container style={{ backgroundColor: '#F5F5F5'}}>
                        <ProblemSpecificationDetails target={solverInput.optimisationTarget} method={"BASIC_SIMPLEX"} status={solverResults.result} />

                        <LpDefinition parsedProblem={solverResults.parsedLP}/>
                    </Container>
                </Container>
            </div>
            <BottomNavBar />
        </>)
    }
}

export default SolveLpResultsBasicSimplex