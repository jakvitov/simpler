import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";

type RevisedSimplexUpdatedBasisElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
}

function RevisedSimplexUpdatedBasisElement(props: RevisedSimplexUpdatedBasisElementProps) {
    if (props.revisedSimplexIterationDto.updatedBasis == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Updated basis after variable switch:</p>
        <BlockMath math={renderTextVector(props.revisedSimplexIterationDto.updatedBasis)} />
    </>)
}

export default RevisedSimplexUpdatedBasisElement;