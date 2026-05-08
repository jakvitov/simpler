import {type Rational, renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";
import type {ResultVariableValues} from "../../../../api/solver/solveLpTypes.ts";

type ResultVariableValuesElementProps = {
    resultVariableValues: ResultVariableValues|undefined
}

type VariableType =
    | "PROBLEM"
    | "SLACK_SURPLUS"

function getResultVariableValuesArrayType(size: number): string {
    let res = "{c"
    for (let i = 1; i < size; i++) {
        res += ":c"
    }
    res += "}"
    return res;
}

function renderResultVariableValuesElement(resultVariableValues: Map<string, Rational>, variablesType: VariableType): string {
    const sortedVariableNames: string[] = [...resultVariableValues.keys()].sort();
    if (resultVariableValues.size == 0) {
        return ""
    }

    let res = "";

    switch (variablesType) {
        case "PROBLEM": res += "\\text{Problem variables:} \\ \\ \\ "; break;
        case "SLACK_SURPLUS": res += "\\text{Slack/surplus variables:} \\ \\ \\ "; break;
    }

    res += "\\def\\arraystretch{2}";
    res += ("\\begin{array}" + getResultVariableValuesArrayType(sortedVariableNames.length) + "\n")
    res += sortedVariableNames[0];

    for (let i = 1; i < resultVariableValues.size; i++) {
        res += `& ${sortedVariableNames[i]}`;
    }
    res += "\\\\ \\hline \n";
    res += renderRationalWithNegativeSignOnly(resultVariableValues.get(sortedVariableNames[0]));

    for (let i = 1; i < resultVariableValues.size; i++) {
        res += `& ${renderRationalWithNegativeSignOnly(resultVariableValues.get(sortedVariableNames[i]))}`;
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
        const problemVariables = new Map<string, Rational>(
            Object.entries(props.resultVariableValues.problemVariables).map(([k, v]) => [(k), v])
        );
        const slackSurplusVariables = new Map<string, Rational>(
            Object.entries(props.resultVariableValues.slackSurplusVariables).map(([k, v]) => [(k), v])
        );
        return (
            <>
            <BlockMath math={renderResultVariableValuesElement(problemVariables, "PROBLEM")} />
            <BlockMath math={renderResultVariableValuesElement(slackSurplusVariables, "SLACK_SURPLUS")} />
            </>
        )
    }
    return <BlockMath math={"EMPTY"}></BlockMath>
}

export default ResultVariableValuesElement