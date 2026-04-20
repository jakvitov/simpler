import MainHeader from "../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import {Container} from "react-bootstrap";
import SolverInputType from "../../components/layout/solve-input/SolverInputType.tsx";
import SolverAlgorithmRadial from "../../components/layout/solve-input/SolverAlgorithmRadial.tsx";
import ConfirmButton from "../../components/ui/general/ConfirmButton.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import {useRef, useState} from "react";
import type {OptimisationTarget, SolverMethods} from "../../api/solver/solveLpTypes.ts";
import InteractiveLpInput, {
    type LPInteractiveInputHandle,
    type LPInteractiveInputState
} from "../../components/layout/solve-input/InteractiveLpInput.tsx";

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
            res += "GE ROW_" + i + "\n";
        } else if (operator === "<=") {
            res += "LE ROW_" + i + "\n";
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
    return res
}

function SolveLpInteractiveInput() {

    const [solverMethod, setSolverMethod] = useState<SolverMethods>("BASIC_SIMPLEX")
    const [optimisationTarget, setOptimisationTarget] = useState<OptimisationTarget>("MIN")

    const ref = useRef<LPInteractiveInputHandle>(null);

    const handleExtract = () => {
        const data = ref.current?.getData();
        console.log("LP Data:", data);
        console.log("Transformed MPS: " + lPInteractiveInputStateToMps(data))
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
                        <InteractiveLpInput ref={ref}/>
                    </Container>
                    <ConfirmButton onChange={handleExtract}></ConfirmButton>
                </Container>
            </div>
            <BottomNavBar />
        </>
    )
}

export default SolveLpInteractiveInput