import type {SimplexTable} from "../../../api/common/lpDefinitionTypes.ts";
import {demoMatrix, demoRational} from "../../../api/common/math.ts";
import {BlockMath} from "react-katex";
import {getKatexArrayType, renderValuesRow, renderVariableNamesRow} from "./SimplexTableComponent.tsx";

type SimplexTableObjectiveRowArrowProps = {
    simplexTable: SimplexTable;
    arrowColumn: number
    demo: boolean
}

function renderSimplexTable(props: SimplexTableObjectiveRowArrowProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}" + getKatexArrayType(props) + "\n"
    res += renderVariableNamesRow(props.simplexTable.variables)
    props.simplexTable.data.forEach((value, i) => {
        res += renderValuesRow(value, props.simplexTable.rhs[i], props.simplexTable.baseVariables[i]) + "\n"
    })
    res += "\\hline{}\\\\[1pt]\n"
    res += renderValuesRow(props.simplexTable.objectiveFunctionRow, props.simplexTable.objectiveValue, "z") + "\n"

    //Insert arrow into the right column, column 0 is first variable in variable names - second column in matrix
    for (let i = 0; i <= props.simplexTable.objectiveFunctionRow.length; i++) {
        if ((i - 1) === props.arrowColumn) {
            res += "\\uparrow "
        } else {
            res += "& "
        }
    }
    res += "\\end{array}\n"
    res += "\\end{pmatrix}\n"
    return res
}

function SimplexTableObjectiveRowArrow(props: SimplexTableObjectiveRowArrowProps) {
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
            arrowColumn: 2,
            demo: true
        }
    }

    console.log(renderSimplexTable(props))
    return <BlockMath math={renderSimplexTable(props)}></BlockMath>
}

export default SimplexTableObjectiveRowArrow;