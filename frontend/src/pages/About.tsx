import MainNavBar from "../components/layout/MainNavBar.tsx";
import AboutBottomNavBar from "../components/ui/AboutBottomNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";
import MainHeader from "../components/ui/general/MainHeader.tsx";

function About() {
    return (
        <>
            <div className={"page-content"}>
                <MainHeader />
                <MainNavBar />
            </div>
            <AboutBottomNavBar />
            <BottomNavBar />
        </>
    )
}

export default About