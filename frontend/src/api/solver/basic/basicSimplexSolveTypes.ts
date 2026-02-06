import type {OptimisationTarget, SolveSimplexResultType} from "../solveLpTypes.ts";
import type {ParsedLpDefinition} from "../../common/lpDefinitionTypes.ts";

export default interface BasicSimplexSolveResponse {
    data: string,
    result: SolveSimplexResultType
    optimisationTarget: OptimisationTarget
    parsedLP: ParsedLpDefinition|null
}