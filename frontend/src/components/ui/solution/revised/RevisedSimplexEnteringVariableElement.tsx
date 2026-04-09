import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {InlineMath} from "react-katex";

type RevisedSimplexEnteringVariableElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
}

function RevisedSimplexEnteringVariableElement(props: RevisedSimplexEnteringVariableElementProps) {
    if (props.revisedSimplexIterationDto.enteringVariableIndex == null || props.revisedSimplexIterationDto.enteringVariableName == null) {
        return (<></>)
    }
    
    return (<>
        <p className={"pt-2"}>Entering variable index: 
            <InlineMath math={`${props.revisedSimplexIterationDto.enteringVariableIndex}`} />
            , name:
            <InlineMath math={`${props.revisedSimplexIterationDto.enteringVariableName}`} />
        </p>
    </>)
}

export default RevisedSimplexEnteringVariableElement