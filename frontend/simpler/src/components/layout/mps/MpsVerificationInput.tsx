import {Container} from "react-bootstrap";
import {useState} from "react";
import MPSInput from "./MpsInput.tsx";
import ConfirmButton from "./ConfirmButton.tsx";
import {verifyMpsCall} from "../../../api/verification/verificationApi.ts";
import {fetchHealthCheck} from "../../../api/manage/healthApi.ts";
import {hashStringSHA256} from "../../../utils/hash.ts";

function MpsVerificationInput() {

    const INPUT_MESSAGE = 'Enter your MPS code here...'
    const MPS_VERIF_SS_PERERFIX = 'MPS-VERIF-RES-'

    const [mpsCode, setMpsCode] = useState(INPUT_MESSAGE);

    const submitVerifyMps = () => {
        console.log("Submitting MPS for verification")
        console.log(mpsCode)

        if ((null === mpsCode) || (0 === mpsCode.length) || INPUT_MESSAGE === mpsCode) {
            alert("Cannot verify empty MPS.")
            return
        }

        const verifyMps = async() => {
            try {
                let dataHash = await hashStringSHA256(mpsCode)
                if (sessionStorage.getItem(MPS_VERIF_SS_PERERFIX + dataHash) != null ) {

                } else {
                    const verificationResponse = await verifyMpsCall({data: mpsCode})
                    sessionStorage.setItem(MPS_VERIF_SS_PERERFIX + dataHash, JSON.stringify(verificationResponse))
                }
            }
            catch (error) {
                console.log("Error verifying MPS: ", error)
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