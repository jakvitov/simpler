import type {AbstractSolutionResponseDto} from "../solveLpTypes.ts";
import type {SimplexTable} from "../../common/lpDefinitionTypes.ts";
import type {TwoPhaseSimplexObjectiveRowNormalizationDto} from "../two-phase/twoPhaseSimplexSolveTypes.ts";
import type {RevisedSimplexIterationDto} from "../revised/revisedSimplexSolveTypes.ts";
import type {Rational} from "../../common/math.ts";


export interface MultiplicativeSimplexIterationDto extends RevisedSimplexIterationDto {
    elementaryMatrix: Rational[]|undefined,
    elementaryMatrixInverse: Rational[]|undefined,
    nextIterationBasisInverse: Rational[]|undefined,
}

export interface MultiplicativeSimplexPhaseOneSolutionDto {
    initialSimplexTable: SimplexTable,
    artificialVariablesNormalization: TwoPhaseSimplexObjectiveRowNormalizationDto,
    iterations: MultiplicativeSimplexIterationDto[]
    resultBase: string[]|undefined,
}

export interface MultiplicativeSimplexPhaseTwoSolutionDto {
    initialFeasibleBase: string[]
    iterations: MultiplicativeSimplexIterationDto[]
}

export interface SolveLpMultiplicativeSimplexResponseDto extends AbstractSolutionResponseDto {
    multiplicativeSimplexPhaseOneSolutionDto: MultiplicativeSimplexPhaseOneSolutionDto|undefined
    multiplicativeSimplexPhaseTwoSolutionDto: MultiplicativeSimplexPhaseTwoSolutionDto|undefined
}