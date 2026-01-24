import {Container} from "react-bootstrap";

function ConfirmButton() {
    return (
        <Container className="text-center pt-3">
            <button className={"border-0"}><span className={"m-3"}>Submit</span></button>
        </Container>
    )
}

export default ConfirmButton