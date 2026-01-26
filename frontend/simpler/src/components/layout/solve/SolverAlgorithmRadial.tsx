import {Navbar, NavbarText} from "react-bootstrap";
import type {SolverMethods} from "../../../api/solver/solveLpTypes.ts";

type SolverAlgorithmRadialProps = {
    onSelected: (value: SolverMethods) => void;
    currentSelected: SolverMethods|null;
}


function SolverAlgorithmRadial(props: SolverAlgorithmRadialProps) {

    //console.log(typeof(props.onSelected))

    return (<div style={{ backgroundColor: '#D9D9D9'}}>
        <Navbar className={"pt-3 pb-3"}>
            <NavbarText className={"ms-4 me-4"}>Solver algorithm</NavbarText>
            <input checked={props.currentSelected === "BASIC_SIMPLEX"} onChange={() => props.onSelected("BASIC_SIMPLEX")} className="form-check-input me-1" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>
            <label className="form-check-label me-4" htmlFor="flexRadioDefault1">
                Basic simplex
            </label>
            <input checked={props.currentSelected === "TWO_PHASE"} onChange={() => props.onSelected("TWO_PHASE")} className="form-check-input me-1" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>
            <label className="form-check-label me-4" htmlFor="flexRadioDefault1">
                Two-phase
            </label>
            <input checked={props.currentSelected === "MULTIPLICATIVE"} onChange={() => props.onSelected("MULTIPLICATIVE")} className="form-check-input me-1" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>
            <label className="form-check-label me-4" htmlFor="flexRadioDefault1">
                Multiplicative
            </label>
            <input checked={props.currentSelected === "REVISED"} onChange={() => props.onSelected("REVISED")} className="form-check-input me-1" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>
            <label className="form-check-label me-4" htmlFor="flexRadioDefault1">
                Revised
            </label>
            <input checked={props.currentSelected === "BOUNDS_OPTIMISATION"} onChange={() => props.onSelected("BOUNDS_OPTIMISATION")} className="form-check-input me-1" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>
            <label className="form-check-label me-4" htmlFor="flexRadioDefault1">
                Bounds optimisations
            </label>
        </Navbar>
        </div>
    )
}

export default SolverAlgorithmRadial