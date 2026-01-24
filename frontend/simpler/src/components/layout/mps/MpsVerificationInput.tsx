import {Container} from "react-bootstrap";
import {useState} from "react";
import MPSInput from "./MpsInput.tsx";
import ConfirmButton from "./ConfirmButton.tsx";

function MpsVerificationInput() {

    const [mpsCode, setMpsCode] = useState('');

    return (
        <Container>
            <MPSInput
                value={mpsCode}
                onChange={setMpsCode}
                placeholder="Enter your MPS code here..."
                rows={12}
            />
            <ConfirmButton/>
        </Container>
    )
}

export default MpsVerificationInput