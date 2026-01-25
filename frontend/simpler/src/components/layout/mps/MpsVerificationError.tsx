import {Container} from "react-bootstrap";

type MpsVerificationErrorProps = {
    errors: string[]
}

function formMpsVerificationErrorText(props: MpsVerificationErrorProps): Element {
    let res = "Encountered problems during MPS verification: \n"
    for (let i = 0; i < props.errors.length; i++) {
        if (i === res.length - 1) {
            res += props.errors[i]
        } else {
            res += (props.errors[i] + "\n")
        }
    }
    return res
}

function MpsVerificationError(props: MpsVerificationErrorProps): Element {
    return (<Container><Container className={"mt-3 mb-4"} style={{
        padding: '0.375rem 0.75rem',
        backgroundColor: '#F5F5F5',
        fontSize: '1rem',
        fontFamily: 'monospace',
        lineHeight: '1.5',
        pointerEvents: 'none',
        wordWrap: 'break-word',
        zIndex: 1,}}><pre>{formMpsVerificationErrorText(props)}</pre> </Container></Container>)
}

export default MpsVerificationError