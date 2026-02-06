/**
 * Common objects with math related objects
 */

//Rational should contain only positive values and use negative sign if needed
export default interface Rational {
    numerator: number;
    denominator: number;
    sign: RationalSign;
}

export default interface Matrix {
    data: Rational[]
}

//Math sign P being positive and N negative
export type RationalSign =
    | "P"
    | "N"

export type InequalitySign =
    | "GE"
    | "LE"