import type {SimplexTable} from "../common/lpDefinitionTypes.ts";
import type {Rational} from "../common/math.ts";

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

export interface SolverConfiguration {
    basicSimplexMaxIterations: number;
    basicSimplexMaxBaseCycles: number;
    twoPhaseMaxIterations: number;
    twoPhaseMaxBaseCycles: number;
    revisedMaxIterations: number;
    revisedMaxBaseCycles: number;
}

export interface SolveLpRequest {
    data: string,
    optimisationTarget: OptimisationTarget,
    solverConfiguration: SolverConfiguration|null
    method: SolverMethods,
}

export interface SolveLpErrorResponse {
    errors: string[],
    //common discriminator for responses
    success: boolean
}

export interface AbstractSolutionResponseDto {
    solutionStatus: SolutionStatus,
    initialSimplexTable: SimplexTable,
    resultVariableValues: Record<string, Rational>|undefined,
    solutionObjectiveFunctionValue: Rational|undefined,
    success: boolean
}