import MainHeader from "../components/ui/MainHeader.tsx";
import MainNavBar from "../components/layout/MainNavBar.tsx";
import BottomNavBar from "../components/layout/BottomNavBar.tsx";
import SettingsBar from "../components/layout/SettingsBar.tsx";

function Settings() {
    return (<>
        <div className={"page-content"}>
            <MainHeader />
            <MainNavBar />
            <SettingsBar/>
        </div>
        <BottomNavBar />
    </>)
}

export default Settings