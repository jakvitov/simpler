import {BlockMath} from 'react-katex';
import type {Bound, LpDefinitionLine, ParsedLpDefinition} from "../../../api/common/lpDefinitionTypes.ts";
import {renderRationalWithNegativeSignOnly, renderRationalWithSign} from "../../../api/common/math.ts";

type ParsedLpDefinitionProps = {
    parsedProblem: ParsedLpDefinition|null
}

function renderParsedLpDefinitionBound(bound: Bound, index: number) {
    let res = "\\tag{" + (index + 1) + "}";
    if (bound.lowerbound === null) {
        res += "0 \\leq "+  bound.variableName
    } else {
        res += renderRationalWithNegativeSignOnly(bound.lowerbound) + " \\leq "  + bound.variableName
    }
    if (bound.upperbound !== null && res.length > 0) {
        res += " \\leq " + renderRationalWithNegativeSignOnly(bound.upperbound)
    } else {
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

    res += (lpDefinitionLine.inequalitySign === "GE" ? "\\geq" : "\\leq")
    res += renderRationalWithSign(lpDefinitionLine.rhs)
    res += ""
    return res
}

function LpDefinition(props: ParsedLpDefinitionProps){
    console.log("Loaded props: "  + props)

    if (props.parsedProblem !== null) {
        return (
            <>
                <h3 className={"pt-2"}>Parsed linear problem:</h3>
                <BlockMath math={"\\text{Equations:}"}></BlockMath>
                {props.parsedProblem.lines.map((line: LpDefinitionLine, i: number) => <BlockMath math={renderParsedLpDefinitionLine(line, i)} />)}
                <BlockMath math={"\\text{Bounds:}"}></BlockMath>
                {props.parsedProblem.bounds.map((bound: Bound, i: number) => <BlockMath math={renderParsedLpDefinitionBound(bound, i)}/>)}
            </>
        )
    }

    return (
        <>
            <BlockMath math="x_1 \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}" />
            <BlockMath math="\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}" />
        </>
    )
}

export default LpDefinition;