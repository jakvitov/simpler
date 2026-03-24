import type {
    TwoPhaseSimplexObjectiveRowNormalizationDto
} from "../../../../api/solver/two-phase/twoPhaseSimplexSolveTypes.ts";
import {type Rational, renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type TwoPhaseSimplexObjectiveRowNormalizationElementProps = {
    twoPhaseSimplexObjectiveRowNormalizationDto: TwoPhaseSimplexObjectiveRowNormalizationDto
}

function getKatexArrayTypeForSimplexObjectiveRowNormalizationElement(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps): string {
    let res = "{c|"
    for (let i = 0; i < props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.variables.length; i++) {
        res += "c"
    }
    //For rhs, arrow and coefficient
    res += "|c|cc}"
    return res
}

function renderSimplexTableObjectiveRowNormalizationElementVariableNamesRow(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps): string {
    if (props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.variables.length === 0) {
        return "";
    }
    let res = ""

    for (let i = 0; i < props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.variables.length; i++) {
        res += `& ${props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.variables[i]} `
    }

    //Leave space for arrow and coefficient after RHS
    res += "& RHS & "
    res += "\\\\[10pt]"
    return res
}

function renderSimplexTableObjectiveRowNormalizationElementRow(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps, rowIndex: number, coefficientsMap: Map<number, Rational>): string {
    if (props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.data.length === 0) {
        return ""
    }

    //baseVariable
    let res: string = `${props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.baseVariables[rowIndex]}`;

    //valuesRow.length
    for (let i = 0; i < props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.data[rowIndex].length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.data[rowIndex][i])} `
    }
    //rhs
    res += `& ${renderRationalWithNegativeSignOnly(props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.rhs[rowIndex])} `

    if (coefficientsMap.has(rowIndex)) {
        res += `& \\to & \\times (${renderRationalWithNegativeSignOnly(coefficientsMap.get(rowIndex) as Rational)})`
    } else {
        //Row is not being added to objective row
        res += `&  & `
    }

    res += "\\\\[15pt]"
    return res
}

function renderSimplexTableObjectiveRowNormalizationElementObjectiveRow(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps): string {
    if (props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.data.length === 0) {
        return ""
    }
    let res = "z";
    for (let i = 0; i < props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.objectiveFunctionRow.length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.objectiveFunctionRow[i])} `
    }

    res += `& ${renderRationalWithNegativeSignOnly(props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.objectiveValue)}`
    res += `& \\gets & \\\\[15pt]`

    return res
}

function renderTwoPhaseSimplexObjectiveRowNormalizationElement(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps): string {
    let coefficients = new Map<number, Rational>()
    if (props.twoPhaseSimplexObjectiveRowNormalizationDto.coefficients != null) {
        coefficients = new Map<number, Rational>(
            Object.entries(props.twoPhaseSimplexObjectiveRowNormalizationDto.coefficients).map(([k, v]) => [Number(k), v])
        );
    }

    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getKatexArrayTypeForSimplexObjectiveRowNormalizationElement(props) + "\n"
    res += renderSimplexTableObjectiveRowNormalizationElementVariableNamesRow(props)
    props.twoPhaseSimplexObjectiveRowNormalizationDto.simplexTableDto.data.forEach((_, i) => {
        res += renderSimplexTableObjectiveRowNormalizationElementRow(props, i, coefficients) + "\n"
    })

    res += "\\hline{}\\\\[1pt]\n"
    res += renderSimplexTableObjectiveRowNormalizationElementObjectiveRow(props) + "\n"
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res;
}

/**
 * Element containing objective row normalization in two phase simplex
 * Renders TwoPhaseSimplexObjectiveRowNormalizationDto
 * @param props
 * @constructor
 */
function TwoPhaseSimplexObjectiveRowNormalizationElement(props: TwoPhaseSimplexObjectiveRowNormalizationElementProps) {
    return <BlockMath math={renderTwoPhaseSimplexObjectiveRowNormalizationElement(props)} />
}

export default TwoPhaseSimplexObjectiveRowNormalizationElement