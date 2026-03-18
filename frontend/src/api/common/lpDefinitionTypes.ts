import type {InequalitySign} from "./math.ts";
import type {Rational} from "./math.ts";


export interface LpDefinitonVariableValue {
    variableName: string,
    value: Rational
}


/**
 * One line in LP definition
 */
export interface LpDefinitionLine {
    variableValues: LpDefinitonVariableValue[],
    inequalitySign: InequalitySign,
    rhs: Rational
}

/**
 * Representation of single variable LP bound
 */
export interface Bound {
    variableName: string,
    upperbound: Rational|null,
    lowerbound: Rational|null
}

/**
 * Parsed LP definition to be returned by BE with parsed MPS to structured LP problem
 */
export interface ParsedLpDefinition {
    lines: LpDefinitionLine[],
    bounds: Bound[],
    warningMessage: string|null
}

export interface SimplexTable {
    variables: string[],
    baseVariables: string[],
    data: Rational[][],
    rhs: Rational[],
    objectiveFunctionRow: Rational[],
    objectiveValue: Rational
}





