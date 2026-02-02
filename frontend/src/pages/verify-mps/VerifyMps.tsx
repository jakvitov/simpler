import MainHeader from "../../components/ui/MainHeader.tsx";
import MainNavBar from "../../components/layout/MainNavBar.tsx";
import BottomNavBar from "../../components/layout/BottomNavBar.tsx";
import PageContentHeader from "../../components/ui/PageContentHeader.tsx";
import MpsVerificationInput from "../../components/layout/mps/MpsVerificationInput.tsx";

function VerifyMps() {
    return (
    <>
        <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
            <PageContentHeader value="MPS verification"></PageContentHeader>
            <MpsVerificationInput />
        </div>
        <BottomNavBar />
    </>
    )
}

export default VerifyMps