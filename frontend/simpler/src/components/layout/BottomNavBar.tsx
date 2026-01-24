import { Navbar, Container } from 'react-bootstrap';

function BottomNavBar() {
    return (
        <Navbar
            style={{ backgroundColor: '#EBEBEB', fontSize: 10}}
            className="fixed-bottom border-top"
        >
            <Container fluid className="d-flex justify-content-between">
                <span>Version : 1.0.0</span>
                <span>Backend: RUNNING</span>
            </Container>
        </Navbar>
    );
}

export default BottomNavBar;