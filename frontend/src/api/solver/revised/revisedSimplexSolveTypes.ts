import type {AbstractSolutionResponseDto} from "../solveLpTypes.ts";
import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {TwoPhaseSimplexObjectiveRowNormalizationDto} from "../two-phase/twoPhaseSimplexSolveTypes.ts";
import type {Rational} from "../../common/math.ts";

export interface NonBasicVariableCurrentReducedCostCalculationDto {
    CJ: Rational,
    AJ: Rational[][],
    variableName: String
    nonBasicVariableReducedCost: Rational,
    result: Rational
}

export interface RevisedSimplexIterationDto {
    currentBasis: string[],
    initialBasisMatrix: Rational[][],
    initialBasisMatrixInverse: Rational[][],
    b: Rational[][]
    XB: Rational[][],
    originalSimplexTableReducedCosts: Rational[][],
    YT: Rational[][],
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

export interface SolveLpRevisedSimlexResponseDto extends AbstractSolutionResponseDto {
    revisedSimplexPhaseOneSolution: RevisedSimplexPhaseOneSolutionDto|undefined;
    revisedSimplexPhaseTwoSolutionDto: RevisedSimplexPhaseTwoSolutionDto|undefined;
}