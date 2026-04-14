import type {RevisedSimplexIterationDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {renderMatrix} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";
import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";

type RevisedSimplexDirectionVectorCalculationElementProps = {
    iterationDto: RevisedSimplexIterationDto|MultiplicativeSimplexIterationDto
}

function renderDirectionVectorCalculation(props: RevisedSimplexDirectionVectorCalculationElementProps): string {
    if (props.iterationDto.directionVector == null || props.iterationDto.enteringVariableColumnInOriginalSimplexTable == null) {
        throw new Error("Direction vector or entering variable column are null, while being rendred.")
    }

    let res = "d = B^{-1} A_{j} = "
    res += renderMatrix(props.iterationDto.initialBasisMatrixInverse) + renderMatrix(props.iterationDto.enteringVariableColumnInOriginalSimplexTable)
    res += " = "
    res += renderMatrix(props.iterationDto.directionVector)
    return res
}

/**
 * Renders direction vector calculation info in revised simplex iteration
 * @param props
 * @constructor
 */
function RevisedSimplexDirectionVectorCalculationElement(props: RevisedSimplexDirectionVectorCalculationElementProps) {
    if (props.iterationDto.directionVector == null || props.iterationDto.enteringVariableColumnInOriginalSimplexTable == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate direction vector:</p>
        <BlockMath math={renderDirectionVectorCalculation(props)} />
    </>)
}

export default RevisedSimplexDirectionVectorCalculationElement