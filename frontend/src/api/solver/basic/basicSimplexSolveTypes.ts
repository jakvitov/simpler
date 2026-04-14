import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {Rational} from "../../common/math.ts";
import type {AbstractSolutionResponseDto} from "../solveLpTypes.ts";

export interface SolveLpBasicSimplexResponseDto extends AbstractSolutionResponseDto{
    iterations: BasicSimplexIterationDto[]
    finalSimplexTable: SimplexTable
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