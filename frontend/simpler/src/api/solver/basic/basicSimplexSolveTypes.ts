import type {OptimisationTarget, SolveSimplexResultType} from "../solveLpTypes.ts";

export default interface BasicSimplexSolveResponse {
    data: string,
    result: SolveSimplexResultType
    optimisationTarget: OptimisationTarget
}