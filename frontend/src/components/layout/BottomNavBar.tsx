import {Container, Navbar} from 'react-bootstrap';
import {useEffect, useState} from "react";
import {fetchHealthCheck} from "../../api/manage/healthApi.ts";
import {clear} from "idb-keyval";

function BottomNavBar() {

    const [version, setVersion] = useState("Loading...")
    const [health, setHealth] = useState("Loading...")

    useEffect(() => {
        const loadHealthCheck = async() => {
            try {
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

        const intervalID = setInterval(() => {
            loadHealthCheck()
        }, 5000)

        //Cleanup interval when component unmounts
        return () => clearInterval(intervalID)
    }, [])

    const handleCleanSorage = () => {
        console.log("Cleaning all persisted data. ")
        clear().then(() => console.log("Persisted data cleaned."))
    }

    return (
        <Navbar
            style={{ backgroundColor: '#EBEBEB', fontSize: 10}}
            className="fixed-bottom border-top"
        >
            <Container fluid className="d-flex justify-content-between">
                <span>Version : {version}</span>
                <td><a onClick={handleCleanSorage} href={"#"}> Clean storage</a></td>
                <span>Backend: {health}</span>
            </Container>
        </Navbar>
    );
}

export default BottomNavBar;