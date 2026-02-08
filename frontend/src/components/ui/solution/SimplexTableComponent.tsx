import type {Rational} from "../../../api/common/math.ts";
import {demoMatrix, renderRationalWithNegativeSignOnly} from "../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableProps = {
    variables: string[],
    data: Rational[][],
    rhs: Rational[],
    demo: boolean
}

// render line like var1 & var2 \\ for simplex table matrix
function renderVariableNamesRow(variables: string[]): string {
    if (variables.length === 0) {
        return "";
    }
    let res = ""
    res += `\\small{${variables[0]}}`

    for (let i = 1; i < variables.length; i++) {
        res += `& ${variables[i]} `
    }

    res += "& RHS"
    res += "\\\\[10pt]"
    return res
}

// render line like 3/2 & -1/2 \\ for simplex table matrix
function renderValuesRow(valuesRow: Rational[], rhs: Rational): string {
    if (valuesRow.length === 0) {
        return ""
    }
    let res = ""
    res += renderRationalWithNegativeSignOnly(valuesRow[0])

    for (let i = 1; i < valuesRow.length; i++) {
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
function getKatexArrayType(props: SimplexTableProps): string {
    let res = "{"
    for (let i = 0; i < props.variables.length; i++) {
        res += "c"
    }
    //For rhs
    res += "|c}"
    return res
}

function renderSimplexTable(props: SimplexTableProps): string {
    let res = "\\begin{pmatrix}"
    res += "\\begin{array}" + getKatexArrayType(props)
    res += renderVariableNamesRow(props.variables)
    props.data.forEach((value, i) => {
        res += renderValuesRow(value, props.rhs[i])
    })
    res += "\\end{array}"
    res += "\\end{pmatrix}"
    return res
}

function SimplexTableComponent(props: SimplexTableProps) {
    if (props.demo) {
        props = {
            variables: ["X1", "X2", "X3"],
            data: demoMatrix(3,3),
            rhs: demoMatrix(1,3)[0],
            demo: true
        }
    }

    return <BlockMath math={renderSimplexTable(props)}></BlockMath>
}

export default SimplexTableComponent