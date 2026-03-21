import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {Rational} from "../../common/math.ts";
import type {SolutionStatus} from "../solveLpTypes.ts";

export interface SolveLpBasicSimplexResponseDto {
    solutionStatus: SolutionStatus
    initialSimplexTable: SimplexTable
    iterations: BasicSimplexIterationDto[]
    finalSimplexTable: SimplexTable
    resultVariableValues: Record<string, Rational>
    solutionObjectiveFunctionValue: Rational,
    success: boolean
}

export interface BasicSimplexIterationDto {
    simplexTableLeavingEnteringVariableDto: SimplexTableLeavingEnteringVariableDto
    simplexTableLeavingRowNormalisationDto: SimplexTableLeavingRowNormalisationDto,
    simplexTableRowsNormalizationDto: SimplexTableRowsNormalizationDto,
    simplexTableAfterVariableSwitch: SimplexTable
}

export interface SimplexTableLeavingEnteringVariableDto {
    simplexTableDto: SimplexTable
    tVector: Rational[]
    leavingVariableIndex: number,
    enteringVariableIndex: number
}

export interface SimplexTableLeavingRowNormalisationDto {
    simplexTableDto: SimplexTable
    rowNormalizationIndex: number,
    by: Rational
}

export interface SimplexTableRowsNormalizationDto {
    simplexTableDto: SimplexTable,
    coefficients: Record<number, Rational>
    objectiveRowCoefficient: Rational
    leavingVariableIndex: number
}