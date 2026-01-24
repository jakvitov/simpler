import {Link} from "react-router-dom";
import {Container} from "react-bootstrap";

function MainHeader() {
    return (<Link
                 to="/" className="pb-0">
        <h1
            className="pt-2 ps-4 pb-3"
            style={{ backgroundColor: '#EBEBEB', fontSize: 50}}
        >Simpler solver</h1>
    </Link>)
}

export default MainHeader