import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";
import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";

type RevisedSimplexUpdatedBasisElementProps = {
    iterationDto: RevisedSimplexIterationDto|MultiplicativeSimplexIterationDto
}

/**
 * Renders udpated basis after variable switch in revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexUpdatedBasisElement(props: RevisedSimplexUpdatedBasisElementProps) {
    if (props.iterationDto.updatedBasis == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Updated basis after variable switch:</p>
        <BlockMath math={renderTextVector(props.iterationDto.updatedBasis)} />
    </>)
}

export default RevisedSimplexUpdatedBasisElement;