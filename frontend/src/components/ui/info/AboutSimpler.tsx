import { Container } from "react-bootstrap";
import {Link} from "react-router-dom";

function AboutSimpler() {
    return (<>
        <Container className={"pt-5 pb-5"}>
            <h2>About</h2>
            <p>Simpler <i>(Simplex solver)</i> is an educational application that was created in order to facilitate easier learning of linear programming as a part of bachelor thesis focusing on the field of operations research.</p>
            <h3 className={"mt-4"}>Source code</h3>
            <p>This software is fully free of charge and is distributed under the <Link to={"https://github.com/jakvitov/simpler/blob/main/LICENSE"}>MIT licence</Link>. Full source code is hosted and available at the projects <a href={"https://github.com/jakvitov/simpler"}>GitHub page</a>.</p>
            <h3>Detailed manual</h3>
            <p>Detailed manual is available at <Link to={"https://simplersolver.com"}>simplersolver.com</Link>.</p>
        </Container>
    </>)
}

export default AboutSimpler;