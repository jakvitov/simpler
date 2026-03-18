import type {BasicSimplexIterationDto} from "../../../../api/solver/basic/basicSimplexSolveTypes.ts";
import SimplexTableLeavingEnteringVariableElement
    from "./SimplexTableLeavingEnteringVariableElement.tsx";
import SimplexTableLeavingRowNormalisationElement
    from "./SimplexTableLeavingRowNormalisationElement.tsx";
import SimplexTableRowsNormalizationElement from "./SimplexTableRowsNormalizationElement.tsx";
import PlainSimplexTableElement from "./PlainSimplexTableElement.tsx";

type BasicSimplexIterationElementProps = {
    basicSimplexIterationDto: BasicSimplexIterationDto;
    iterationIndex: number;
}

/**
 * Element containing one basic simplex iteration
 * Renders BasicSimplexIterationDto
 * @param props
 * @constructor
 */
function BasicSimplexIterationElement(props: BasicSimplexIterationElementProps) {
    return (<div className="basicSimplexIterationElement">
        <hr/>
        <h3 className={"pt-2"}>Start iteration {props.iterationIndex}</h3>
        <h3 className={"pt-2"}>Leaving and entering variable calculation:</h3>
        <SimplexTableLeavingEnteringVariableElement simplexTableLeavingEnteringVariableDto={props.basicSimplexIterationDto.simplexTableLeavingEnteringVariableDto} />
        <h3 className={"pt-2"}>Normalization of leaving variable row:</h3>
        <SimplexTableLeavingRowNormalisationElement simplexTableLeavingEnteringVariableDto={props.basicSimplexIterationDto.simplexTableLeavingRowNormalisationDto} />
        <h3 className={"pt-2"}>Normalization of remaining rows:</h3>
        <SimplexTableRowsNormalizationElement simplexTableRowNormalizationElementDto={props.basicSimplexIterationDto.simplexTableRowsNormalizationDto} />
        <h3 className={"pt-2"}>Final simplex after variable switch:</h3>
        <PlainSimplexTableElement simplexTable={props.basicSimplexIterationDto.simplexTableAfterVariableSwitch} />
        <h3 className={"pt-2"}>End iteration {props.iterationIndex}</h3>
        <hr/>
    </div>)
}

export default BasicSimplexIterationElement;