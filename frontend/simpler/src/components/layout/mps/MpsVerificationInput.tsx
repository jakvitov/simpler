import {Container} from "react-bootstrap";
import {useState} from "react";
import MPSInput from "./MpsInput.tsx";
import ConfirmButton from "./ConfirmButton.tsx";
import {verifyMpsCall} from "../../../api/verification/verificationApi.ts";
import {hashStringSHA256} from "../../../utils/hash.ts";
import {useNavigate} from "react-router-dom";
import {MPS_DATA_SS_PREFIX, MPS_VERIF_SS_PREFIX} from "../../../utils/sessionStorageConstants.ts";
import type {MpsVerificationResponse} from "../../../api/verification/verificationTypes.ts";

type MpsVerificationInputProps = {
    initialText?: string
}

function MpsVerificationInput(props: MpsVerificationInputProps) {

    const INPUT_MESSAGE = 'Enter your MPS code here...'

    const [mpsCode, setMpsCode] = useState((props.initialText === null || props.initialText === undefined) ? INPUT_MESSAGE : props.initialText);
    const navigate = useNavigate();

    const submitVerifyMps = () => {

        if ((null === mpsCode) || (0 === mpsCode.length) || INPUT_MESSAGE === mpsCode) {
            alert("Cannot verify empty MPS.")
            return
        }

        const verifyMps = async() => {
            try {
                let dataHash = await hashStringSHA256(mpsCode)
                if (sessionStorage.getItem(MPS_VERIF_SS_PREFIX + dataHash) == null ) {
                    const verificationResponse: MpsVerificationResponse  = await verifyMpsCall({data: mpsCode})
                    sessionStorage.setItem(MPS_VERIF_SS_PREFIX + dataHash, JSON.stringify(verificationResponse))
                }
                if (sessionStorage.getItem(MPS_DATA_SS_PREFIX + dataHash) == null) {
                    sessionStorage.setItem(MPS_DATA_SS_PREFIX + dataHash, mpsCode)
                }
                navigate(`/mps-verification-results/${dataHash}`)
            }
            catch (error) {
                console.log("Error during MPS verification: ", error)
            }
        }
        verifyMps()
    }

    return (
        <Container>
            <MPSInput
                value={mpsCode}
                onChange={setMpsCode}
                rows={12}
            />
            <ConfirmButton onChange={submitVerifyMps}/>
        </Container>
    )
}

export default MpsVerificationInput