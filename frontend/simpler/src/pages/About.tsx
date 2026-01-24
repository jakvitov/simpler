import MainNavBar from "../components/layout/MainNavBar.tsx";
import AboutBottomNavBar from "../components/ui/AboutBottomNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";
import MainHeader from "../components/ui/MainHeader.tsx";

function About() {
    return (
        <>
            <MainHeader />
            <MainNavBar />
            <AboutBottomNavBar />
            <BottomNavBar />
        </>
    )
}

export default About