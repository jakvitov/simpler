import React from "react";
import { Row, Col } from "react-bootstrap";

type DividerWithTextProps = {
    text: string;
};

const DividerWithText: React.FC<DividerWithTextProps> = ({ text }) => {
    return (
        <Row className="align-items-center my-4">
            <Col>
                <div style={{ borderTop: "1px solid #ccc", width: "100%" }} />
            </Col>

            <Col xs="auto">
                <h3 className="mb-0 text-center px-3">{text}</h3>
            </Col>

            <Col>
                <div style={{ borderTop: "1px solid #ccc", width: "100%" }} />
            </Col>
        </Row>
    );
};

export default DividerWithText;