import MainHeader from "../components/ui/general/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import IntroManual from "../components/ui/IntroManual.tsx";
import AboutBottomNavBar from "../components/ui/AboutBottomNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";

function Home() {
    return (
        <>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
                <IntroManual />
            </div>
            <AboutBottomNavBar />
            <BottomNavBar />
        </>
    )
}

export default Home