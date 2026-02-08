import MainHeader from "../../components/ui/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import PageContentHeader from "../../components/ui/PageContentHeader.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import {useNavigate, useParams} from "react-router-dom";
import {MPS_DATA_SS_PREFIX, MPS_VERIF_SS_PREFIX} from "../../utils/storageConstants.ts";
import type {MpsVerificationResponse} from "../../api/verification/verificationTypes.ts";
import MpsVerificationInput from "../../components/layout/mps/MpsVerificationInput.tsx";
import {get} from "idb-keyval";
import {useEffect, useState} from "react";
import ErrorEnumerationContainer from "../../components/ui/ErrorEnumerationContainer.tsx";

function MpsVerificationResults() {

    const { key } = useParams<{ key: string }>();
    const navigate = useNavigate();

    const [mpsData, setMpsData] = useState<string|null>(null)
    const [mpsVerificationResult, setMpsVerificationResult] = useState<MpsVerificationResponse|null>(null)

    useEffect(() => {
        get(MPS_DATA_SS_PREFIX + key).then(setMpsData)
        get(MPS_VERIF_SS_PREFIX + key).then((i) => JSON.parse(i) as MpsVerificationResponse).then(setMpsVerificationResult)
    }, [key])

    if (mpsVerificationResult === null || mpsData === null) {
        navigate("/verify-mps")
    }
    else if (mpsVerificationResult.status === "OK") {
        return (<>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <PageContentHeader value="MPS verified ✅"></PageContentHeader>
                <MpsVerificationInput initialText={mpsData} />
            </div>
            <BottomNavBar />
        </>)
    } else if (mpsVerificationResult.status === "VERIFICATION_FAILED") {
        return (<>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <PageContentHeader value="MPS verification failed ⚠️"></PageContentHeader>
                <ErrorEnumerationContainer mainReason={"Encountered problems during MPS verification:"} errors={mpsVerificationResult.errors} />
                <MpsVerificationInput initialText={mpsData} />
            </div>
            <BottomNavBar />
        </>)
    } else {
        alert("Application error occured. Unknown verification status encountered " + mpsVerificationResult.status)
        navigate("/verify-mps")
    }


}

export default MpsVerificationResults