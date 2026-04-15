import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import {renderMatrix, renderMatrixWithName} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type MultiplicativeSimplexElementaryMatrixInverseCalculationElementProps = {
    multiplicativeSimplexIterationDto: MultiplicativeSimplexIterationDto
}

function renderElementaryMatrixCreation(props: MultiplicativeSimplexElementaryMatrixInverseCalculationElementProps) {
    if (props.multiplicativeSimplexIterationDto.directionVector == null || props.multiplicativeSimplexIterationDto.elementaryMatrix == null) {
        throw new Error("Cannot render elementary matrix calculation. One of the neccessary components are null.");
    }

    let res = "d = "
    res += renderMatrix(props.multiplicativeSimplexIterationDto.directionVector)
    res += ", "
    res +=  "E =\n" +
        "\\begin{pmatrix}\n" +
        "1 &        &        & d_1 &        \\\\\n" +
        "  & 1      &        & d_2 &        \\\\\n" +
        "  &        & \\ddots & \\vdots &     \\\\\n" +
        "  &        &        & d_r &        \\\\\n" +
        "  &        &        & \\vdots & 1\n" +
        "\\end{pmatrix}"

    res += " = "
    res += renderMatrix(props.multiplicativeSimplexIterationDto.elementaryMatrix)
    return res;
}

/**
 * Renders elementary matrix inverse calculation for multiplicative simplex
 * @param props
 * @constructor
 */
function MultiplicativeSimplexElementaryMatrixInverseCalculationElement(props: MultiplicativeSimplexElementaryMatrixInverseCalculationElementProps) {
    if (props.multiplicativeSimplexIterationDto.elementaryMatrix == null || props.multiplicativeSimplexIterationDto.elementaryMatrixInverse == null || props.multiplicativeSimplexIterationDto.directionVector == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Calculate elementary matrix inverse:</p>
        <BlockMath math={renderElementaryMatrixCreation(props)} />
        <BlockMath math={renderMatrixWithName("E^{-1}", props.multiplicativeSimplexIterationDto.elementaryMatrixInverse)} />
    </>)
}

export default MultiplicativeSimplexElementaryMatrixInverseCalculationElement;