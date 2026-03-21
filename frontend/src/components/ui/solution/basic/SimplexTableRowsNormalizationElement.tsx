import type {SimplexTableRowsNormalizationDto} from "../../../../api/solver/basic/basicSimplexSolveTypes.ts";
import {type Rational, renderRationalWithNegativeSignOnly} from "../../../../api/common/math.ts";
import {BlockMath} from "react-katex";

type SimplexTableRowNormalizationElementProps = {
    simplexTableRowNormalizationElementDto: SimplexTableRowsNormalizationDto
}

function getKatexArrayTypeForSimplexTableRowsNormalizationElement(props: SimplexTableRowNormalizationElementProps): string {
    let res = "{c|"
    for (let i = 0; i < props.simplexTableRowNormalizationElementDto.simplexTableDto.variables.length; i++) {
        res += "c"
    }
    //For rhs, arrow and coefficient
    res += "|c|cc}"
    return res
}

function renderSimplexTableRowsNormalizationElementVariableNamesRow(props: SimplexTableRowNormalizationElementProps): string {
    if (props.simplexTableRowNormalizationElementDto.simplexTableDto.variables.length === 0) {
        return "";
    }
    let res = ""

    for (let i = 0; i < props.simplexTableRowNormalizationElementDto.simplexTableDto.variables.length; i++) {
        res += `& ${props.simplexTableRowNormalizationElementDto.simplexTableDto.variables[i]} `
    }

    //Leave space for arrow and coefficient after RHS
    res += "& RHS & "
    res += "\\\\[10pt]"
    return res
}

//We receive coefficients as Record from BE and it needs to be transformed to a map separately
function renderSimplexTableRowsNormalizationElementRow(props: SimplexTableRowNormalizationElementProps, rowIndex: number, coefficientsMap: Map<number, Rational>): string {
    console.log("Map type: " + typeof props.simplexTableRowNormalizationElementDto.coefficients)
    if (props.simplexTableRowNormalizationElementDto.simplexTableDto.data.length === 0) {
        return ""
    }

    //baseVariable
    let res: string = `${props.simplexTableRowNormalizationElementDto.simplexTableDto.baseVariables[rowIndex]}`;

    //valuesRow.length
    for (let i = 0; i < props.simplexTableRowNormalizationElementDto.simplexTableDto.data[rowIndex].length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableRowNormalizationElementDto.simplexTableDto.data[rowIndex][i])} `
    }
    //rhs
    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableRowNormalizationElementDto.simplexTableDto.rhs[rowIndex])} `

    if (coefficientsMap.has(rowIndex)) {
        res += `& \\gets & \\times (${renderRationalWithNegativeSignOnly(coefficientsMap.get(rowIndex) as Rational)})`
    } else {
        //Leaving variable row is not multiplied
        res += `& \\to & Source`
    }

    res += "\\\\[15pt]"
    return res
}

function renderSimplexTableRowsNormalizationElementObjectiveRow(props: SimplexTableRowNormalizationElementProps): string {
    if (props.simplexTableRowNormalizationElementDto.simplexTableDto.data.length === 0) {
        return ""
    }
    let res = "z";
    for (let i = 0; i < props.simplexTableRowNormalizationElementDto.simplexTableDto.objectiveFunctionRow.length; i++) {
        //
        res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableRowNormalizationElementDto.simplexTableDto.objectiveFunctionRow[i])} `
    }

    res += `& ${renderRationalWithNegativeSignOnly(props.simplexTableRowNormalizationElementDto.simplexTableDto.objectiveValue)}`

    res += "& & \\\\[15pt]"

    return res
}

function renderSimplexTableRowsNormalizationElement(props: SimplexTableRowNormalizationElementProps): string {
    const coefficientsMap = new Map<number, Rational>(
        Object.entries(props.simplexTableRowNormalizationElementDto.coefficients).map(([k, v]) => [Number(k), v])
    );

    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getKatexArrayTypeForSimplexTableRowsNormalizationElement(props) + "\n"
    res += renderSimplexTableRowsNormalizationElementVariableNamesRow(props)

    props.simplexTableRowNormalizationElementDto.simplexTableDto.data.forEach((_, i) => {
        res += renderSimplexTableRowsNormalizationElementRow(props, i, coefficientsMap) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderSimplexTableRowsNormalizationElementObjectiveRow(props) + "\n"
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res;
}

/**
 * Element rendering the simplex table with rows normalization by already normalized leaving variable row
 * Each row has multiplication coefficient by which it is multiplied numbers of leaving variable row
 * Renders SimplexTableRowsNormalizationDto
 * @param props
 * @constructor
 */
function SimplexTableRowsNormalizationElement(props: SimplexTableRowNormalizationElementProps) {
    return <BlockMath math={renderSimplexTableRowsNormalizationElement(props)} />
}

export default SimplexTableRowsNormalizationElement;