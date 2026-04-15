import type {
    NonBasicVariableCurrentReducedCostCalculationDto,
    RevisedSimplexIterationDto
} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import {BlockMath} from "react-katex";
import {
    type Rational,
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

export function renderRevisedSimplexRhsCalculation(initialBasisMatrixInverse: Rational[][], b: Rational[][], xB: Rational[][]): string {
    let res = "x_b = B^{-1} b  = "
    res += renderMatrix(initialBasisMatrixInverse) + renderMatrix(b)
    res += " = "
    res += renderMatrix(xB)
    return res;
}

export function renderRevisedSimplexYTCalculation(originalSimplexTableReducedCosts: Rational[][], initialBasisMatrixInverse: Rational[][], yT: Rational[][]): string {
    let res = "y^T = c_{B}^{T} B^{-1} = "
    res += renderMatrix(originalSimplexTableReducedCosts) + renderMatrix(initialBasisMatrixInverse);
    res += " = "
    res += renderMatrix(yT)
    return res;
}

export function renderRevisedSimplexNonBasicVariablesCurrentReducedCostsCalculation(nonBasicVariablesCurrentReducedCosts: NonBasicVariableCurrentReducedCostCalculationDto[], yT: Rational[][]) {

    let res = "\\bar{c}_j = c_j - y^T A_j \\\\[10pt] \n"

    nonBasicVariablesCurrentReducedCosts.forEach((calculation) => {
        res += `\\bar{c}_{${calculation.variableName}} = ${renderRationalWithNegativeSignOnly(calculation.CJ)} - ${renderMatrix(yT)} ${renderMatrix(calculation.AJ)} = ${renderRationalWithNegativeSignOnly(calculation.result)} \\\\[10pt]`;
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
        <BlockMath math={renderRevisedSimplexRhsCalculation(props.revisedSimplexIterationDto.initialBasisMatrixInverse, props.revisedSimplexIterationDto.b, props.revisedSimplexIterationDto.XB)} />
        <p className={"pt-2"}>Get reduced cost of current basis variables in the original simplex table:</p>
        <BlockMath math={renderMatrixWithName("c_{B}^{T}", props.revisedSimplexIterationDto.originalSimplexTableReducedCosts)} />
        <p className={"pt-2"}>Calculate y vector for non basis variables reduced costs calculation:</p>
        <BlockMath math={renderRevisedSimplexYTCalculation(props.revisedSimplexIterationDto.originalSimplexTableReducedCosts, props.revisedSimplexIterationDto.initialBasisMatrixInverse, props.revisedSimplexIterationDto.YT)} />
        <p className={"pt-2"}>Calculate reduced cost for each non basic variable:</p>
        <BlockMath math={renderRevisedSimplexNonBasicVariablesCurrentReducedCostsCalculation(props.revisedSimplexIterationDto.nonBasicVariablesCurrentReducedCosts, props.revisedSimplexIterationDto.YT)} />
        <RevisedSimplexEnteringVariableElement iterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexDirectionVectorCalculationElement iterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexRatioVectorCalculationElement iterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexLeavingVariableElement iterationDto={props.revisedSimplexIterationDto} />
        <RevisedSimplexUpdatedBasisElement iterationDto={props.revisedSimplexIterationDto} />
    </>)
}

export default RevisedSimplexIterationElement