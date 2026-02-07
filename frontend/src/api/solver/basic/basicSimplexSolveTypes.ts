import type {SolveSimplexResultType} from "../solveLpTypes.ts";
import type {ParsedLpDefinition} from "../../common/lpDefinitionTypes.ts";

export default interface BasicSimplexSolveResponse {
    success: boolean
    result: SolveSimplexResultType
    parsedLP: ParsedLpDefinition|null
}