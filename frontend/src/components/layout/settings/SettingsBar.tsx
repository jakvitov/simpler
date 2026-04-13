import {Container} from "react-bootstrap";
import PageContentHeader from "../../ui/general/PageContentHeader.tsx";
import StorageUsageEstimate from "./StorageUsageEstimate.tsx";
import SolverConfigurationElement from "./SolverConfigurationElement.tsx";

function SettingsBar() {
    return (
        <Container>
            <PageContentHeader value={"Settings"}/>
            <StorageUsageEstimate />
            <SolverConfigurationElement />
        </Container>
    )
}

export default SettingsBar