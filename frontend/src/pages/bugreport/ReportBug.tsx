import MainHeader from "../../components/ui/general/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../components/ui/general/PageContentHeader.tsx";
import {Container} from "react-bootstrap";
import CommonErrorBoundary from "../../components/ui/error/CommonErrorBoundary.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import type {LastErrorResponseDto} from "../../api/bugreport/lastErrorTypes.ts";
import {useEffect, useState} from "react";
import {fetchLastErrorData} from "../../api/bugreport/lastErrorApi.ts";
import BugReportInputTextField from "../../components/layout/bugreport/BugReportInputTextField.tsx";

function ReportBug() {

    //undefined - not loaded, null - not found on backend
    const [lastErrorDto, setLastErrorDto] = useState<LastErrorResponseDto|undefined|null>(undefined)

    useEffect(() => {
        fetchLastErrorData().then(i => setLastErrorDto(i));
    })

    if (lastErrorDto === undefined) {

    } else {
        return (<>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <PageContentHeader value={"Report bug 🐞"}></PageContentHeader>
                <Container>
                    <CommonErrorBoundary>
                        <BugReportInputTextField lastErrorResponseDto={lastErrorDto} />
                    </CommonErrorBoundary>
                </Container>
            </div>
            <BottomNavBar />
        </>)
    }
}

export default ReportBug;