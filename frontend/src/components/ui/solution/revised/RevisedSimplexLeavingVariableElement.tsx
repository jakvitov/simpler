import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {InlineMath} from "react-katex";

type RevisedSimplexLeavingVariableElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
}

function RevisedSimplexLeavingVariableElement(props: RevisedSimplexLeavingVariableElementProps) {
    if (props.revisedSimplexIterationDto.leavingVariableIndex == null || props.revisedSimplexIterationDto.leavingVariableName == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Leaving variable index:
            <InlineMath math={`${props.revisedSimplexIterationDto.leavingVariableIndex}`} />
            , name:
            <InlineMath math={`${props.revisedSimplexIterationDto.leavingVariableName}`} />
        </p>
    </>)
}

export default RevisedSimplexLeavingVariableElement