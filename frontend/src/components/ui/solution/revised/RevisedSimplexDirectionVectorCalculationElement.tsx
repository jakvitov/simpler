import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {renderMatrix} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type RevisedSimplexDirectionVectorCalculationElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
}

function renderDirectionVectorCalculation(props: RevisedSimplexDirectionVectorCalculationElementProps): string {
    if (props.revisedSimplexIterationDto.directionVector == null || props.revisedSimplexIterationDto.enteringVariableColumnInOriginalSimplexTable == null) {
        throw new Error("Direction vector or entering variable column are null, while being rendred.")
    }

    let res = "d = B^{-1} A_{j} = "
    res += renderMatrix(props.revisedSimplexIterationDto.initialBasisMatrixInverse) + renderMatrix(props.revisedSimplexIterationDto.enteringVariableColumnInOriginalSimplexTable)
    res += " = "
    res += renderMatrix(props.revisedSimplexIterationDto.directionVector)
    return res
}

/**
 * Renders direction vector calculation info in revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexDirectionVectorCalculationElement(props: RevisedSimplexDirectionVectorCalculationElementProps) {
    if (props.revisedSimplexIterationDto.directionVector == null || props.revisedSimplexIterationDto.enteringVariableColumnInOriginalSimplexTable == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate direction vector:</p>
        <BlockMath math={renderDirectionVectorCalculation(props)} />
    </>)
}

export default RevisedSimplexDirectionVectorCalculationElement