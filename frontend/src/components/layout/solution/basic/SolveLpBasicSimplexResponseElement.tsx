import type {SolveLpBasicSimplexResponseDto} from "../../../../api/solver/basic/basicSimplexSolveTypes.ts";
import PageContentHeader from "../../../ui/general/PageContentHeader.tsx";
import PlainSimplexTableElement from "../../../ui/solution/basic/PlainSimplexTableElement.tsx";
import BasicSimplexIterationElement from "../../../ui/solution/basic/BasicSimplexIterationElement.tsx";
import ResultVariableValuesElement from "../../../ui/solution/common/ResultVariableValuesElement.tsx";
import {BlockMath} from "react-katex";
import {renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";

type SolveLpBasicSimplexResponseElementProps = {
    solveLpBasicSimplexResponseDto: SolveLpBasicSimplexResponseDto
}


/**
 * Element with whole basic simplex problem solution
 * Renders SolveLpBasicSimplexResponseDto
 * @param props
 * @constructor
 */
function SolveLpBasicSimplexResponseElement(props: SolveLpBasicSimplexResponseElementProps) {
    if (props.solveLpBasicSimplexResponseDto.solutionStatus === "SOLVED") {
        return (
            <div className="basicSimplexSolution">
                <h3 className={"pt-2"}>Initial simplex table:</h3>
                <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.initialSimplexTable} />
                {(props.solveLpBasicSimplexResponseDto.iterations != null) ? props.solveLpBasicSimplexResponseDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
                <h3 className={"pt-2"}>Final simplex table:</h3>
                <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.finalSimplexTable} />
                <h3 className={"pt-2"}>Optimal variable values found:</h3>
                <ResultVariableValuesElement resultVariableValues={props.solveLpBasicSimplexResponseDto.resultVariableValues} />
                <h3 className={"pt-2"}>Objective function value:</h3>
                <BlockMath math={renderRationalWithNegativeSignOnly(props.solveLpBasicSimplexResponseDto.solutionObjectiveFunctionValue)} />
            </div>
        )
    } else if (props.solveLpBasicSimplexResponseDto.solutionStatus === "UNBOUNDED") {
        return (<div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.initialSimplexTable} />
            {(props.solveLpBasicSimplexResponseDto.iterations != null) ? props.solveLpBasicSimplexResponseDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
            <h3 className={"pt-2"}>Objective function value:</h3>
            <BlockMath math={"+/- \\infty"} />
        </div>)
    } else if (props.solveLpBasicSimplexResponseDto.solutionStatus === "MAX_ITERATIONS") {
        return (<div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.initialSimplexTable} />
            {(props.solveLpBasicSimplexResponseDto.iterations != null) ? props.solveLpBasicSimplexResponseDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
            <h3 className={"pt-2"}>Final simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.finalSimplexTable} />
        </div>)
    } else if (props.solveLpBasicSimplexResponseDto.solutionStatus === "CYCLE") {
        return (<div className="basicSimplexSolution">
            <h3 className={"pt-2"}>Initial simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.initialSimplexTable} />
            {(props.solveLpBasicSimplexResponseDto.iterations != null) ? props.solveLpBasicSimplexResponseDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
            <h3 className={"pt-2"}>Final simplex table:</h3>
            <PlainSimplexTableElement simplexTable={props.solveLpBasicSimplexResponseDto.finalSimplexTable} />
        </div>)
    }
    else {
        let text = "LP solution status " + props.solveLpBasicSimplexResponseDto.solutionStatus + " not implemented yet."
        return (
            <div className="basicSimplexSolution">
                <PageContentHeader value={text}></PageContentHeader>
            </div>
        )
    }
}

export default SolveLpBasicSimplexResponseElement

