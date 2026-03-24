import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {Rational} from "../../common/math.ts";
import type {BasicSimplexIterationDto} from "../basic/basicSimplexSolveTypes.ts";
import type {SolutionStatus} from "../solveLpTypes.ts";


export interface TwoPhaseSimplexObjectiveRowNormalizationDto {
    simplexTableDto: SimplexTable,
    coefficients: Record<number, Rational>
}

export interface TwoPhaseSimplexPhaseOneSolutionDto {
    initialSimplexTable: SimplexTable,
    simplexTableWithRestoredObjectiveRow: SimplexTable,
    artificialVariablesNormalization: TwoPhaseSimplexObjectiveRowNormalizationDto,
    iterations: BasicSimplexIterationDto[],
    finalSimplexTable: SimplexTable
}

export interface TwoPhaseSimplexPhaseTwoSolutionDto {
    initialSimplexTable: SimplexTable,
    simplexTableWithRestoredObjectiveRow: SimplexTable,
    objectiveRowToBaseVariablesAdjustment: TwoPhaseSimplexObjectiveRowNormalizationDto,
    iterations: BasicSimplexIterationDto[],
    finalSimplexTable: SimplexTable
}

export interface SolveLpTwoPhaseSimplexResponseDto {
    initialSimplexTable: SimplexTable,
    phaseOneSolutionDto: TwoPhaseSimplexPhaseOneSolutionDto,
    phaseTwoSolutionDto: TwoPhaseSimplexPhaseTwoSolutionDto|undefined,
    solutionStatus: SolutionStatus,
    resultVariableValues: Record<string, Rational>|undefined,
    solutionObjectiveFunctionValue: Rational|undefined,
    success: boolean
}



