import type {
    MultiplicativeSimplexPhaseOneSolutionDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import DividerWithText from "../../general/DividerWithText.tsx";
import PlainSimplexTableElement from "../basic/PlainSimplexTableElement.tsx";
import TwoPhaseSimplexObjectiveRowNormalizationElement
    from "../two-phase/TwoPhaseSimplexObjectiveRowNormalizationElement.tsx";
import MultiplicativeSimplexIterationDtoElement from "./MultiplicativeSimplexIterationDtoElement.tsx";
import RevisedSimplexPhaseOneResultBasisElement from "../revised/RevisedSimplexPhaseOneResultBasisElement.tsx";

type MultiplicativeSimplexPhaseOneSolutionElementProps = {
    multiplicativeSimplexPhaseOneSolutionDto: MultiplicativeSimplexPhaseOneSolutionDto|undefined
}


function MultiplicativeSimplexPhaseOneSolutionElement(props: MultiplicativeSimplexPhaseOneSolutionElementProps) {
    if (props.multiplicativeSimplexPhaseOneSolutionDto == null) {
        return (<DividerWithText text={"Phase I skipped"}/>)
    }

    return (<>
        <DividerWithText text={"Start phase I"}/>
        <p className={"pt-2"}>Initial simplex table for phase I:</p>
        <PlainSimplexTableElement simplexTable={props.multiplicativeSimplexPhaseOneSolutionDto.initialSimplexTable}/>
        <p className={"pt-2"}>Adjusting artificial variables in objective row to base:</p>
        <TwoPhaseSimplexObjectiveRowNormalizationElement
            twoPhaseSimplexObjectiveRowNormalizationDto={props.multiplicativeSimplexPhaseOneSolutionDto.artificialVariablesNormalization}/>
        {(props.multiplicativeSimplexPhaseOneSolutionDto.iterations != null) ? props.multiplicativeSimplexPhaseOneSolutionDto.iterations.map((iterationDto, index) =>
            <MultiplicativeSimplexIterationDtoElement multiplicativeSimplexIterationDto={iterationDto} iterationIndex={index}/>) : <></>}
        <RevisedSimplexPhaseOneResultBasisElement resultBasis={props.multiplicativeSimplexPhaseOneSolutionDto.resultBase}/>
        <DividerWithText text={"End phase I"}/>

    </>)
}

export default MultiplicativeSimplexPhaseOneSolutionElement;