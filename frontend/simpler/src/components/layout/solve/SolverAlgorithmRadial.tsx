import {Navbar, NavbarText} from "react-bootstrap";
import type {OptimisationTarget, SolverMethods} from "../../../api/solver/solveLpTypes.ts";

type SolverAlgorithmRadialProps = {
    onSelectedSolverMethod: (value: SolverMethods) => void;
    currentSelectedSolverMethod: SolverMethods;

    onSelectedOptimisationTarget: (value: OptimisationTarget) => void;
    currentSelectedOptimisationTarget: OptimisationTarget
}


function SolverAlgorithmRadial(props: SolverAlgorithmRadialProps) {

    return (
        <div style={{ backgroundColor: '#D9D9D9'}}>
            <Navbar className="pt-3 pb-3 d-flex justify-content-between align-items-center">

                {/* Left group: Solver algorithms */}
                <div className="d-flex align-items-center">
                    <NavbarText className="me-4">Solver algorithm</NavbarText>

                    <input
                        checked={props.currentSelectedSolverMethod === "BASIC_SIMPLEX"}
                        onChange={() => props.onSelectedSolverMethod("BASIC_SIMPLEX")}
                        className="form-check-input me-1"
                        type="radio"
                        name="solverAlgorithm"
                        id="basicSimplex"
                    />
                    <label className="form-check-label me-4" htmlFor="basicSimplex">
                        Basic simplex
                    </label>

                    <input
                        checked={props.currentSelectedSolverMethod === "TWO_PHASE"}
                        onChange={() => props.onSelectedSolverMethod("TWO_PHASE")}
                        className="form-check-input me-1"
                        type="radio"
                        name="solverAlgorithm"
                        id="twoPhase"
                    />
                    <label className="form-check-label me-4" htmlFor="twoPhase">
                        Two-phase
                    </label>

                    <input
                        checked={props.currentSelectedSolverMethod === "MULTIPLICATIVE"}
                        onChange={() => props.onSelectedSolverMethod("MULTIPLICATIVE")}
                        className="form-check-input me-1"
                        type="radio"
                        name="solverAlgorithm"
                        id="multiplicative"
                    />
                    <label className="form-check-label me-4" htmlFor="multiplicative">
                        Multiplicative
                    </label>

                    <input
                        checked={props.currentSelectedSolverMethod === "REVISED"}
                        onChange={() => props.onSelectedSolverMethod("REVISED")}
                        className="form-check-input me-1"
                        type="radio"
                        name="solverAlgorithm"
                        id="revised"
                    />
                    <label className="form-check-label me-4" htmlFor="revised">
                        Revised
                    </label>

                    <input
                        checked={props.currentSelectedSolverMethod === "BOUNDS_OPTIMISATION"}
                        onChange={() => props.onSelectedSolverMethod("BOUNDS_OPTIMISATION")}
                        className="form-check-input me-1"
                        type="radio"
                        name="solverAlgorithm"
                        id="boundsOptimisation"
                    />
                    <label className="form-check-label me-4" htmlFor="boundsOptimisation">
                        Bounds optimisations
                    </label>
                </div>

                {/* Right group: MIN / MAX */}
                <div className="d-flex align-items-center">
                    <input
                        checked={props.currentSelectedOptimisationTarget === "MIN"}
                        onChange={() => props.onSelectedOptimisationTarget("MIN")}
                        className="form-check-input me-1"
                        type="radio"
                        name="minMax"
                        id="min"
                    />
                    <label className="form-check-label me-4" htmlFor="min">
                        MIN
                    </label>

                    <input
                        checked={props.currentSelectedOptimisationTarget === "MAX"}
                        onChange={() => props.onSelectedOptimisationTarget("MAX")}
                        className="form-check-input me-1"
                        type="radio"
                        name="minMax"
                        id="max"
                    />
                    <label className="form-check-label me-4" htmlFor="max">
                        MAX
                    </label>
                </div>
            </Navbar>
        </div>
    )
}

export default SolverAlgorithmRadial