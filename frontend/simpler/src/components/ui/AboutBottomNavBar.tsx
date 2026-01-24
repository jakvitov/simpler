import {Container, Navbar} from "react-bootstrap";

function AboutBottomNavBar() {
    return (<Navbar
        style={{
            fontSize: 15,
            bottom: '30px'
        }}
        className="fixed-bottom"
    >
        <Container fluid className={"d-flex justify-content-end"}>
            <span className="text-center me-3">About</span>
            <span className="text-center me-3"><a href={"https://github.com/jakvitov/simpler"}>Source</a></span>
        </Container>
    </Navbar>)
}

export default AboutBottomNavBar