import {Container} from "react-bootstrap";

type PageContentHeaderProps = {
    value: string
}

function PageContentHeader(props: PageContentHeaderProps) {
    return (
    <Container className={"pt-4"}>
        <h2>{props.value}</h2>
    </Container>
    )
}

export default PageContentHeader