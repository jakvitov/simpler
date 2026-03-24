import type {TwoPhaseSimplexPhaseOneSolutionDto} from "../../../../api/solver/two-phase/twoPhaseSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../basic/PlainSimplexTableElement.tsx";
import TwoPhaseSimplexObjectiveRowNormalizationElement from "./TwoPhaseSimplexObjectiveRowNormalizationElement.tsx";
import BasicSimplexIterationElement from "../basic/BasicSimplexIterationElement.tsx";

type TwoPhaseSimplexPhaseOneSolutionElementProps = {
    twoPhaseSimplexPhaseOneSolutionDto: TwoPhaseSimplexPhaseOneSolutionDto
}


function TwoPhaseSimplexPhaseOneSolutionElement(props: TwoPhaseSimplexPhaseOneSolutionElementProps) {
    return (<>
        <h3 className={"pt-2"}>Start phase I</h3>
        <p className={"pt-2"}>Initial simplex table:</p>
        <PlainSimplexTableElement simplexTable={props.twoPhaseSimplexPhaseOneSolutionDto.initialSimplexTable} />
        <p className={"pt-2"}>Adjusting artificial variables in objective row to base:</p>
        <TwoPhaseSimplexObjectiveRowNormalizationElement twoPhaseSimplexObjectiveRowNormalizationDto={props.twoPhaseSimplexPhaseOneSolutionDto.artificialVariablesNormalization} />
        {(props.twoPhaseSimplexPhaseOneSolutionDto.iterations != null) ? props.twoPhaseSimplexPhaseOneSolutionDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
        <p className={"pt-2"}>Final simplex table after phase I:</p>
        <PlainSimplexTableElement simplexTable={props.twoPhaseSimplexPhaseOneSolutionDto.finalSimplexTable} />
    </>)
}

export default TwoPhaseSimplexPhaseOneSolutionElement;