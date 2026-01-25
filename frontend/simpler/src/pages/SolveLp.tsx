import MainHeader from "../components/ui/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";

function SolveLp() {
    return (
    <>
        <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
        </div>
        <BottomNavBar />
    </>
    )
}

export default SolveLp