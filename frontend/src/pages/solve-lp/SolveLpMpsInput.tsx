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
import {LAST_MPS_INPUT_DATA, SOLVER_CONFIGURATION_KEY,} from "../../utils/storageConstants.ts";
import {useNavigate} from "react-router-dom";
import {handleSolveRequestBasedOnSolverMethod} from "../../api/solver/solverApiFacade.tsx";
import {fetchHealthCheck} from "../../api/manage/healthApi.ts";

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

    const handleSolveMpsButtonClick = async () => {

        //Null when not set in settings, backend will use defaults
        const solverConfigurationStr: string|null = localStorage.getItem(SOLVER_CONFIGURATION_KEY);
        let solverConfiguration: SolverConfiguration|null = null
        if (solverConfigurationStr != null) {
            solverConfiguration = JSON.parse(solverConfigurationStr)
        }

        //Get current BE version
        const healthResponse = await fetchHealthCheck();
        let requestVersion: string;
        if (healthResponse == null || healthResponse.version == null) {
            //BE version in request is used for FE hash creation, this forces new unknown hash to be created
            //causes cache miss
            requestVersion = "ERROR_OBTAINING_REQUEST_VERSION" + Math.random().toString(32);
        } else {
            requestVersion = healthResponse.version;
        }

        const request: SolveLpRequest = {
            data: mpsInput,
            optimisationTarget: optimisationTarget,
            method: solverMethod,
            solverConfiguration: solverConfiguration,
            version: requestVersion
        }

        handleSolveRequestBasedOnSolverMethod(request, navigate);
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
