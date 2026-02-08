/**
 * Common objects with math related objects
 */

//Rational should contain only positive values and use negative sign if needed
export interface Rational {
    numerator: number;
    denominator: number;
    sign: RationalSign;
}

export function renderRationalWithSign(r: Rational): string {
    return (r.sign === "P" ? "+" : "-") + "\\frac{" + r.denominator + "}{" + r.numerator + "}";
}

export function renderRationalWithNegativeSignOnly(r: Rational): string {
    if (r.sign === "N") {
        return renderRationalWithSign(r)
    } else {
        return "\\frac{" + r.denominator + "}{" + r.numerator + "}";
    }
}

export interface Matrix {
    data: Rational[]
}

//Math sign P being positive and N negative
export type RationalSign =
    | "P"
    | "N"

export type InequalitySign =
    | "GE"
    | "LE"