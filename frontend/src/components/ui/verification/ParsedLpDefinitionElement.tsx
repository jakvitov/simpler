import {BlockMath} from 'react-katex';
import type {Bound, LpDefinitionLine, ParsedLpDefinition} from "../../../api/common/lpDefinitionTypes.ts";
import {renderRationalWithNegativeSignOnly, renderRationalWithSign} from "../../../api/common/math.ts";

type ParsedLpDefinitionElementProps = {
    parsedLpDefinition: ParsedLpDefinition
}

function renderParsedLpDefinitionBound(bound: Bound, index: number) {
    let res = "\\tag{" + (index + 1) + "}";
    if (bound.lowerbound === null || bound.lowerbound === undefined) {
        res += "0 \\leq "+  bound.variableName
    } else {
        res += renderRationalWithNegativeSignOnly(bound.lowerbound) + " \\leq "  + bound.variableName
    }
    if (bound.upperbound !== null && res.length > 0) {
        res += " \\leq " + renderRationalWithNegativeSignOnly(bound.upperbound)
    } else if (bound.upperbound !== null) {
        res += bound.variableName + " \\leq " + renderRationalWithNegativeSignOnly(bound.upperbound)
    }
    return res;
}

function renderParsedLpDefinitionLine(lpDefinitionLine: LpDefinitionLine, i: number) {
    let res = "\\tag{" + (i+1) + "}"

    //Render first variable with rational coefficient without + sign
    if (lpDefinitionLine.variableValues.length !== 0) {
        res += renderRationalWithNegativeSignOnly(lpDefinitionLine.variableValues[0].value) + lpDefinitionLine.variableValues[0].variableName
    }

    //Render the rest with any sign
    for (let i = 1; i < lpDefinitionLine.variableValues.length; i++) {
        res += renderRationalWithSign(lpDefinitionLine.variableValues[i].value) + lpDefinitionLine.variableValues[i].variableName
    }

    if (lpDefinitionLine.inequalitySign === "GE") {
        res += "\\geq"
        res += renderRationalWithSign(lpDefinitionLine.rhs)
    } else if (lpDefinitionLine.inequalitySign === "LE") {
        res += "\\geq"
        res += renderRationalWithSign(lpDefinitionLine.rhs)
    } else if (lpDefinitionLine.inequalitySign === "EQ") {
        res += "="
        res += renderRationalWithSign(lpDefinitionLine.rhs)
    } else if (lpDefinitionLine.inequalitySign === "N") {
        res += "\\rightarrow OBJECTIVE"
    }
    res += ""
    return res
}

function ParsedLpDefinitionElement(props: ParsedLpDefinitionElementProps){

    if (props.parsedLpDefinition.warningMessage !== null && props.parsedLpDefinition.warningMessage !== undefined) {
        return (<>
            <h3 className={"pt-2"}>Parsed linear problem:</h3>
            <h4 className={"pt-2"}>Warning:</h4>
            <pre>{props.parsedLpDefinition.warningMessage}</pre>
            <BlockMath key={"1"} math={"\\text{Equations:}"}></BlockMath>
            {props.parsedLpDefinition.lines.map((line: LpDefinitionLine, i: number) => <BlockMath key={"l" + i} math={renderParsedLpDefinitionLine(line, i)} />)}
            <BlockMath key={"2"} math={"\\text{Bounds:}"}></BlockMath>
            {props.parsedLpDefinition.bounds.map((bound: Bound, i: number) => <BlockMath key={"b" + i} math={renderParsedLpDefinitionBound(bound, i)}/>)}
            </>
            )
    }
    return (
            <>
                <h3 className={"pt-2"}>Parsed linear problem:</h3>
                <BlockMath key={"1"} math={"\\text{Equations:}"}></BlockMath>
                {props.parsedLpDefinition.lines.map((line: LpDefinitionLine, i: number) => <BlockMath key={"l" + i} math={renderParsedLpDefinitionLine(line, i)} />)}
                <BlockMath key={"2"} math={"\\text{Bounds:}"}></BlockMath>
                {props.parsedLpDefinition.bounds.map((bound: Bound, i: number) => <BlockMath key={"b" + i} math={renderParsedLpDefinitionBound(bound, i)}/>)}
            </>
        )
}

export default ParsedLpDefinitionElement;