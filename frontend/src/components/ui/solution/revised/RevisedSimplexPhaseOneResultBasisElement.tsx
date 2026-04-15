import {BlockMath} from "react-katex";
import {renderTextVector} from "../../../../api/common/math.ts";

type RevisedSimplexPhaseOneResultBasisElementProps = {
    resultBasis: string[]|undefined
}

/**
 * Renders result base of phase I in revised simplex
 * @param props
 * @constructor
 */
function RevisedSimplexPhaseOneResultBasisElement(props: RevisedSimplexPhaseOneResultBasisElementProps) {
    if (props.resultBasis == null) {
        return (<></>)
    }

    return (<>
        <p className={"pt-2"}>Result base after iteration:</p>
        <BlockMath math={renderTextVector(props.resultBasis)} />
    </>)
}

export default RevisedSimplexPhaseOneResultBasisElement