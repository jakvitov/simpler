import type {
    MultiplicativeSimplexIterationDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import {BlockMath} from "react-katex";
import {renderMatrixWithName, renderTextVector} from "../../../../api/common/math.ts";
import RevisedSimplexEnteringVariableElement from "../revised/RevisedSimplexEnteringVariableElement.tsx";
import RevisedSimplexDirectionVectorCalculationElement
    from "../revised/RevisedSimplexDirectionVectorCalculationElement.tsx";
import RevisedSimplexRatioVectorCalculationElement from "../revised/RevisedSimplexRatioVectorCalculationElement.tsx";
import RevisedSimplexLeavingVariableElement from "../revised/RevisedSimplexLeavingVariableElement.tsx";
import RevisedSimplexUpdatedBasisElement from "../revised/RevisedSimplexUpdatedBasisElement.tsx";
import {
    renderRevisedSimplexNonBasicVariablesCurrentReducedCostsCalculation,
    renderRevisedSimplexRhsCalculation,
    renderRevisedSimplexYTCalculation
} from "../revised/RevisedSimplexIterationElement.tsx";
import MultiplicativeSimplexElementaryMatrixInverseCalculationElement
    from "./MultiplicativeSimplexElementaryMatrixInverseCalculationElement.tsx";
import MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElement
    from "./MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElement.tsx";

type MultiplicativeSimplexIterationDtoElementProps = {
    multiplicativeSimplexIterationDto: MultiplicativeSimplexIterationDto
    iterationIndex: number
}

/**
 * Element containing whole multiplicative simplex iteration
 * Renders MultiplicativeSimplexIterationDto
 * @param props
 * @constructor
 */
function MultiplicativeSimplexIterationDtoElement(props: MultiplicativeSimplexIterationDtoElementProps) {
    return (<>
        <hr/>
        <h3 className={"pt-2"}>Start iteration {props.iterationIndex + 1}</h3>
        <p className={"pt-2"}>Current basis:</p>
        <BlockMath math={renderTextVector(props.multiplicativeSimplexIterationDto.currentBasis)} />
        <p className={"pt-2"}>Basis matrix and its inverse:</p>
        <BlockMath math={renderMatrixWithName("B", props.multiplicativeSimplexIterationDto.initialBasisMatrix)} />
        <BlockMath math={renderMatrixWithName("B^{-1}", props.multiplicativeSimplexIterationDto.initialBasisMatrixInverse)} />
        <p className={"pt-2"}>Calculate RHS:</p>
        <BlockMath math={renderRevisedSimplexRhsCalculation(props.multiplicativeSimplexIterationDto.initialBasisMatrixInverse, props.multiplicativeSimplexIterationDto.b, props.multiplicativeSimplexIterationDto.XB)} />
        <p className={"pt-2"}>Get reduced cost of current basis variables in the original simplex table:</p>
        <BlockMath math={renderMatrixWithName("c_{B}^{T}", props.multiplicativeSimplexIterationDto.originalSimplexTableReducedCosts)} />
        <p className={"pt-2"}>Calculate y vector for non basis variables reduced costs calculation:</p>
        <BlockMath math={renderRevisedSimplexYTCalculation(props.multiplicativeSimplexIterationDto.originalSimplexTableReducedCosts, props.multiplicativeSimplexIterationDto.initialBasisMatrixInverse, props.multiplicativeSimplexIterationDto.YT)} />
        <p className={"pt-2"}>Calculate reduced cost for each non basic variable:</p>
        <BlockMath math={renderRevisedSimplexNonBasicVariablesCurrentReducedCostsCalculation(props.multiplicativeSimplexIterationDto.nonBasicVariablesCurrentReducedCosts, props.multiplicativeSimplexIterationDto.YT)} />
        <RevisedSimplexEnteringVariableElement iterationDto={props.multiplicativeSimplexIterationDto} />
        <RevisedSimplexDirectionVectorCalculationElement iterationDto={props.multiplicativeSimplexIterationDto} />
        <RevisedSimplexRatioVectorCalculationElement iterationDto={props.multiplicativeSimplexIterationDto} />
        <RevisedSimplexLeavingVariableElement iterationDto={props.multiplicativeSimplexIterationDto} />
        <RevisedSimplexUpdatedBasisElement iterationDto={props.multiplicativeSimplexIterationDto} />
        <MultiplicativeSimplexElementaryMatrixInverseCalculationElement multiplicativeSimplexIterationDto={props.multiplicativeSimplexIterationDto} />
        <MultiplicativeSimplexNextInterationInverseBasisMatrixCalculationElement multiplicativeSimplexIterationDto={props.multiplicativeSimplexIterationDto} />
    </>)
}

export default MultiplicativeSimplexIterationDtoElement;