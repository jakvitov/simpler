import MainHeader from "../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve-input/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve-input/SolverAlgorithmRadial.tsx";
import ConfirmButton from "../../components/ui/general/ConfirmButton.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import {useRef, useState} from "react";
import type {
    OptimisationTarget,
    SolveLpRequest,
    SolverConfiguration,
    SolverMethods
} from "../../api/solver/solveLpTypes.ts";
import LPInteractiveInputForm, {
    type LPInteractiveInputHandle,
    type LPInteractiveInputState
} from "../../components/layout/solve-input/InteractiveLpInput.tsx";
import {SOLVER_CONFIGURATION_KEY} from "../../utils/storageConstants.ts";
import {handleSolveRequestBasedOnSolverMethod} from "../../api/solver/solverApiFacade.tsx";
import {useNavigate} from "react-router-dom";
import {fetchHealthCheck} from "../../api/manage/healthApi.ts";

function lPInteractiveInputStateToMps(interactiveLpInputData: LPInteractiveInputState|undefined): string {

    if (interactiveLpInputData == null) {
        alert("Cannot work with empty interactive input data!")
        return ""
    }

    let res = "NAME INTERACTIVE INPUT \n"
    res += "ROWS \n"
    res += "N Z\n"

    interactiveLpInputData.operators.forEach((operator, i) => {
        if (operator === "=") {
            res += "E ROW_" + i + "\n";
        } else if (operator === ">=") {
            res += "G ROW_" + i + "\n";
        } else if (operator === "<=") {
            res += "L ROW_" + i + "\n";
        }
    })

    res += "COLUMNS \n"

    interactiveLpInputData.variables.forEach((variableName, columnIndex) => {
        for (let rowIndex = 0; rowIndex < interactiveLpInputData.operators.length; rowIndex++) {
            let dataValueForVariable = interactiveLpInputData.rows[rowIndex][columnIndex].length == 0 ? "0" : interactiveLpInputData.rows[rowIndex][columnIndex]

            res += variableName + " ROW_" + rowIndex + " " + dataValueForVariable + "\n"
        }
        res += variableName + " Z " + interactiveLpInputData.objective[columnIndex] + "\n"
    })

    res += "RHS \n"

    interactiveLpInputData.rhs.forEach((rhs, index) => {
        res += `RHS1 ROW_${index} ${rhs} \n`
    })

    res += "ENDATA"
    console.log(res)
    return res
}

function SolveLpInteractiveInput() {

    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")
    const [optimisationTarget, setOptimisationTarget] = useState<OptimisationTarget>("MIN")
    const navigate = useNavigate()

    const ref = useRef<LPInteractiveInputHandle>(null);

    const handleExtract = async () => {
        const data = ref.current?.getData();

        //Null when not set in settings, backend will use defaults
        const solverConfigurationStr: string | null = localStorage.getItem(SOLVER_CONFIGURATION_KEY);
        let solverConfiguration: SolverConfiguration | null = null
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
            data: lPInteractiveInputStateToMps(data),
            optimisationTarget: optimisationTarget,
            method: solverMethod,
            solverConfiguration: solverConfiguration,
            version: requestVersion
        };
        handleSolveRequestBasedOnSolverMethod(request, navigate);

    };

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
                    <Container style={{ backgroundColor: '#F5F5F5'}}>
                        <LPInteractiveInputForm ref={ref}/>
                    </Container>
                    <ConfirmButton onChange={handleExtract}></ConfirmButton>
                </Container>
            </div>
            <BottomNavBar />
        </>
    )
}

export default SolveLpInteractiveInput