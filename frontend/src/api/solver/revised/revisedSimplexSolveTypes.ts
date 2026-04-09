import type {SolutionStatus} from "../solveLpTypes.ts";
import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {TwoPhaseSimplexObjectiveRowNormalizationDto} from "../two-phase/twoPhaseSimplexSolveTypes.ts";
import type {Rational} from "../../common/math.ts";

export interface NonBasicVariableCurrentReducedCostCalculationDto {
    cJ: Rational,
    aJ: Rational[][],
    variableName: String
    nonBasicVariableReducedCost: Rational,
    result: Rational
}

export interface RevisedSimplexIterationDto {
    currentBasis: string[],
    initialBasisMatrix: Rational[][],
    initialBasisMatrixInverse: Rational[][],
    b: Rational[][]
    xB: Rational[][],
    originalSimplexTableReducedCosts: Rational[][],
    yT: Rational[][],
    nonBasicVariablesCurrentReducedCosts: NonBasicVariableCurrentReducedCostCalculationDto[],
    enteringVariableIndex: number|undefined,
    enteringVariableName: string|undefined,
    enteringVariableColumnInOriginalSimplexTable: Rational[][]|undefined
    directionVector: Rational[][]|undefined,
    ratioVector: Rational[]|undefined,
    leavingVariableName: string|undefined,
    leavingVariableIndex: number|undefined,
    updatedBasis: string[]|undefined
}

export interface RevisedSimplexPhaseOneSolutionDto {
    initialSimplexTable: SimplexTable;
    artificialVariablesNormalization: TwoPhaseSimplexObjectiveRowNormalizationDto;
    iterations: RevisedSimplexIterationDto[],
    resultBase: string[]|undefined
}

export interface RevisedSimplexPhaseTwoSolutionDto {
    initialFeasibleBase: string[],
    iterations: RevisedSimplexIterationDto[],
}

export interface SolveLpRevisedSimlexResponseDto {
    solutionStatus: SolutionStatus;
    initialSimplexTable: SimplexTable;
    revisedSimplexPhaseOneSolution: RevisedSimplexPhaseOneSolutionDto|undefined;
    resultVariableValues: Record<string, Rational>|undefined;
    solutionObjectiveFunctionValue: Rational|undefined;
    success: boolean
}