import {demoRational, type Rational} from "../../../api/common/math.ts";
import {demoMatrix, renderRationalWithNegativeSignOnly} from "../../../api/common/math.ts";
import {BlockMath} from "react-katex";
import type {SimplexTable} from "../../../api/common/lpDefinitionTypes.ts";

type SimplexTableProps = {
    simplexTable: SimplexTable;
    demo: boolean
}

// render line like var1 & var2 \\ for simplex table matrix
export function renderVariableNamesRow(variables: string[]): string {
    if (variables.length === 0) {
        return "";
    }
    let res = ""

    for (let i = 0; i < variables.length; i++) {
        res += `& ${variables[i]} `
    }

    res += "& RHS"
    res += "\\\\[10pt]"
    return res
}

// render line like 3/2 & -1/2 \\ for simplex table matrix
export function renderValuesRow(valuesRow: Rational[], rhs: Rational, baseVariable: string): string {
    if (valuesRow.length === 0) {
        return ""
    }

    let res: string = `${baseVariable}`;

    for (let i = 0; i < valuesRow.length; i++) {
        res += `& ${renderRationalWithNegativeSignOnly(valuesRow[i])} `
    }
    res += `& ${renderRationalWithNegativeSignOnly(rhs)} `
    res += "\\\\[15pt]"
    return res
}

/**
 * Based on the props sized, return properties for katex array inside of matrix
 * example {cccc|c}, rhs is rendered in the matrix block |c
 */
export function getKatexArrayType(props: SimplexTableProps): string {
    let res = "{c|"
    for (let i = 0; i < props.simplexTable.variables.length; i++) {
        res += "c"
    }
    //For rhs
    res += "|c}"
    return res
}

function renderSimplexTable(props: SimplexTableProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getKatexArrayType(props) + "\n"
    res += renderVariableNamesRow(props.simplexTable.variables)
    props.simplexTable.data.forEach((value, i) => {
            res += renderValuesRow(value, props.simplexTable.rhs[i], props.simplexTable.baseVariables[i]) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderValuesRow(props.simplexTable.objectiveFunctionRow, props.simplexTable.objectiveValue, "z") + "\n"
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res
}

function SimplexTableComponent(props: SimplexTableProps) {
    if (props.demo) {
        props = {
            simplexTable: {
                variables: ["X1", "X2", "X3"],
                data: demoMatrix(2, 3),
                rhs: demoMatrix(1, 3)[0],
                baseVariables: ["X1", "X2"],
                objectiveFunctionRow: demoMatrix(1, 3)[0],
                objectiveValue: demoRational(),
            },
            demo: true
        }
    }
    return <BlockMath math={renderSimplexTable(props)}></BlockMath>
}

export default SimplexTableComponent