import MainHeader from "../../../components/ui/MainHeader.tsx";
import MainNavBar from "../../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../../components/ui/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import BottomNavBar from "../../../components/layout/BottomNavBar.tsx";
import {useEffect, useState} from "react";
import {get} from "idb-keyval";
import {SOLVE_LP_DATA_PREFIX, SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX} from "../../../utils/storageConstants.ts";
import {useParams} from "react-router-dom";
import type {SolveLpErrorResponse, SolveLpRequest} from "../../../api/solver/solveLpTypes.ts";
import ErrorEnumerationContainer from "../../../components/ui/ErrorEnumerationContainer.tsx";

function SolveLpResultsError() {

    const { key } = useParams<{ key: string }>();

    const [solverInput, setSolverInput] = useState<SolveLpRequest|null>(null)
    const [solverError, setSolverError] = useState<SolveLpErrorResponse|null>(null)


    useEffect(() => {
        get(SOLVE_LP_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpRequest).then(setSolverInput)
        get(SOLVE_LP_SOLUTION_ERROR_DATA_PREFIX + key).then(i => JSON.parse(i) as SolveLpErrorResponse).then(setSolverError)
    }, [])


    if (solverError === null || solverInput === null) {
        return <></>
    }

    return (<>
        <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
            <Container>
                <PageContentHeader value="Error occurred during solution ⚠️"></PageContentHeader>
                <ErrorEnumerationContainer mainReason={"Encountered errors during solution attempt:"} errors={solverError.errors} />
            </Container>
        </div>
        <BottomNavBar />
    </>)
}

export default SolveLpResultsError;