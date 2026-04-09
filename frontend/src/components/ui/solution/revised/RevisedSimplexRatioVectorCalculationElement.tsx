import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {renderMatrix} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type RevisedSimplexRatioVectorCalculationElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
}

function renderRatioVectorCalculation(props: RevisedSimplexRatioVectorCalculationElementProps): string {
    if (props.revisedSimplexIterationDto.ratioVector == null || props.revisedSimplexIterationDto.directionVector == null) {
        throw new Error("Ratio vector or d vector are being rendered while being null!");
    }


    let res = "\\theta = x_B / d = "
    res += renderMatrix(props.revisedSimplexIterationDto.xB) + " / " + renderMatrix(props.revisedSimplexIterationDto.directionVector)
    res += " = "
    res += renderMatrix([props.revisedSimplexIterationDto.ratioVector])
    return res
}


function RevisedSimplexRatioVectorCalculationElement(props: RevisedSimplexRatioVectorCalculationElementProps) {
    if (props.revisedSimplexIterationDto.ratioVector == null || props.revisedSimplexIterationDto.directionVector == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate ratio vector:</p>
        <BlockMath math={renderRatioVectorCalculation(props)} />
    </>)
}

export default RevisedSimplexRatioVectorCalculationElement