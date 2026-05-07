import { Container } from "react-bootstrap";

function AboutSimpler() {
    return (<>
        <Container className={"pt-5 pb-5"}>
            <h2>About</h2>
            <p>Simpler <i>(Simplex solver)</i> is an educational application that was created in order to facilitate easier learning of linear programming as a part of bachelor thesis focusing on the field of operations research.</p>
            <h3 className={"mt-4"}>Source code</h3>
            <p>This software is fully free of charge and is distributed under the <a href={"https://github.com/jakvitov/simpler/blob/main/LICENSE"}>MIT licence</a>. Full source code is hosted and available at the projects <a href={"https://github.com/jakvitov/simpler"}>GitHub page</a>.</p>
            <></>
        </Container>
    </>)
}

export default AboutSimpler;