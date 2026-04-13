import type {RevisedSimplexPhaseTwoSolutionDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import RevisedSimplexIterationElement from "./RevisedSimplexIterationElement.tsx";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";
import DividerWithText from "../../general/DividerWithText.tsx";

type RevisedSimplexPhaseTwoSolutionElementProps = {
    revisedSimplexPhaseTwoSolutionDto: RevisedSimplexPhaseTwoSolutionDto|undefined
}

/**
 * Element responsible for rendering phase two solution for revised simplex
 * Renders RevisedSimplexPhaseTwoSolutionDto
 * @param props
 * @constructor
 */
function RevisedSimplexPhaseTwoSolutionElement(props: RevisedSimplexPhaseTwoSolutionElementProps) {
    if (props.revisedSimplexPhaseTwoSolutionDto == null) {
        return (<DividerWithText text={"Phase II skipped"}/>)
    }

    return (<>
        <DividerWithText text={"Start phase II"}/>
        <p className={"pt-2"}>Initial feasible base:</p>
        <BlockMath math={renderTextVector(props.revisedSimplexPhaseTwoSolutionDto.initialFeasibleBase)} />
        {(props.revisedSimplexPhaseTwoSolutionDto.iterations != null) ? props.revisedSimplexPhaseTwoSolutionDto.iterations.map((iterationDto, index) => <RevisedSimplexIterationElement revisedSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
        <DividerWithText text={"End phase II"}/>

    </>)
}

export default RevisedSimplexPhaseTwoSolutionElement;