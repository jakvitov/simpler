import type {SolveLpTwoPhaseSimplexResponseDto} from "../../../../api/solver/two-phase/twoPhaseSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../../../ui/solution/basic/PlainSimplexTableElement.tsx";
import TwoPhaseSimplexPhaseOneSolutionElement
    from "../../../ui/solution/two-phase/TwoPhaseSimplexPhaseOneSolutionElement.tsx";
import TwoPhaseSimplexPhaseTwoSolutionElement
    from "../../../ui/solution/two-phase/TwoPhaseSimplexPhaseTwoSolutionElement.tsx";
import ResultVariableValuesElement from "../../../ui/solution/common/ResultVariableValuesElement.tsx";
import {BlockMath} from "react-katex";
import {renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";

type SolveLpTwoPhaseSimplexResponseElementProps = {
    solveLpTwoPhaseSimplexResponseDto: SolveLpTwoPhaseSimplexResponseDto
}


function SolveLpTwoPhaseSimplexResponseElement(props: SolveLpTwoPhaseSimplexResponseElementProps) {
    if (props.solveLpTwoPhaseSimplexResponseDto.solutionStatus === "SOLVED") {
        return (<div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpTwoPhaseSimplexResponseDto.initialSimplexTable} />
            <TwoPhaseSimplexPhaseOneSolutionElement twoPhaseSimplexPhaseOneSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseOneSolutionDto} />
            <TwoPhaseSimplexPhaseTwoSolutionElement twoPhaseSimplexPhaseTwoSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Optimal variable values found:</h3>
            <ResultVariableValuesElement resultVariableValues={props.solveLpTwoPhaseSimplexResponseDto.resultVariableValues} />
            <h3 className={"pt-2"}>Objective function value:</h3>
            <BlockMath math={renderRationalWithNegativeSignOnly(props.solveLpTwoPhaseSimplexResponseDto.solutionObjectiveFunctionValue)} />
        </div>)
    } else if (props.solveLpTwoPhaseSimplexResponseDto.solutionStatus === "UNBOUNDED") {
        return (<div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpTwoPhaseSimplexResponseDto.initialSimplexTable} />
            <TwoPhaseSimplexPhaseOneSolutionElement twoPhaseSimplexPhaseOneSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseOneSolutionDto} />
            <TwoPhaseSimplexPhaseTwoSolutionElement twoPhaseSimplexPhaseTwoSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseTwoSolutionDto} />
            <h3 className={"pt-2"}>Objective function value:</h3>
            <BlockMath math={"+/- \\infty"} />
        </div>)
    }
    else if (props.solveLpTwoPhaseSimplexResponseDto.solutionStatus === "MAX_ITERATIONS") {
        <div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpTwoPhaseSimplexResponseDto.initialSimplexTable} />
            <TwoPhaseSimplexPhaseOneSolutionElement twoPhaseSimplexPhaseOneSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseOneSolutionDto} />
            <TwoPhaseSimplexPhaseTwoSolutionElement twoPhaseSimplexPhaseTwoSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseTwoSolutionDto} />
        </div>
    } else if (props.solveLpTwoPhaseSimplexResponseDto.solutionStatus === "CYCLE") {
        <div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpTwoPhaseSimplexResponseDto.initialSimplexTable} />
            <TwoPhaseSimplexPhaseOneSolutionElement twoPhaseSimplexPhaseOneSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseOneSolutionDto} />
            <TwoPhaseSimplexPhaseTwoSolutionElement twoPhaseSimplexPhaseTwoSolutionDto={props.solveLpTwoPhaseSimplexResponseDto.phaseTwoSolutionDto} />
        </div>
    }
}

export default SolveLpTwoPhaseSimplexResponseElement;