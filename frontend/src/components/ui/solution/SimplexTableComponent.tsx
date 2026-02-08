import type {Rational} from "../../../api/common/math.ts";
import {demoMatrix, renderRationalWithNegativeSignOnly} from "../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableProps = {
    variables: string[],
    base_variables: string[]
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

    for (let i = 0; i < variables.length; i++) {
        res += `& ${variables[i]} `
    }

    res += "& RHS"
    res += "\\\\[10pt]"
    return res
}

// render line like 3/2 & -1/2 \\ for simplex table matrix
// null base variable means that the row is last and should be evaluated like z function
function renderValuesRow(valuesRow: Rational[], rhs: Rational, baseVariable: string|null): string {
    if (valuesRow.length === 0) {
        return ""
    }

    let res: string = "";
    if (baseVariable === null) {
        res += "z"
    } else {
        res += `${baseVariable}`
    }

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
function getKatexArrayType(props: SimplexTableProps): string {
    let res = "{c|"
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
        if (i === props.base_variables.length) {
            res += renderValuesRow(value, props.rhs[i], null)
        } else {
            res += renderValuesRow(value, props.rhs[i], props.base_variables[i])
        }
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
            base_variables: ["X1", "X2"],
            demo: true
        }
    }

    return <BlockMath math={renderSimplexTable(props)}></BlockMath>
}

export default SimplexTableComponent