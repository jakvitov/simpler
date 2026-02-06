import type {OptimisationTarget, SolverMethods, SolveSimplexResultType} from "../../../api/solver/solveLpTypes.ts";

type ProblemSpecificationDetailsProps = {
    target: OptimisationTarget,
    method: SolverMethods,
    status: SolveSimplexResultType
}

function ProblemSpecificationDetails(props: ProblemSpecificationDetailsProps) {
    return <>
        <h3 className={"pt-2"}>Solution specification:</h3>
        <p className={"pt-2"}>Solver status: {props.status}. Optimisation target: {props.target}. Solver method: {props.method}</p>
    </>
}

export default ProblemSpecificationDetails;