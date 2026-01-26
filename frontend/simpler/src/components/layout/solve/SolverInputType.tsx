import {Container, Nav, Navbar} from "react-bootstrap";
import {Link} from "react-router-dom";

function SolverInputType() {
    return (
        <Navbar expand="lg" className="pt-0 main-navbar border-top border-start border-end mt-4" style={{background: "white", fontSize: 20}}>
        <Nav className="w-100 d-flex">
            <Nav.Link
                as={Link}
                to="/verify-mps"
                className="w-50 text-center border-end"
            >
                MPS input
            </Nav.Link>
            <Nav.Link
                as={Link}
                to="/solve-lp"
                className="w-50 text-center"
            >
                Interactive input
            </Nav.Link>
        </Nav>
        </Navbar>)
}

export default SolverInputType