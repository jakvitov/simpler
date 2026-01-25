import MainHeader from "../components/ui/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import PageContentHeader from "../components/ui/PageContentHeader.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";
import {useNavigate, useParams} from "react-router-dom";
import {MPS_DATA_SS_PREFIX, MPS_VERIF_SS_PREFIX} from "../utils/sessionStorageConstants.ts";
import type {MpsVerificationResponse} from "../api/verification/verificationTypes.ts";
import MpsVerificationInput from "../components/layout/mps/MpsVerificationInput.tsx";
import MpsVerificationError from "../components/layout/mps/MpsVerificationError.tsx";

function MpsVerificationResults() {

    const { key } = useParams<{ key: string }>();
    const navigate = useNavigate();


    if (sessionStorage.getItem(MPS_VERIF_SS_PREFIX + key) != null) {

        const mpsData: string|null = sessionStorage.getItem(MPS_DATA_SS_PREFIX + key)

        const mpsVerificationRaw = sessionStorage.getItem(MPS_VERIF_SS_PREFIX + key);

        const verificationResult: MpsVerificationResponse | null =
            mpsVerificationRaw ? (JSON.parse(mpsVerificationRaw) as MpsVerificationResponse) : null;

        if (mpsData === null || verificationResult === null) {
            navigate("/verify-mps")
        }
        else if (verificationResult.status === "OK") {
            return (<>
                <div className={"page-content"}>
                    <MainHeader />
                    <MainNavBar />
                    <PageContentHeader value="MPS verified ✅"></PageContentHeader>
                    <MpsVerificationInput initialText={mpsData} />
                </div>
                <BottomNavBar />
            </>)
        } else if (verificationResult.status === "VERIFICATION_FAILED") {
            return (<>
                <div className={"page-content"}>
                    <MainHeader />
                    <MainNavBar />
                    <PageContentHeader value="MPS verification failed ⚠️"></PageContentHeader>
                    <MpsVerificationError errors={verificationResult.errors} />
                    <MpsVerificationInput initialText={mpsData} />
                </div>
                <BottomNavBar />
            </>)
        }

    } else {
        navigate("/verify-mps")
    }
}

export default MpsVerificationResults