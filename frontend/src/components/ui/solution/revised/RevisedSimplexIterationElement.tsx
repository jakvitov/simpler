import type {
    RevisedSimplexIterationDto
} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {BlockMath} from "react-katex";
import {
    renderMatrix,
    renderMatrixWithName,
    renderRationalWithNegativeSignOnly,
    renderTextVector
} from "../../../../api/common/math.ts";
import RevisedSimplexEnteringVariableElement from "./RevisedSimplexEnteringVariableElement.tsx";
import RevisedSimplexDirectionVectorCalculationElement from "./RevisedSimplexDirectionVectorCalculationElement.tsx";
import RevisedSimplexRatioVectorCalculationElement from "./RevisedSimplexRatioVectorCalculationElement.tsx";
import RevisedSimplexLeavingVariableElement from "./RevisedSimplexLeavingVariableElement.tsx";
import RevisedSimplexUpdatedBasisElement from "./RevisedSimplexUpdatedBasisElement.tsx";

type RevisedSimplexIterationElementProps = {
    revisedSimplexIterationDto: RevisedSimplexIterationDto
    iterationIndex: number
}

function renderRhsCalculation(props: RevisedSimplexIterationElementProps): string {
    let res = "x_b = B^{-1} b  = "
    res += renderMatrix(props.revisedSimplexIterationDto.initialBasisMatrixInverse) + renderMatrix(props.revisedSimplexIterationDto.initialBasisMatrixInverse)
    res += " = "
    res += renderMatrix(props.revisedSimplexIterationDto.XB)
    return res;
}

function renderYTCalculation(props: RevisedSimplexIterationElementProps): string {
    let res = "y^T = c_{B}^{T} B^{-1} = "
    res += renderMatrix(props.revisedSimplexIterationDto.originalSimplexTableReducedCosts) + renderMatrix(props.revisedSimplexIterationDto.initialBasisMatrixInverse);
    res += " = "
    res += renderMatrix(props.revisedSimplexIterationDto.YT)
    return res;
}

function renderNonBasicVariablesCurrentReducedCostsCalculation(props: RevisedSimplexIterationElementProps) {

    let res = "\\bar{c}_j = c_j - y^T A_j \\\\[10pt] \n"

    props.revisedSimplexIterationDto.nonBasicVariablesCurrentReducedCosts.forEach((calculation) => {
        res += `\\bar{c}_{${calculation.variableName}} = ${renderRationalWithNegativeSignOnly(calculation.CJ)} - ${renderMatrix(props.revisedSimplexIterationDto.YT)} ${renderMatrix(calculation.AJ)} = ${renderRationalWithNegativeSignOnly(calculation.result)} \\\\[10pt]`;
    })

    return res;
}

/**
 * Element containing whole revised simplex iteration
 * Renders RevisedSimplexIterationDto
 * @param props
 * @constructor
 */
function RevisedSimplexIterationElement(props: RevisedSimplexIterationElementProps) {
    return (<>
        <hr/>
        <h3 className={"pt-2"}>Start iteration {props.iterationIndex + 1}</h3>
        <p className={"pt-2"}>Current basis:</p>
        <BlockMath math={renderTextVector(props.revisedSimplexIterationDto.currentBasis)} />
        <p className={"pt-2"}>Basis matrix and its inverse:</p>
        <BlockMath math={renderMatrixWithName("B", props.revisedSimplexIterationDto.initialBasisMatrix)} />
        <BlockMath math={renderMatrixWithName("B^{-1}", props.revisedSimplexIterationDto.initialBasisMatrixInverse)} />
        <p className={"pt-2"}>Calculate RHS:</p>
        <BlockMath math={renderRhsCalculation(props)} />
        <p className={"pt-2"}>Get reduced cost of current basis variables in the original simplex table:</p>
        <BlockMath math={renderMatrixWithName("c_{B}^{T}", props.revisedSimplexIterationDto.originalSimplexTableReducedCosts)} />
        <p className={"pt-2"}>Calculate y vector for non basis variables reduced costs calculation:</p>
        <BlockMath math={renderYTCalculation(props)} />
        <p className={"pt-2"}>Calculate reduced cost for each non basic variable:</p>
        <BlockMath math={renderNonBasicVariablesCurrentReducedCostsCalculation(props)} />
        <RevisedSimplexEnteringVariableElement revisedSimplexIterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexDirectionVectorCalculationElement revisedSimplexIterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexRatioVectorCalculationElement revisedSimplexIterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexLeavingVariableElement revisedSimplexIterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexUpdatedBasisElement revisedSimplexIterationDto={props.revisedSimplexIterationDto} />
    </>)
}

export default RevisedSimplexIterationElement