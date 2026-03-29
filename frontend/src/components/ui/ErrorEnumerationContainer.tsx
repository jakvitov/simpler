import {Container} from "react-bootstrap";
import ReportBugLink from "./error/ReportBugLink.tsx";

type ErrorEnumerationContainerProps = {
    mainReason: string
    errors: string[]
}

function formMpsVerificationErrorText(props: ErrorEnumerationContainerProps): string {
    let res = props.mainReason + "\n"
    for (let i = 0; i < props.errors.length; i++) {
        if (i === res.length - 1) {
            res += props.errors[i]
        } else {
            res += (props.errors[i] + "\n")
        }
    }
    return res
}

function ErrorEnumerationContainer(props: ErrorEnumerationContainerProps) {
    return (<Container><Container className={"mt-3 mb-4"} style={{
        padding: '0.375rem 0.75rem',
        backgroundColor: '#F5F5F5',
        fontSize: '1rem',
        fontFamily: 'monospace',
        lineHeight: '1.5',
        pointerEvents: 'none',
        wordWrap: 'break-word',
        zIndex: 1,}}><pre>{formMpsVerificationErrorText(props)}</pre></Container><ReportBugLink /></Container>)
}

export default ErrorEnumerationContainer