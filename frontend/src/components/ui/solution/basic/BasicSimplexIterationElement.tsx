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
    //Singals unbounded solution
    if (props.basicSimplexIterationDto.simplexTableLeavingRowNormalisationDto == null) {
        return (<div className="basicSimplexIterationElement">
            <hr/>
            <h3 className={"pt-2"}>Start iteration {props.iterationIndex + 1}</h3>
            <p className={"pt-2"}>Leaving and entering variable calculation:</p>
            <SimplexTableLeavingEnteringVariableElement simplexTableLeavingEnteringVariableDto={props.basicSimplexIterationDto.simplexTableLeavingEnteringVariableDto} />
            <hr/>
        </div>)
    }

    //Standard iteration
    return (<div className="basicSimplexIterationElement">
        <hr/>
        <h3 className={"pt-2"}>Start iteration {props.iterationIndex + 1}</h3>
        <p className={"pt-2"}>Leaving and entering variable calculation:</p>
        <SimplexTableLeavingEnteringVariableElement simplexTableLeavingEnteringVariableDto={props.basicSimplexIterationDto.simplexTableLeavingEnteringVariableDto} />
        <p className={"pt-2"}>Normalization of leaving variable row:</p>
        <SimplexTableLeavingRowNormalisationElement simplexTableLeavingEnteringVariableDto={props.basicSimplexIterationDto.simplexTableLeavingRowNormalisationDto} />
        <p className={"pt-2"}>Normalization of remaining rows:</p>
        <SimplexTableRowsNormalizationElement simplexTableRowNormalizationElementDto={props.basicSimplexIterationDto.simplexTableRowsNormalizationDto} />
        <p className={"pt-2"}>Final simplex after variable switch:</p>
        <PlainSimplexTableElement simplexTable={props.basicSimplexIterationDto.simplexTableAfterVariableSwitch} />
        <p className={"pt-2"}>End iteration {props.iterationIndex}</p>
        <hr/>
    </div>)
}

export default BasicSimplexIterationElement;