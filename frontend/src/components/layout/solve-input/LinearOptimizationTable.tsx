import { useState } from "react";

type Operator = ">=" | "<=" | "=";

function InteractiveLpInputTable() {
    const [variables, setVariables] = useState<string[]>(["x1", "x2"]);
    const [rows, setRows] = useState<string[][]>([
        ["0", "0"],
        ["0", "0"],
    ]);
    const [operators, setOperators] = useState<Operator[]>([">=", "<="]);
    const [rhs, setRhs] = useState<string[]>(["0", "0"]);

    const addVariable = () => {
        const newVar = `x${variables.length + 1}`;
        setVariables([...variables, newVar]);
        setRows(rows.map((r) => [...r, "0"]));
    };

    const removeVariable = (index: number) => {
        const updatedVars = [...variables];
        updatedVars.splice(index, 1);
        setVariables(updatedVars);

        const updatedRows = rows.map((r) => {
            const copy = [...r];
            copy.splice(index, 1);
            return copy;
        });
        setRows(updatedRows);
    };

    const addRow = () => {
        setRows([...rows, Array(variables.length).fill("0")]);
        setOperators([...operators, ">="]);
        setRhs([...rhs, "0"]);
    };

    const removeRow = (index: number) => {
        setRows(rows.filter((_, i) => i !== index));
        setOperators(operators.filter((_, i) => i !== index));
        setRhs(rhs.filter((_, i) => i !== index));
    };

    const updateCell = (rowIdx: number, colIdx: number, value: string) => {
        const updated = [...rows];
        updated[rowIdx][colIdx] = value;
        setRows(updated);
    };

    const updateRhs = (rowIdx: number, value: string) => {
        const updated = [...rhs];
        updated[rowIdx] = value;
        setRhs(updated);
    };

    const updateOp = (rowIdx: number, value: Operator) => {
        const updated = [...operators];
        updated[rowIdx] = value;
        setOperators(updated);
    };

    return (
        <div className="p-4 space-y-4">
            <div className="flex gap-2">
                <button
                    onClick={addVariable}
                    className="px-3 py-1 bg-blue-500 text-white rounded"
                >
                    + Variable
                </button>
                <button
                    onClick={addRow}
                    className="px-3 py-1 bg-green-500 text-white rounded"
                >
                    + Constraint
                </button>
            </div>

            <div className="overflow-auto border rounded">
                <table className="min-w-full border-collapse">
                    <thead>
                    <tr>
                        {variables.map((v, i) => (
                            <th key={i} className="border p-2">
                                <div className="flex items-center justify-center gap-1">
                                    <input
                                        className="w-20 text-center border rounded"
                                        value={v}
                                        onChange={(e) => {
                                            const copy = [...variables];
                                            copy[i] = e.target.value;
                                            setVariables(copy);
                                        }}
                                    />
                                    <button
                                        onClick={() => removeVariable(i)}
                                        className="text-red-500 font-bold px-1"
                                        title="Remove variable"
                                    >
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
                    {rows.map((row, rIdx) => (
                        <tr key={rIdx}>
                            {row.map((cell, cIdx) => (
                                <td key={cIdx} className="border p-2">
                                    <input
                                        type="text"
                                        className="w-20 border rounded text-center"
                                        value={cell}
                                        onChange={(e) => updateCell(rIdx, cIdx, e.target.value)}
                                    />
                                </td>
                            ))}
                            <td className="border p-2">
                                <select
                                    value={operators[rIdx]}
                                    onChange={(e) => updateOp(rIdx, e.target.value as Operator)}
                                    className="border rounded"
                                >
                                    <option value=">=">≥</option>
                                    <option value="<=">≤</option>
                                    <option value="=">=</option>
                                </select>
                            </td>
                            <td className="border p-2">
                                <input
                                    type="text"
                                    className="w-24 border rounded text-center"
                                    value={rhs[rIdx]}
                                    onChange={(e) => updateRhs(rIdx, e.target.value)}
                                />
                            </td>
                            <td className="border p-2 text-center">
                                <button
                                    onClick={() => removeRow(rIdx)}
                                    className="text-red-500 font-bold"
                                >
                                    -
                                </button>
                            </td>
                        </tr>
                    ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}

export default InteractiveLpInputTable