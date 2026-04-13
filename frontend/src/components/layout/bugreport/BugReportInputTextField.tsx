import { useState, useEffect } from 'react';
import type {LastErrorResponseDto} from "../../../api/bugreport/lastErrorTypes.ts";
import ConfirmButton from "../../ui/general/ConfirmButton.tsx";
import {Container} from "react-bootstrap";

type BugReportTextInputFieldProps = {
    lastErrorResponseDto: LastErrorResponseDto|null
}

function mailBugReport(message: string) {
    const targetMail: string = "reporting@simplersolver.com";
    const subject = encodeURIComponent('Bug Report');
    const encodedBody = encodeURIComponent(message);
    window.open(`mailto:${targetMail}?subject=${subject}&body=${encodedBody}`, '_blank');
}

function prepareBugReportTextFromLastError(lastErrorResponseDto: LastErrorResponseDto): string {
    let res = "SIMPLER bug report. Included data:\n"
    res += "Error time: " + lastErrorResponseDto.exceptionTime + "\n"
    res += "Simpler version: " + lastErrorResponseDto.version + "\n"
    res += "Request: " + lastErrorResponseDto.request.substring(0, 1500) + "\n"
    res += "Error message: " + lastErrorResponseDto.exceptionMessage.substring(0, 100) + "\n"
    res += "Error stack: " + lastErrorResponseDto.stackTrace.substring(0, 300) + "\n"
    return res.substring(0, 1999);
}

/**
 * Text input field with last bugreport info in it
 * If backend does not register any last bugreport - null is in props and describe bugreport message is displayed
 * @param props
 * @constructor
 */
export default function BugReportInputTextField(props: BugReportTextInputFieldProps) {
    const [text, setText] = useState<string|null>(null);
    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = e.target.value;
        setText(newValue);
    };

    useEffect(() => {
        if (props.lastErrorResponseDto == null) {
            setText("No last bugreport found. Please describe your issue, inputs, actions and version bellow.")
        } else {
            setText(prepareBugReportTextFromLastError(props.lastErrorResponseDto));
        }
    }, [text]);

    if (text == null) {
        return <></>
    } else {
        return (<>
            <Container style={{ backgroundColor: '#F5F5F5'}}>
                <textarea
                value={text}
                onChange={handleChange}
                placeholder={"Bug report."}
                rows={20}
                spellCheck={false}
                style={{
                    width: '100%',
                    backgroundColor: '#F5F5F5',
                    color: 'black',
                    fontSize: '1rem',
                    fontFamily: 'monospace',
                    lineHeight: '1.5',
                    border: '0px',
                    padding: '0.375rem 0.75rem',
                    resize: 'none',
                }}
            />
            </Container>
        <ConfirmButton onChange={() => mailBugReport(text)} />
        </>)
    }

}