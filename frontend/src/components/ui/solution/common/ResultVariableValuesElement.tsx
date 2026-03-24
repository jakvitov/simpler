import {type Rational, renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type ResultVariableValuesElementProps = {
    resultVariableValues: Record<string, Rational>|undefined
}

function getResultVariableValuesArrayType(resultVariableValues: Map<string, Rational>): string {
    let res = "{c"
    for (let i = 1; i < resultVariableValues.size; i++) {
        res += ":c"
    }
    res += "}"
    return res;
}

function renderResultVariableValuesElement(resultVariableValues: Map<string, Rational>): string {
    if (resultVariableValues.size == 0) {
        return ""
    }

    let res = "\\def\\arraystretch{2}";
    res += ("\\begin{array}" + getResultVariableValuesArrayType(resultVariableValues) + "\n")
    let variableNames: string[] = []
    resultVariableValues.forEach((_value, key) => variableNames.push(key) )

    res += variableNames[0];

    for (let i = 1; i < resultVariableValues.size; i++) {
        res += `& ${variableNames[i]}`;
    }
    res += "\\\\ \\hline \n";
    res += renderRationalWithNegativeSignOnly(resultVariableValues.get(variableNames[0]));

    for (let i = 1; i < resultVariableValues.size; i++) {
        res += `& ${renderRationalWithNegativeSignOnly(resultVariableValues.get(variableNames[i]))}`;
    }
    res += "\\\\ \n \\end{array}"
    return res;
}

/**
 * Element with table containing result variable values from successfully solved LP
 * Renders resultVariableValues
 * @param props
 * @constructor
 */
function ResultVariableValuesElement(props: ResultVariableValuesElementProps) {
    if (props.resultVariableValues != null) {
        const resultVariableValuesMap = new Map<string, Rational>(
            Object.entries(props.resultVariableValues).map(([k, v]) => [(k), v])
        );
        return <BlockMath math={renderResultVariableValuesElement(resultVariableValuesMap)} />
    }
    return <BlockMath math={"EMPTY"}></BlockMath>
}

export default ResultVariableValuesElement