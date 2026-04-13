import type {RevisedSimplexPhaseOneSolutionDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../basic/PlainSimplexTableElement.tsx";
import TwoPhaseSimplexObjectiveRowNormalizationElement
    from "../two-phase/TwoPhaseSimplexObjectiveRowNormalizationElement.tsx";
import RevisedSimplexIterationElement from "./RevisedSimplexIterationElement.tsx";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";
import DividerWithText from "../../general/DividerWithText.tsx";

type RevisedSimplexPhaseOneSolutionElementProps = {
    revisedSimplexPhaseOneSolutionDto: RevisedSimplexPhaseOneSolutionDto|undefined;
}

type RevisedSimplexPhaseOneResultBasisElementProps = {
    resultBasis: string[]|undefined
}

function RevisedSimplexPhaseOneResultBasisElement(props: RevisedSimplexPhaseOneResultBasisElementProps) {
    if (props.resultBasis == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Result base after iteration:</p>
        <BlockMath math={renderTextVector(props.resultBasis)} />
    </>)
}

/**
 * Element responsible for rendering phase one solution for revised simplex
 * Renders RevisedSimplexPhaseOneSolutionDto
 * @param props
 * @constructor
 */
function RevisedSimplexPhaseOneSolutionElement(props: RevisedSimplexPhaseOneSolutionElementProps) {
    if (props.revisedSimplexPhaseOneSolutionDto == null) {
        return (<DividerWithText text={"Phase I skipped"}/>)
    }

    return (<>
        <DividerWithText text={"Start phase I"}/>
        <p className={"pt-2"}>Initial simplex table for phase I:</p>
        <PlainSimplexTableElement simplexTable={props.revisedSimplexPhaseOneSolutionDto.initialSimplexTable}/>
        <p className={"pt-2"}>Adjusting artificial variables in objective row to base:</p>
        <TwoPhaseSimplexObjectiveRowNormalizationElement
            twoPhaseSimplexObjectiveRowNormalizationDto={props.revisedSimplexPhaseOneSolutionDto.artificialVariablesNormalization}/>
        {(props.revisedSimplexPhaseOneSolutionDto.iterations != null) ? props.revisedSimplexPhaseOneSolutionDto.iterations.map((iterationDto, index) =>
            <RevisedSimplexIterationElement revisedSimplexIterationDto={iterationDto} iterationIndex={index}/>) : <></>}
        <RevisedSimplexPhaseOneResultBasisElement resultBasis={props.revisedSimplexPhaseOneSolutionDto.resultBase}/>
        <DividerWithText text={"End phase I"}/>

    </>)

}

export default RevisedSimplexPhaseOneSolutionElement;