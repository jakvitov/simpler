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
import InitialSimplexTable from "../../../components/layout/solution/InitialSimplexTable.tsx";
import SimplexTableObjectiveRowArrow from "../../../components/ui/solution/SimplexTableObjectiveRowArrow.tsx";
import SimplexTableRowAddititionArrows from "../../../components/ui/solution/SimplexTableRowAddititionArrows.tsx";
import {demoMatrix, demoRational, type Rational} from "../../../api/common/math.ts";
import SimplexTableWithTVec from "../../../components/ui/solution/SimplexTableWithTVec.tsx";

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

        //todo demo
        let testMap = new Map<number, Rational>()
        testMap.set(1, demoRational())

        return (<>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <PageContentHeader value="Basic simplex LP solution"></PageContentHeader>
                <Container>
                    <Container style={{ backgroundColor: '#F5F5F5'}}>
                        <ProblemSpecificationDetails target={solverInput.optimisationTarget} method={"BASIC_SIMPLEX"} status={solverResults.result} />
                        <LpDefinition parsedProblem={solverResults.parsedLP}/>
                        <InitialSimplexTable initialST={solverResults.initialST} />
                        <SimplexTableObjectiveRowArrow simplexTable={solverResults.initialST} arrowColumn={1} demo={false} />

                        <SimplexTableRowAddititionArrows simplexTable={solverResults.initialST} sourceRowIndex={0} targetRows={testMap} />

                        <SimplexTableWithTVec simplexTable={solverResults.initialST} tVec={demoMatrix(1, 2)[0]} pivotRowIndex={0} pivotColIndex={0} />

                    </Container>
                </Container>
            </div>
            <BottomNavBar />
        </>)
    }
}

export default SolveLpResultsBasicSimplex