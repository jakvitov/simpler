import {Container} from "react-bootstrap";

type ConfirmButtonProps = {
    onChange: () => void;
    text?: string
}

function ConfirmButton(props: ConfirmButtonProps) {
    return (
        <Container className="text-center pt-3">
            <button onClick={props.onChange} className={"border-0"}><span className={"m-3"}>{(props.text === null || props.text === undefined) ? "Submit" : props.text}</span></button>
        </Container>
    )
}

export default ConfirmButton