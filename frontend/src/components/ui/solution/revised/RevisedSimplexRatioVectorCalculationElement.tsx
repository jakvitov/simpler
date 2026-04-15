import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {renderMatrix} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";
import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";

type RevisedSimplexRatioVectorCalculationElementProps = {
    iterationDto: RevisedSimplexIterationDto|MultiplicativeSimplexIterationDto
}

function renderRatioVectorCalculation(props: RevisedSimplexRatioVectorCalculationElementProps): string {
    if (props.iterationDto.ratioVector == null || props.iterationDto.directionVector == null) {
        throw new Error("Ratio vector or d vector are being rendered while being null!");
    }


    let res = "\\theta = x_B / d = "
    res += renderMatrix(props.iterationDto.XB) + " / " + renderMatrix(props.iterationDto.directionVector)
    res += " = "
    res += renderMatrix([props.iterationDto.ratioVector])
    return res
}

/**
 * Renders ratio vector calculation for revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexRatioVectorCalculationElement(props: RevisedSimplexRatioVectorCalculationElementProps) {
    if (props.iterationDto.ratioVector == null || props.iterationDto.directionVector == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate ratio vector:</p>
        <BlockMath math={renderRatioVectorCalculation(props)} />
    </>)
}

export default RevisedSimplexRatioVectorCalculationElement