export type SolverMethods =
    | "BASIC_SIMPLEX"
    | "TWO_PHASE"
    | "MULTIPLICATIVE"
    | "REVISED"
    | "BOUNDS_OPTIMISATION";

export type OptimisationTarget =
    | "MAX"
    | "MIN"

export type SolveSimplexResultType =
    | "SOLVED"
    | "UNBOUNDED"
    | "ERROR"

export default interface SolveLpRequest {
    data: string,
    optimisationTarget: OptimisationTarget
}