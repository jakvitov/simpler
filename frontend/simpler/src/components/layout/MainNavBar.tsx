import {Nav, Navbar} from "react-bootstrap"
import { Link } from 'react-router-dom';

function MainNavBar () {
    return (<Navbar expand="lg" className="pt-0 main-navbar border-bottom sticky-top" style={{background: "white", fontSize: 25}}>
        <Nav className="w-100 d-flex">
            <Nav.Link
                as={Link}
                to="/verify-mps"
                className="w-50 text-center border-end"
            >
                Verify MPS
            </Nav.Link>
            <Nav.Link
                as={Link}
                to="/solve-lp"
                className="w-50 text-center"
            >
                Solve LP
            </Nav.Link>
        </Nav>
    </Navbar>)
}

export default MainNavBar