import { Container, Table } from "react-bootstrap";
import { useState } from "react";
import {SOLVER_CONFIGURATION_KEY} from "../../../utils/storageConstants.ts";
import type {SolverConfiguration} from "../../../api/solver/solveLpTypes.ts";


function SolverConfiguration() {

    const [configuration, setConfiguration] = useState<SolverConfiguration>(() => {
        const loadedConfig = localStorage.getItem(SOLVER_CONFIGURATION_KEY);
        if (loadedConfig == null) {
            return {
                basicSimplexMaxIterations: 20,
                basicSimplexMaxBaseCycles: 4,
                twoPhaseMaxIterations: 20,
                twoPhaseMaxBaseCycles: 4,
                revisedMaxIterations: 20,
                revisedMaxBaseCycles: 4
            }
        } else {
            return JSON.parse(loadedConfig);
        }
    });

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        setConfiguration(prev => ({
            ...prev,
            [name]: Number(value)
        }));
    };

    const validateConfiguration = (config: SolverConfiguration) => {
        Object.values(config).forEach(value => {
            if (isNaN(value)) {
                throw new Error("All configuration values must be valid numbers");
            }
            if (value <= 0) {
                throw new Error("All values must be greater than 0");
            }
        });
    };

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        try {
            validateConfiguration(configuration);
            localStorage.setItem(SOLVER_CONFIGURATION_KEY, JSON.stringify(configuration));
            console.log("Updated config:", configuration);
        } catch (err: any) {
            alert(err.message);
        }
    };

    return (
        <Container className="mt-2">
            <h3>Solver configuration</h3>
            <p>You can configure custom configuration for individual simplex variants below.</p>

            <form onSubmit={handleSubmit}>
                <Table bordered className="mt-2">
                    <thead>
                    <tr>
                        <th>Simplex variant</th>
                        <th>Parameter name</th>
                        <th>Value</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>Basic simplex</td>
                        <td>Max iterations</td>
                        <td>
                            <input
                                name="basicSimplexMaxIterations"
                                type="number"
                                value={configuration.basicSimplexMaxIterations}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>Max base cycles</td>
                        <td>
                            <input
                                name="basicSimplexMaxBaseCycles"
                                type="number"
                                value={configuration.basicSimplexMaxBaseCycles}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    <tr>
                        <td>Two phase simplex</td>
                        <td>Max iterations</td>
                        <td>
                            <input
                                name="twoPhaseMaxIterations"
                                type="number"
                                value={configuration.twoPhaseMaxIterations}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>Max base cycles</td>
                        <td>
                            <input
                                name="twoPhaseMaxBaseCycles"
                                type="number"
                                value={configuration.twoPhaseMaxBaseCycles}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    <tr>
                        <td>Revised simplex</td>
                        <td>Max iterations</td>
                        <td>
                            <input
                                name="revisedMaxIterations"
                                type="number"
                                value={configuration.revisedMaxIterations}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td>Max base cycles</td>
                        <td>
                            <input
                                name="revisedMaxBaseCycles"
                                type="number"
                                value={configuration.revisedMaxBaseCycles}
                                onChange={handleChange}
                            />
                        </td>
                    </tr>
                    </tbody>
                    <button type="submit">Update configuration</button>
                </Table>
            </form>
        </Container>
    );
}

export default SolverConfiguration;