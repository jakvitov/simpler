import type {TwoPhaseSimplexPhaseTwoSolutionDto} from "../../../../api/solver/two-phase/twoPhaseSimplexSolveTypes.ts";
import PlainSimplexTableElement from "../basic/PlainSimplexTableElement.tsx";
import TwoPhaseSimplexObjectiveRowNormalizationElement from "./TwoPhaseSimplexObjectiveRowNormalizationElement.tsx";
import BasicSimplexIterationElement from "../basic/BasicSimplexIterationElement.tsx";
import DividerWithText from "../../general/DividerWithText.tsx";

type TwoPhaseSimplexPhaseTwoSolutionElementProps = {
    twoPhaseSimplexPhaseTwoSolutionDto: TwoPhaseSimplexPhaseTwoSolutionDto|undefined
}

function TwoPhaseSimplexPhaseTwoSolutionElement(props: TwoPhaseSimplexPhaseTwoSolutionElementProps) {
    if (props.twoPhaseSimplexPhaseTwoSolutionDto == null) {
        return (<DividerWithText text={"Phase II skipped"}/>)
    }
    return (<>
        <DividerWithText text={"Start phase II"}/>
        <p className={"pt-2"}>Initial simplex table:</p>
        <PlainSimplexTableElement simplexTable={props.twoPhaseSimplexPhaseTwoSolutionDto.initialSimplexTable} />
        <p className={"pt-2"}>Simplex table with restored original objective row:</p>
        <PlainSimplexTableElement simplexTable={props.twoPhaseSimplexPhaseTwoSolutionDto.simplexTableWithRestoredObjectiveRow} />
        <p className={"pt-2"}>Adjusting objective row to base variables:</p>
        <TwoPhaseSimplexObjectiveRowNormalizationElement twoPhaseSimplexObjectiveRowNormalizationDto={props.twoPhaseSimplexPhaseTwoSolutionDto.objectiveRowToBaseVariablesAdjustment} />
        {(props.twoPhaseSimplexPhaseTwoSolutionDto.iterations != null) ? props.twoPhaseSimplexPhaseTwoSolutionDto.iterations.map((iterationDto, index) => <BasicSimplexIterationElement basicSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
        <p className={"pt-2"}>Final simplex table after phase II:</p>
        <PlainSimplexTableElement simplexTable={props.twoPhaseSimplexPhaseTwoSolutionDto.finalSimplexTable} />
        <DividerWithText text={"End phase II"}/>
    </>)
}

export default TwoPhaseSimplexPhaseTwoSolutionElement;