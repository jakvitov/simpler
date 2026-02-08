import type {SolveSimplexResultType} from "../solveLpTypes.ts";
import type {ParsedLpDefinition, SimplexTable} from "../../common/lpDefinitionTypes.ts";

export default interface BasicSimplexSolveResponse {
    success: boolean
    result: SolveSimplexResultType
    parsedLP: ParsedLpDefinition|null
    initialST: SimplexTable
}