import type {SolveLpRevisedSimlexResponseDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../../../ui/solution/basic/PlainSimplexTableElement.tsx";
import ResultVariableValuesElement from "../../../ui/solution/common/ResultVariableValuesElement.tsx";
import {BlockMath} from "react-katex";
import {renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import RevisedSimplexPhaseOneSolutionElement
    from "../../../ui/solution/revised/RevisedSimplexPhaseOneSolutionElement.tsx";
import RevisedSimplexPhaseTwoSolutionElement
    from "../../../ui/solution/revised/RevisedSimplexPhaseTwoSolutionElement.tsx";

type SolveLpRevisedSimlexResponseElementProps = {
    solveLpRevisedSimlexResponseDto: SolveLpRevisedSimlexResponseDto
}

function SolveLpRevisedSimplexResponseElement(props: SolveLpRevisedSimlexResponseElementProps) {
    if (props.solveLpRevisedSimlexResponseDto.solutionStatus === "SOLVED") {
        return (<div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpRevisedSimlexResponseDto.initialSimplexTable} />
            <RevisedSimplexPhaseOneSolutionElement revisedSimplexPhaseOneSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseOneSolution} />
            <RevisedSimplexPhaseTwoSolutionElement revisedSimplexPhaseTwoSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Optimal variable values found:</h3>
            <ResultVariableValuesElement resultVariableValues={props.solveLpRevisedSimlexResponseDto.resultVariableValues} />
            <h3 className={"pt-2"}>Objective function value:</h3>
            <BlockMath math={renderRationalWithNegativeSignOnly(props.solveLpRevisedSimlexResponseDto.solutionObjectiveFunctionValue)} />
        </div>)
    } else if (props.solveLpRevisedSimlexResponseDto.solutionStatus === "UNBOUNDED") {
        return (<div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpRevisedSimlexResponseDto.initialSimplexTable} />
            <RevisedSimplexPhaseOneSolutionElement revisedSimplexPhaseOneSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseOneSolution} />
            <RevisedSimplexPhaseTwoSolutionElement revisedSimplexPhaseTwoSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Optimal variable values found:</h3>
            <BlockMath math={"+/- \\infty"} />
        </div>)
    }
    else if (props.solveLpRevisedSimlexResponseDto.solutionStatus === "MAX_ITERATIONS") {
        return <div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpRevisedSimlexResponseDto.initialSimplexTable} />
            <RevisedSimplexPhaseOneSolutionElement revisedSimplexPhaseOneSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseOneSolution} />
            <RevisedSimplexPhaseTwoSolutionElement revisedSimplexPhaseTwoSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseTwoSolutionDto} />
        </div>
    } else if (props.solveLpRevisedSimlexResponseDto.solutionStatus === "CYCLE") {
        return <div className="revisedSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpRevisedSimlexResponseDto.initialSimplexTable} />
            <RevisedSimplexPhaseOneSolutionElement revisedSimplexPhaseOneSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseOneSolution} />
            <RevisedSimplexPhaseTwoSolutionElement revisedSimplexPhaseTwoSolutionDto={props.solveLpRevisedSimlexResponseDto.revisedSimplexPhaseTwoSolutionDto} />
        </div>
    }
}

export default SolveLpRevisedSimplexResponseElement