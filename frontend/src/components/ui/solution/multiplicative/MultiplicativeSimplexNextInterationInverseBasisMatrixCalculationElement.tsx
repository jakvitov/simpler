import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import {renderMatrix} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElementProps = {
    multiplicativeSimplexIterationDto: MultiplicativeSimplexIterationDto
}

function renderNextIterationInverseBasisMatrixCalculation(props: MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElementProps) {
    if (props.multiplicativeSimplexIterationDto.elementaryMatrixInverse == null || props.multiplicativeSimplexIterationDto.nextIterationBasisInverse == null) {
        throw new Error("Cannot render next iteration inverse basis matrix calculation. One of the required components is null.");
    }

    let res = "B_{i+1}^{-1} = E^{-1} B^{-1} = "
    res += renderMatrix(props.multiplicativeSimplexIterationDto.elementaryMatrixInverse);
    res += renderMatrix(props.multiplicativeSimplexIterationDto.initialBasisMatrixInverse);
    res += " = "
    res += renderMatrix(props.multiplicativeSimplexIterationDto.nextIterationBasisInverse);
    return res;
}

/**
 * Renders next iterations inverse basis matrix for multiplicative simplex
 * @param props
 * @constructor
 */
function MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElement(props: MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElementProps) {
    if (props.multiplicativeSimplexIterationDto.elementaryMatrixInverse == null || props.multiplicativeSimplexIterationDto.nextIterationBasisInverse == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate next iteration inverse basis matrix:</p>
        <BlockMath math={renderNextIterationInverseBasisMatrixCalculation(props)} />
    </>)
}

export default MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElement;