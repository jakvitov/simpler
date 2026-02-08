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
    res += "\\\\[10pt]"
    return res
}

// render line like 3/2 & -1/2 \\ for simplex table matrix
function renderValuesRow(valuesRow: Rational[]): string {
    if (valuesRow.length === 0) {
        return ""
    }
    let res = ""
    res += renderRationalWithNegativeSignOnly(valuesRow[0])

    for (let i = 1; i < valuesRow.length; i++) {
        res += `& ${renderRationalWithNegativeSignOnly(valuesRow[i])} `
    }
    res += "\\\\[15pt]"
    return res
}

function renderSimplexTable(props: SimplexTableProps): string {
    console.log(props.data)
    let res = "\\begin{pmatrix}"
    res += renderVariableNamesRow(props.variables)
    props.data.forEach((value) => {
        res += renderValuesRow(value)
    })
    res += "\\end{pmatrix}"
    return res
}

function SimplexTable(props: SimplexTableProps) {
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

export default SimplexTable