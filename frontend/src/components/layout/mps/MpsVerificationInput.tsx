import {Container} from "react-bootstrap";
import {useEffect, useState} from "react";
import MPSInput from "./MpsInput.tsx";
import ConfirmButton from "../../ui/ConfirmButton.tsx";
import {verifyMpsCall} from "../../../api/verification/verificationApi.ts";
import {hashStringSHA256} from "../../../utils/hash.ts";
import {useNavigate} from "react-router-dom";
import {LAST_MPS_INPUT_DATA, MPS_DATA_SS_PREFIX, MPS_VERIF_SS_PREFIX} from "../../../utils/storageConstants.ts";
import type {MpsVerificationResponse} from "../../../api/verification/verificationTypes.ts";
import {get, set} from 'idb-keyval';

type MpsVerificationInputProps = {
    initialText?: string
}

function MpsVerificationInput(props: MpsVerificationInputProps) {

    const INPUT_MESSAGE = 'Enter your MPS code here...'

    const [mpsCode, setMpsCode] = useState((props.initialText === null || props.initialText === undefined) ? INPUT_MESSAGE : props.initialText);
    const navigate = useNavigate();

    useEffect(() => {
        const previousInput: string|null = localStorage.getItem(LAST_MPS_INPUT_DATA);
        if (previousInput !== null) {
            setMpsCode(previousInput)
        }
    }, []);

    const submitVerifyMps = () => {

        if ((null === mpsCode) || (0 === mpsCode.length) || INPUT_MESSAGE === mpsCode) {
            alert("Cannot verify empty MPS.")
            return
        }

        const verifyMps = async() => {
            console.log("TRRIGGERED  MPS VERIFICATION")
            try {
                let dataHash = await hashStringSHA256(mpsCode)

                if (await get(MPS_VERIF_SS_PREFIX + dataHash) === undefined) {
                    const verificationResponse: MpsVerificationResponse  = await verifyMpsCall({data: mpsCode})
                    await set(MPS_VERIF_SS_PREFIX + dataHash, JSON.stringify(verificationResponse))
                }
                if (await get(MPS_DATA_SS_PREFIX + dataHash) === undefined) {
                    await set(MPS_DATA_SS_PREFIX + dataHash, mpsCode)
                }
                navigate(`/verify-mps/results/${dataHash}`)
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