import type {
    MultiplicativeSimplexPhaseTwoSolutionDto
} from "../../../../api/solver/multiplicative/multiplicativeSimplexSolveTypes.ts";
import DividerWithText from "../../general/DividerWithText.tsx";
import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";
import MultiplicativeSimplexIterationDtoElement from "./MultiplicativeSimplexIterationDtoElement.tsx";

type MultiplicativeSimplexPhaseOneResultBasisElementProps = {
    multiplicativeSimplexPhaseTwoSolutionDto: MultiplicativeSimplexPhaseTwoSolutionDto|undefined;
}

function MultiplicativeSimplexPhaseTwoSolutionElement(props: MultiplicativeSimplexPhaseOneResultBasisElementProps) {
    if (props.multiplicativeSimplexPhaseTwoSolutionDto == null) {
        return (<DividerWithText text={"Phase II skipped"}/>)
    }

    return (<>
        <DividerWithText text={"Start phase II"}/>
        <p className={"pt-2"}>Initial feasible base:</p>
        <BlockMath math={renderTextVector(props.multiplicativeSimplexPhaseTwoSolutionDto.initialFeasibleBase)} />
        {(props.multiplicativeSimplexPhaseTwoSolutionDto.iterations != null) ? props.multiplicativeSimplexPhaseTwoSolutionDto.iterations.map((iterationDto, index) => <MultiplicativeSimplexIterationDtoElement multiplicativeSimplexIterationDto={iterationDto} iterationIndex={index} />) : <></>}
        <DividerWithText text={"End phase II"}/>

    </>)
}

export default MultiplicativeSimplexPhaseTwoSolutionElement;