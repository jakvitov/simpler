import {Container} from "react-bootstrap";
import PageContentHeader from "../../ui/general/PageContentHeader.tsx";
import StorageUsageEstimate from "./StorageUsageEstimate.tsx";

function SettingsBar() {
    return (
        <Container>
            <PageContentHeader value={"Settings"}/>
            <StorageUsageEstimate />
        </Container>
    )
}

export default SettingsBar