import SimplexTableComponent from "../../ui/solution/SimplexTableComponent.tsx";
import type {SimplexTable} from "../../../api/common/lpDefinitionTypes.ts";

type InitialSimplexTableProps = {
    initialST: SimplexTable
}


function InitialSimplexTable(props: InitialSimplexTableProps) {
    return <>
        <h3 className={"pt-2"}>Initial simplex table:</h3>
        <SimplexTableComponent simplexTable={props.initialST} demo={true}></SimplexTableComponent>
    </>
}

export default InitialSimplexTable