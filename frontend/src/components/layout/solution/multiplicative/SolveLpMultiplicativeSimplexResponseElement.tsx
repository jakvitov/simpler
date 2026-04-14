import type {
    SolveLpMultiplicativeSimplexResponseDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../../../ui/solution/basic/PlainSimplexTableElement.tsx";
import ResultVariableValuesElement from "../../../ui/solution/common/ResultVariableValuesElement.tsx";
import {BlockMath} from "react-katex";
import {renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import MultiplicativeSimplexPhaseTwoSolutionElement
    from "../../../ui/solution/multiplicative/MultiplicativeSimplexPhaseTwoSolutionElement.tsx";
import MultiplicativeSimplexPhaseOneSolutionElement
    from "../../../ui/solution/multiplicative/MultiplicativeSimplexPhaseOneSolutionElement.tsx";

type SolveLpMultiplicativeSimplexResponseElementProps = {
    solveLpMultiplicativeSimplexResponseElementProps: SolveLpMultiplicativeSimplexResponseDto
}

function SolveLpMultiplicativeSimplexResponseElement(props: SolveLpMultiplicativeSimplexResponseElementProps) {
    if (props.solveLpMultiplicativeSimplexResponseElementProps.solutionStatus === "SOLVED") {
        return (<div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpMultiplicativeSimplexResponseElementProps.initialSimplexTable} />
            <MultiplicativeSimplexPhaseOneSolutionElement multiplicativeSimplexPhaseOneSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseOneSolutionDto} />
            <MultiplicativeSimplexPhaseTwoSolutionElement multiplicativeSimplexPhaseTwoSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Optimal variable values found:</h3>
            <ResultVariableValuesElement resultVariableValues={props.solveLpMultiplicativeSimplexResponseElementProps.resultVariableValues} />
            <h3 className={"pt-2"}>Objective function value:</h3>
            <BlockMath math={renderRationalWithNegativeSignOnly(props.solveLpMultiplicativeSimplexResponseElementProps.solutionObjectiveFunctionValue)} />
        </div>)
    } else if (props.solveLpMultiplicativeSimplexResponseElementProps.solutionStatus === "UNBOUNDED") {
        return (<div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpMultiplicativeSimplexResponseElementProps.initialSimplexTable} />
            <MultiplicativeSimplexPhaseOneSolutionElement multiplicativeSimplexPhaseOneSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseOneSolutionDto} />
            <MultiplicativeSimplexPhaseTwoSolutionElement multiplicativeSimplexPhaseTwoSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Optimal variable values found:</h3>
            <BlockMath math={"+/- \\infty"} />
        </div>)
    }
    else if (props.solveLpMultiplicativeSimplexResponseElementProps.solutionStatus === "MAX_ITERATIONS") {
        return <div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpMultiplicativeSimplexResponseElementProps.initialSimplexTable} />
            <MultiplicativeSimplexPhaseOneSolutionElement multiplicativeSimplexPhaseOneSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseOneSolutionDto} />
            <MultiplicativeSimplexPhaseTwoSolutionElement multiplicativeSimplexPhaseTwoSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseTwoSolutionDto} />
        </div>
    } else if (props.solveLpMultiplicativeSimplexResponseElementProps.solutionStatus === "CYCLE") {
        return <div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpMultiplicativeSimplexResponseElementProps.initialSimplexTable} />
            <MultiplicativeSimplexPhaseOneSolutionElement multiplicativeSimplexPhaseOneSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseOneSolutionDto} />
            <MultiplicativeSimplexPhaseTwoSolutionElement multiplicativeSimplexPhaseTwoSolutionDto={props.solveLpMultiplicativeSimplexResponseElementProps.multiplicativeSimplexPhaseTwoSolutionDto} />
        </div>
    }
}

export default SolveLpMultiplicativeSimplexResponseElement;