import type {SimplexTableLeavingEnteringVariableDto} from "../../../../api/solver/basic/basicSimplexSolveTypes.ts";
import {
    getPlainSimplexTableKatexArrayType,
    renderValuesRow,
    renderVariableNamesRow
} from "./PlainSimplexTableElement.tsx";
import {renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableLeavingEnteringVariableElementProps = {
    simplexTableLeavingEnteringVariableDto: SimplexTableLeavingEnteringVariableDto
}

/**
 * Render T-Vec with left arrow in the entering variable column
 * @param props
 */
function renderTVecWithLeavingVariable(props: SimplexTableLeavingEnteringVariableElementProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}{cc}"
    res += "t \\\\[10pt]\n"
    props.simplexTableLeavingEnteringVariableDto.tVector.forEach((t_vec_item, index) => {
        if (index === props.simplexTableLeavingEnteringVariableDto.leavingVariableIndex) {
            res += `${renderRationalWithNegativeSignOnly(t_vec_item)} & \\gets \\\\[15pt]\n`
        } else  {
            res += `${renderRationalWithNegativeSignOnly(t_vec_item)} & \\\\[15pt]\n`
        }
    })
    res += "\\hline{}\\\\[1pt]"
    res += "\\\\[15pt]\n"
    res += "\\phantom{\\uparrow}\n"
    res += "\\end{array}"
    res += "\\end{pmatrix}"
    return res;
}

/**
 * Render simplex table with up arrow in the entering variable column
 * @param props
 */
function renderSimplexTableWithEnteringVariable(props: SimplexTableLeavingEnteringVariableElementProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getPlainSimplexTableKatexArrayType(props.simplexTableLeavingEnteringVariableDto.simplexTableDto) + "\n"
    res += renderVariableNamesRow(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.variables)
    props.simplexTableLeavingEnteringVariableDto.simplexTableDto.data.forEach((value, i) => {
        res += renderValuesRow(value, props.simplexTableLeavingEnteringVariableDto.simplexTableDto.rhs[i], props.simplexTableLeavingEnteringVariableDto.simplexTableDto.baseVariables[i]) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderValuesRow(props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveFunctionRow, props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveValue, "z") + "\n"

    for (let i = 0; i <= props.simplexTableLeavingEnteringVariableDto.simplexTableDto.objectiveFunctionRow.length; i++) {
        if ((i - 1) === props.simplexTableLeavingEnteringVariableDto.enteringVariableIndex) {
            res += "\\uparrow "
        } else {
            res += "& "
        }
    }

    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res
}

/**
 * Element rendering the SimplexTableLeavingEnteringVariableDto into a BlockMath
 * @param props
 * @constructor
 */
function SimplexTableLeavingEnteringVariableElement(props: SimplexTableLeavingEnteringVariableElementProps) {
    const simplexTableWithEnteringVariableCode = renderSimplexTableWithEnteringVariable(props);
    if (props.simplexTableLeavingEnteringVariableDto.tVector == null) {
        return <BlockMath math={`${simplexTableWithEnteringVariableCode}`}/>
    }
    const tVecWithLeavingVariableCode = renderTVecWithLeavingVariable(props);
    return <BlockMath math={`${simplexTableWithEnteringVariableCode + tVecWithLeavingVariableCode}`}/>
}

export default SimplexTableLeavingEnteringVariableElement;