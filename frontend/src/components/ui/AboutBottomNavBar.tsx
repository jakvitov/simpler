import {Container, Navbar} from "react-bootstrap";
import {Link} from "react-router-dom";

function AboutBottomNavBar() {
    return (<Navbar
        style={{
            fontSize: 15,
            bottom: '30px'
        }}
        className="fixed-bottom"
    >
        <Container fluid className={"d-flex justify-content-end"}>
            <Link to="/report-bug" className="text-center me-3">Bug</Link>
            <Link to="/about" className="text-center me-3">About</Link>
            <span className="text-center me-3"><a href={"https://github.com/jakvitov/simpler"}>Source</a></span>
        </Container>
    </Navbar>)
}

export default AboutBottomNavBar