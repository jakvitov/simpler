import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {InlineMath} from "react-katex";
import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";

type RevisedSimplexLeavingVariableElementProps = {
    iterationDto: RevisedSimplexIterationDto|MultiplicativeSimplexIterationDto
}

/**
 * Renders leaving variable info in revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexLeavingVariableElement(props: RevisedSimplexLeavingVariableElementProps) {
    if (props.iterationDto.leavingVariableIndex == null || props.iterationDto.leavingVariableName == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Leaving variable basis index:
            <InlineMath math={`${props.iterationDto.leavingVariableIndex}`} />
            , name:
            <InlineMath math={`${props.iterationDto.leavingVariableName}`} />
        </p>
    </>)
}

export default RevisedSimplexLeavingVariableElement