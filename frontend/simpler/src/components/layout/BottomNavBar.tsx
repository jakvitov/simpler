import {Container, Navbar} from 'react-bootstrap';
import {useEffect, useState} from "react";
import {fetchHealthCheck} from "../../api/manage/healthApi.ts";

function BottomNavBar() {

    const [version, setVersion] = useState("Loading...")
    const [health, setHealth] = useState("Loading...")

    useEffect(() => {
        const loadHealthCheck = async() => {
            try {
                console.log("Fetchinig healthcheck")
                const data = await fetchHealthCheck();
                setVersion(data.version)
                setHealth(data.status)
            }
            catch (error) {
                console.log("Error fetching version: ", error)
                setVersion("Unknown")
                setHealth("ERROR")
            }
        }
        loadHealthCheck()
    }, [])


    return (
        <Navbar
            style={{ backgroundColor: '#EBEBEB', fontSize: 10}}
            className="fixed-bottom border-top"
        >
            <Container fluid className="d-flex justify-content-between">
                <span>Version : {version}</span>
                <span>Backend: {health}</span>
            </Container>
        </Navbar>
    );
}

export default BottomNavBar;