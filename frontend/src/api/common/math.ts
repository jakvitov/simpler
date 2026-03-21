/**
 * Common objects with math related objects
 */

//Rational should contain only positive values and use negative sign if needed
export interface Rational {
    numerator: number;
    denominator: number;
    sign: RationalSign;
}

export function renderRationalWithSign(r: Rational|undefined): string {
    if (r == undefined) {
        return "ERROR_UNDEFINED"
    }
    if (r.denominator == 1) {
        return (r.sign === "P" ? "+" : "-") + r.numerator;
    }
    return (r.sign === "P" ? "+" : "-") + "\\dfrac{" + r.numerator + "}{" + r.denominator + "}";
}

export function renderRationalWithNegativeSignOnly(r: Rational|undefined): string {
    if (r == undefined) {
        return "ERROR_UNDEFINED"
    }
    if (r.sign === "P") {
        if (r.denominator == 1) {
            return "\\phantom{+}" + r.numerator
        } else {
            //Phantom + makes alignment with - in matrices possible
            return "\\phantom{+}" + "\\dfrac{" + r.numerator + "}{" + r.denominator + "}";
        }
    } else {
        if (r.denominator == 1) {
            return "- " + r.numerator;
        } else {
            return "- \\dfrac{" + r.numerator + "}{" + r.denominator + "}";
        }
    }
}

export function demoRational(): Rational {
    let diceRoll = Math.floor(Math.random() * 6)
    if (diceRoll > 3) {
        return {numerator: 2, denominator: 3, sign: "P"}
    }
    return {numerator: 3, denominator: 8, sign: "N"}
}

export function demoMatrix(m: number, n: number): Rational[][] {
    if (m <= 0 || n <= 0) {
        throw "Number of rows and columns must be greater than 0";
    }

    let res: Rational[][] = []
    let buf: Rational[] = []
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            buf.push(demoRational())
        }
        res.push(buf)
        buf = []
    }
    return res
}

//Math sign P being positive and N negative
export type RationalSign =
    | "P"
    | "N"

export type InequalitySign =
    | "GE"
    | "LE"
    | "EQ"
    | "N"