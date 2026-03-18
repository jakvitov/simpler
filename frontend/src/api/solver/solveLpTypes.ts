export type SolverMethods =
    | "BASIC_SIMPLEX"
    | "TWO_PHASE"
    | "MULTIPLICATIVE"
    | "REVISED"
    | "BOUNDS_OPTIMISATION";

export type OptimisationTarget =
    | "MAX"
    | "MIN"

export type SolutionStatus =
    | "SOLVED"
    | "MAX_ITERATIONS"
    | "CYCLE"
    | "UNBOUNDED"

export interface SolveLpRequest {
    data: string,
    optimisationTarget: OptimisationTarget,
    method: SolverMethods,
}

export interface SolveLpErrorResponse {
    errors: string[],
    //common discriminator for responses
    success: boolean
}