export type SolverMethods =
    | "BASIC_SIMPLEX"
    | "TWO_PHASE"
    | "MULTIPLICATIVE"
    | "REVISED"
    | "BOUNDS_OPTIMISATION";

export type OptimisationTarget =
    | "MAX"
    | "MIN"
