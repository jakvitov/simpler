import type {SimplexTable} from "../../../api/common/lpDefinitionTypes.ts";
import {type Rational, renderRationalWithNegativeSignOnly} from "../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableRowAddititionArrowsProps = {
    simplexTable: SimplexTable,
    sourceRowIndex: number,
    //Keys are row indexes and rational is coefficient of how much source is added there
    targetRows: Map<number, Rational>
}

function getKatexArrayTypeForTableRowAddition(props: SimplexTableRowAddititionArrowsProps): string {
    let res = "{c|"
    for (let i = 0; i < props.simplexTable.variables.length; i++) {
        res += "c"
    }
    //For rhs, arrow and coefficient
    res += "|ccc}"
    return res
}

function renderVariableNamesRow(variables: string[]): string {
    if (variables.length === 0) {
        return "";
    }
    let res = ""

    for (let i = 0; i < variables.length; i++) {
        res += `& ${variables[i]} `
    }

    res += "& RHS & "
    res += "\\\\[10pt]"
    return res
}

// render line like 3/2 & -1/2 \\ for simplex table matrix and add arrow and coefficient at the end if necessary
// row index is index of the row 0-n starting with 0 at the first base variable, the row with variable names being -1
function renderValuesRow(props: SimplexTableRowAddititionArrowsProps, rowIndex: number): string {
    if (props.simplexTable.data.length === 0) {
        return ""
    }

    //baseVariable
    let res: string = `${props.simplexTable.baseVariables[rowIndex]}`;

    //valuesRow.length
    for (let i = 0; i < props.simplexTable.data[rowIndex].length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTable.data[rowIndex][i])} `
    }
    //rhs
    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTable.rhs[rowIndex])} `

    if (rowIndex === props.sourceRowIndex) {
        res += "& \\to & "
    } else if (props.targetRows.has(rowIndex)) {
        const by = props.targetRows.get(rowIndex);
        res += `& \\gets & \\times (${renderRationalWithNegativeSignOnly(by as Rational)})`
    }

    res += "\\\\[15pt]"
    return res
}

function renderObjectiveRow(props: SimplexTableRowAddititionArrowsProps): string {
    if (props.simplexTable.data.length === 0) {
        return ""
    }
    let res = "z";
    for (let i = 0; i < props.simplexTable.objectiveFunctionRow.length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTable.objectiveFunctionRow[i])} `
    }

    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTable.objectiveValue)}`

    res += "& & \\\\[15pt]"

    return res
}


function renderSimplexTableWithRowAdditionArrows(props: SimplexTableRowAddititionArrowsProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getKatexArrayTypeForTableRowAddition(props) + "\n"
    res += renderVariableNamesRow(props.simplexTable.variables)
    props.simplexTable.data.forEach((_, i) => {
        res += renderValuesRow(props, i) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderObjectiveRow(props) + "\n"
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res
}


function SimplexTableRowAddititionArrows(props: SimplexTableRowAddititionArrowsProps) {
    return <BlockMath math={renderSimplexTableWithRowAdditionArrows(props)} />
}

export default SimplexTableRowAddititionArrows;