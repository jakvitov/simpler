import type {SimplexTableLeavingRowNormalisationDto} from "../../../../api/solver/basic/basicSimplexSolveTypes.ts";
import type {SimplexTable} from "../../../../api/common/lpDefinitionTypes.ts";
import {type Rational, renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableLeavingRowNormalisationElementProps = {
    simplexTableLeavingEnteringVariableDto: SimplexTableLeavingRowNormalisationDto
}

function getSimplexTableLeavingRowNormalizationKatexArrayType(simplexTable: SimplexTable): string {
    let res = "{c|"
    for (let i = 0; i < simplexTable.variables.length; i++) {
        res += "c"
    }
    //For rhs and left arrow and multiplication coefficient
    res += "|c|cc}"
    return res
}

function renderVariableNamesRowForLeavingRowNormalisationElement(simplexTable: SimplexTable): string{
    if (simplexTable.variables.length == 0) {
        return "";
    }
    let res = ""

    for (let i = 0; i < simplexTable.variables.length; i++) {
        res += `& ${simplexTable.variables[i]} `
    }

    //RHS and space for left arrow and multiplication coefficient
    res += "& RHS & & "
    res += "\\\\[10pt]"
    return res
}

function renderSimplexTableLeavingRowNormalisationRow(props: SimplexTableLeavingRowNormalisationElementProps, rowIndex: number): string {
    let res: string = `${props.simplexTableLeavingEnteringVariableDto.simplexTableDto.baseVariables[rowIndex]}`;
    //Render data matrix values
    for (let i = 0; i < props.simplexTableLeavingEnteringVariableDto.simplexTableDto.data[rowIndex].length; i++) {
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.data[rowIndex][i])} `
    }
    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.rhs[rowIndex])} `

    if (rowIndex == props.simplexTableLeavingEnteringVariableDto.rowNormalizationIndex) {
        res += `& \\gets & \\times (${renderRationalWithNegativeSignOnly(props.simplexTableLeavingEnteringVariableDto.by as Rational)})`
    } else {
        res += res += "& \\to & "
    }
    res += "\\\\[15pt]"
    return res;
}

function renderObjectiveRow(props: SimplexTableLeavingRowNormalisationElementProps): string {
    if (props.simplexTableLeavingEnteringVariableDto.simplexTableDto.data.length === 0) {
        return ""
    }
    let res = "z";
    for (let i = 0; i < props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveFunctionRow.length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveFunctionRow[i])} `
    }

    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveValue)}`

    res += "& & \\\\[15pt]"

    return res
}

function renderSimplexTableLeavingRowNormalisationElement(props: SimplexTableLeavingRowNormalisationElementProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getSimplexTableLeavingRowNormalizationKatexArrayType(props.simplexTableLeavingEnteringVariableDto.simplexTableDto) + "\n"
    res += renderVariableNamesRowForLeavingRowNormalisationElement(props.simplexTableLeavingEnteringVariableDto.simplexTableDto)
    props.simplexTableLeavingEnteringVariableDto.simplexTableDto.data.forEach((_, i) => {
        res += renderSimplexTableLeavingRowNormalisationRow(props, i) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderObjectiveRow(props) + "\n"
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res
}

/**
 * Element rendering simplex table with normalization of the leaving variable row
 * Only one row is being normalized by the given coefficient
 * Render of SimplexTableLeavingRowNormalisationDto
 * @param props
 * @constructor
 */
function SimplexTableLeavingRowNormalisationElement(props: SimplexTableLeavingRowNormalisationElementProps) {
    return <BlockMath math={renderSimplexTableLeavingRowNormalisationElement(props)} />
}

export default SimplexTableLeavingRowNormalisationElement;