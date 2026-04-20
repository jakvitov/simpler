import { useState, forwardRef, useImperativeHandle } from "react";

type Operator = ">=" | "<=" | "=";

export type LPInteractiveInputState = {
    variables: string[];
    objective: string[];
    rows: string[][];
    operators: Operator[];
    rhs: string[];
};

export type LPInteractiveInputHandle = {
    getData: () => LPInteractiveInputState;
};

const LPInteractiveInputForm = forwardRef<LPInteractiveInputHandle>((_props: any, ref) => {
    const [state, setState] = useState<LPInteractiveInputState>({
        variables: ["x1", "x2"],
        objective: ["0", "0"],
        rows: [
            ["0", "0"],
            ["0", "0"],
        ],
        operators: [">=", "<="] as Operator[],
        rhs: ["0", "0"],
    });

    useImperativeHandle(ref, () => ({
        getData: () => state,
    }));

    const addVariable = () => {
        const newVar = `x${state.variables.length + 1}`;

        setState((prev) => ({
            ...prev,
            variables: [...prev.variables, newVar],
            objective: [...prev.objective, "0"],
            rows: prev.rows.map((r) => [...r, "0"]),
        }));
    };

    const removeVariable = (index: number) => {
        setState((prev) => ({
            ...prev,
            variables: prev.variables.filter((_, i) => i !== index),
            objective: prev.objective.filter((_, i) => i !== index),
            rows: prev.rows.map((r) => r.filter((_, i) => i !== index)),
        }));
    };

    const addRow = () => {
        setState((prev) => ({
            ...prev,
            rows: [...prev.rows, Array(prev.variables.length).fill("0")],
            operators: [...prev.operators, ">="] as Operator[],
            rhs: [...prev.rhs, "0"],
        }));
    };

    const removeRow = (index: number) => {
        setState((prev) => ({
            ...prev,
            rows: prev.rows.filter((_, i) => i !== index),
            operators: prev.operators.filter((_, i) => i !== index),
            rhs: prev.rhs.filter((_, i) => i !== index),
        }));
    };

    const updateObjective = (i: number, value: string) => {
        setState((prev) => {
            const objective = [...prev.objective];
            objective[i] = value;
            return { ...prev, objective };
        });
    };

    const updateCell = (r: number, c: number, value: string) => {
        setState((prev) => {
            const rows = [...prev.rows];
            rows[r] = [...rows[r]];
            rows[r][c] = value;
            return { ...prev, rows };
        });
    };

    const updateRhs = (r: number, value: string) => {
        setState((prev) => {
            const rhs = [...prev.rhs];
            rhs[r] = value;
            return { ...prev, rhs };
        });
    };

    const updateOp = (r: number, value: Operator) => {
        setState((prev) => {
            const operators = [...prev.operators];
            operators[r] = value;
            return { ...prev, operators };
        });
    };

    return (
        <div className="p-4 space-y-4">
            <div className="flex gap-2">
                <button onClick={addVariable} className={"mb-2 border-0"}>
                    + Variable
                </button>
                <button onClick={addRow} className="ms-2 mb-2 border-0">
                    + Constraint
                </button>
            </div>

            <div className="overflow-auto">
                <table className="min-w-full border-collapse">
                    <thead>
                    <tr>
                        {state.variables.map((v, i) => (
                            <th key={i} className="border p-2">
                                <div className="flex items-center justify-center gap-1">
                                    <input
                                        className="w-20 text-center border"
                                        value={v}
                                        onChange={(e) => {
                                            setState((prev) => {
                                                const variables = [...prev.variables];
                                                variables[i] = e.target.value;
                                                return { ...prev, variables };
                                            });
                                        }}
                                    />
                                    <button onClick={() => removeVariable(i)} className="font-bold border-0 ms-2">
                                        -
                                    </button>
                                </div>
                            </th>
                        ))}
                        <th className="border p-2">Sign</th>
                        <th className="border p-2">RHS</th>
                        <th className="border p-2">Actions</th>
                    </tr>
                    </thead>

                    <tbody>
                    {/* Constraint rows */}
                    {state.rows.map((row, rIdx) => (
                        <tr key={rIdx}>
                            {row.map((cell, cIdx) => (
                                <td key={cIdx} className="border p-2">
                                    <input
                                        type="text"
                                        className="w-20 border text-center"
                                        value={cell}
                                        onChange={(e) => updateCell(rIdx, cIdx, e.target.value)}
                                    />
                                </td>
                            ))}

                            <td className="border p-2">
                                <select
                                    value={state.operators[rIdx]}
                                    onChange={(e) => updateOp(rIdx, e.target.value as Operator)}
                                    className="border"
                                >
                                    <option value=">=">≥</option>
                                    <option value="<=">≤</option>
                                    <option value="=">=</option>
                                </select>
                            </td>

                            <td className="border p-2">
                                <input
                                    type="text"
                                    className="w-24 border text-center"
                                    value={state.rhs[rIdx]}
                                    onChange={(e) => updateRhs(rIdx, e.target.value)}
                                />
                            </td>

                            <td className="border p-2 text-center">
                                <button onClick={() => removeRow(rIdx)} className="font-bold border-0">
                                    -
                                </button>
                            </td>
                        </tr>
                    ))}

                    {/* Objective row (BOTTOM) */}
                    <tr>
                        {state.objective.map((val, i) => (
                            <td key={i} className="border p-2">
                                <input
                                    type="text"
                                    className="w-20 border text-center"
                                    value={val}
                                    onChange={(e) => updateObjective(i, e.target.value)}
                                />
                            </td>
                        ))}
                        <td className="border p-2 text-center font-bold" colSpan={3}>
                            Z
                        </td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <p className={"pt-4 text-black-50"}>Interactive input is still under development and not yet finished.</p>
        </div>
    );
});

export default LPInteractiveInputForm;