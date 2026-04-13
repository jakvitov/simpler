import {Container} from "react-bootstrap";
import PageContentHeader from "../../ui/general/PageContentHeader.tsx";
import StorageUsageEstimate from "./StorageUsageEstimate.tsx";
import SolverConfiguration from "./SolverConfiguration.tsx";

function SettingsBar() {
    return (
        <Container>
            <PageContentHeader value={"Settings"}/>
            <StorageUsageEstimate />
            <SolverConfiguration />
        </Container>
    )
}

export default SettingsBar