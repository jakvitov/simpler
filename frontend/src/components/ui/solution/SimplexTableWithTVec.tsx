import type {SimplexTable} from "../../../api/common/lpDefinitionTypes.ts";
import {type Rational, renderRationalWithNegativeSignOnly} from "../../../api/common/math.ts";
import {renderSimplexTableWithObjectiveRowArrow} from "./SimplexTableObjectiveRowArrow.tsx";
import {BlockMath} from "react-katex";

type SimplexTableWithTVecProps = {
    simplexTable: SimplexTable,
    tVec: Rational[],
    pivotRowIndex: number,
    pivotColIndex: number
}

function renderTVecIwhtPivotRowArrow(props: SimplexTableWithTVecProps): string {
    let res = "\\begin{pmatrix}\n"
    res += "\\begin{array}{cc}"
    res += "t \\\\[10pt]\n"
    props.tVec.forEach((t_vec_item, index) => {
        if (index === props.pivotRowIndex) {
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


function SimplexTableWithTVec(props: SimplexTableWithTVecProps) {
    let simplexTableRenderCode = renderSimplexTableWithObjectiveRowArrow({simplexTable: props.simplexTable, arrowColumn: props.pivotColIndex, demo: false});
    console.log(simplexTableRenderCode);
    let tVecRendered = renderTVecIwhtPivotRowArrow(props)
    console.log(tVecRendered);

    return <BlockMath math={`${simplexTableRenderCode + tVecRendered}`}/>
}

export default SimplexTableWithTVec;