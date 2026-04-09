import type {RevisedSimplexPhaseTwoSolutionDto} from "../../../../api/solver/revised/revisedSimplexSolveTypes.ts";
import RevisedSimplexIterationElement from "./RevisedSimplexIterationElement.tsx";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";

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
        return (<h3 className={"pt-2"}>Phase II skipped.</h3>)
    }

    return (<>
        <h3 className={"pt-2"}>Start phase II</h3>
        <p className={"pt-2"}>Initial feasible base:</p>
        <BlockMath math={renderTextVector(props.revisedSimplexPhaseTwoSolutionDto.initialFeasibleBase)} />
        {(props.revisedSimplexPhaseTwoSolutionDto.iterations != null) ? props.revisedSimplexPhaseTwoSolutionDto.iterations.map((iterationDto, index) => <RevisedSimplexIterationElement revisedSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
        <h3 className={"pt-2"}>End phase II</h3>
    </>)
}

export default RevisedSimplexPhaseTwoSolutionElement;