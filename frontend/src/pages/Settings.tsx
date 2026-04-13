import MainHeader from "../components/ui/general/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";
import SettingsBar from "../components/layout/settings/SettingsBar.tsx";
import AboutBottomNavBar from "../components/ui/AboutBottomNavBar.tsx";

function Settings() {
    return (<>
        <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
            <SettingsBar/>
        </div>
        <AboutBottomNavBar />
        <BottomNavBar />
    </>)
}

export default Settings