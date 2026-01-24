import MainHeader from "../components/ui/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import Manual from "../components/ui/Manual.tsx";
import AboutBottomNavBar from "../components/ui/AboutBottomNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";

function Home() {
    return (
        <>
            <MainHeader />
            <MainNavBar />
            <Manual />
            <AboutBottomNavBar />
            <BottomNavBar />
        </>
    )
}

export default Home