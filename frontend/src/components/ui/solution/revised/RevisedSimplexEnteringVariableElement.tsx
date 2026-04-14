import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {InlineMath} from "react-katex";
import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";

type RevisedSimplexEnteringVariableElementProps = {
    iterationDto: RevisedSimplexIterationDto|MultiplicativeSimplexIterationDto
}

/**
 * Renders entering variable info in revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexEnteringVariableElement(props: RevisedSimplexEnteringVariableElementProps) {
    if (props.iterationDto.enteringVariableIndex == null || props.iterationDto.enteringVariableName == null) {
        return (<></>)
    }
    
    return (<>
        <p className={"pt-2"}>Entering variable index:
            <InlineMath math={`${props.iterationDto.enteringVariableIndex}`} />
            , name:
            <InlineMath math={`${props.iterationDto.enteringVariableName}`} />
        </p>
    </>)
}

export default RevisedSimplexEnteringVariableElement