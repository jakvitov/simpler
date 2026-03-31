package com.github.jakvitov.simplex;

import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.mps.RowType;
import org.hipparchus.fraction.BigFraction;

import java.util.*;

public class SimplexTable {

    public List<String> variables = new ArrayList<>();
    public List<String> baseVariables = new ArrayList<>();

    public List<List<BigFraction>> data = new ArrayList<>();
    public List<BigFraction> rhs = new ArrayList<>();
    public List<BigFraction> objectiveFunctionRow = new ArrayList<>();
    public BigFraction objectiveValue;

    //Metadata - not transformed to out DTOS
    public boolean containsArtificialVariables;
    //Computed dynamically, since not being used in all algorithms
    private final Map<String, Integer> columnVariableIndexes = new HashMap<>();

    public static SimplexTable fromMpsData(MpsDataTransformedBounds mpsData) {
        SimplexTable result = new SimplexTable();
        Optional<String> objectiveRowNameOpt = mpsData.rows.entrySet().stream().filter((rowEntry) -> rowEntry.getValue() == RowType.N).map(entry -> entry.getKey()).findFirst();
        if (objectiveRowNameOpt.isEmpty()) {
            throw new SimplexTableTransformationError("No objective row definition found!");
        }
        String objectiveRowName = objectiveRowNameOpt.get();

        //Each row has either zero or its value of slack variable
        List<BigFraction> slackVariables = new ArrayList<>(mpsData.rows.size());
        //Each row has either zero or its value of artificial variable
        List<BigFraction> artificialVariables = new ArrayList<>(mpsData.rows.size());

        //We fill in data from the constraint rows
        //First we create the explicit values from MPS data model and we note the artificial variables
        int rowIndex = 0;
        for (Map.Entry<String, RowType> row: mpsData.rows.entrySet()) {
            //Skip the objective row
            if (row.getValue().equals(RowType.N)) {
                continue;
            }

            List<BigFraction> rowValues = new ArrayList<>();

            for (Map.Entry<String, Map<String, BigFraction>> variableValues: mpsData.columns.entrySet()) {
                BigFraction variableValueForRow = variableValues.getValue().get(row.getKey());
                if (variableValueForRow == null) {
                    variableValueForRow = BigFraction.ZERO;
                }
                rowValues.add(variableValueForRow);
            }

            if (row.getValue().equals(RowType.L)) {
                slackVariables.add(BigFraction.ONE);
                artificialVariables.add(BigFraction.ZERO);
            } else if (row.getValue().equals(RowType.G)) {
                slackVariables.add(BigFraction.ONE.negate());
                artificialVariables.add(BigFraction.ONE);
            } else if  (row.getValue().equals(RowType.E)) {
                artificialVariables.add(BigFraction.ONE);
                slackVariables.add(BigFraction.ZERO);
            } else {
                throw new IllegalStateException("Target row was included in the data creation part in the simplex table.");
            }
            rowIndex++;
            result.data.add(rowValues);
        }

        //Fill in the objective row (without slack variables etc.)
        for (Map.Entry<String, Map<String, BigFraction>> variableValues: mpsData.columns.entrySet()) {
            BigFraction variableValueForRow = variableValues.getValue().get(objectiveRowName);
            if (variableValueForRow == null) {
                variableValueForRow = BigFraction.ZERO;
            }
            result.objectiveFunctionRow.add(variableValueForRow.negate());
        }

        //Fill in the RHS
        for (String rowName: mpsData.rows.keySet()) {
            if (rowName.equals(objectiveRowName)) {
                continue;
            }
            BigFraction rhsValueForRow = mpsData.rhs.get(rowName);
            if (rhsValueForRow == null) {
                rhsValueForRow = BigFraction.ZERO;
            }
            result.rhs.add(rhsValueForRow);
        }

        //Fill in the variable names
        result.variables.addAll(mpsData.columns.keySet());

        result.fillInSlackSurplusAndArtificialVariables(slackVariables, artificialVariables);

        result.objectiveValue = BigFraction.ZERO;
        return result;
    }

    /**
     * Fill in slack and surplus variables and also fill in the initial base variables
     * @param slackVariables
     * @param artificialVariables
     */
    private void fillInSlackSurplusAndArtificialVariables(List<BigFraction> slackVariables, List<BigFraction> artificialVariables) {
        //Add slack and surplus and artificial variable data to simplex table

        for (int ri = 0; ri < this.data.size(); ri++) {
            int slackIndex = 1;
            int artificialIndex = 1;
            for (int si = 0; si < slackVariables.size(); si++) {
                if (si == ri && slackVariables.get(si).equals(BigFraction.ONE)) {
                    this.data.get(ri).add(slackVariables.get(si));
                    this.baseVariables.add("S_" + slackIndex);
                } else if (si == ri && !slackVariables.get(si).equals(BigFraction.ZERO)) {
                    this.data.get(ri).add(slackVariables.get(si));
                } else if (!slackVariables.get(si).equals(BigFraction.ZERO)){
                    this.data.get(ri).add(BigFraction.ZERO);
                }
                if (!slackVariables.get(si).equals(BigFraction.ZERO)) {
                    slackIndex++;
                }
            }

            for (int ai = 0; ai < artificialVariables.size(); ai++) {
                if (ai == ri && artificialVariables.get(ai).equals(BigFraction.ONE)) {
                    this.data.get(ri).add(artificialVariables.get(ai));
                    this.baseVariables.add("A_" + artificialIndex);
                } else if (!artificialVariables.get(ai).equals(BigFraction.ZERO)){
                    this.data.get(ri).add(BigFraction.ZERO);
                }

                if (!artificialVariables.get(ai).equals(BigFraction.ZERO)) {
                    artificialIndex++;
                }
            }
        }

        int slackIndex = 1;
        //Add slack and surplus and artificial variable names to variable names
        for (int si = 0; si < slackVariables.size(); si++) {
            if (!slackVariables.get(si).equals(BigFraction.ZERO)) {
                this.variables.add("S_" + slackIndex);
                this.objectiveFunctionRow.add(BigFraction.ZERO);
                slackIndex ++;
            }
        }
        int artificialIndex = 1;
        for (int ai = 0; ai < artificialVariables.size(); ai++) {
            if (!artificialVariables.get(ai).equals(BigFraction.ZERO)) {
                this.variables.add("A_" + artificialIndex);
                this.objectiveFunctionRow.add(BigFraction.ZERO);
                artificialIndex ++;
                this.containsArtificialVariables = true;
            }
        }
    }

    public SimplexTable() {
    }

    // Copy constructor
    public SimplexTable(SimplexTable other) {

        // Deep copy simple lists
        this.variables = new ArrayList<>(other.variables);
        this.baseVariables = new ArrayList<>(other.baseVariables);

        // Deep copy 2D list
        this.data = new ArrayList<>();
        for (List<BigFraction> row : other.data) {
            this.data.add(new ArrayList<>(row));
        }

        // Deep copy remaining lists
        this.rhs = new ArrayList<>(other.rhs);
        this.objectiveFunctionRow = new ArrayList<>(other.objectiveFunctionRow);

        // Copy value
        this.objectiveValue = other.objectiveValue;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        SimplexTable that = (SimplexTable) o;
        return Objects.equals(variables, that.variables) && Objects.equals(baseVariables, that.baseVariables) && Objects.equals(data, that.data) && Objects.equals(rhs, that.rhs) && Objects.equals(objectiveFunctionRow, that.objectiveFunctionRow) && Objects.equals(objectiveValue, that.objectiveValue);
    }

    @Override
    public int hashCode() {
        return Objects.hash(variables, baseVariables, data, rhs, objectiveFunctionRow, objectiveValue);
    }

    public void print() {
        int colWidth = 12;
        int numVars = variables.size();
        int numRows = baseVariables.size();

        // Helper to center a string in a field
        java.util.function.BiFunction<String, Integer, String> center = (s, width) -> {
            if (s.length() >= width) return s.substring(0, width);
            int pad = width - s.length();
            int left = pad / 2;
            int right = pad - left;
            return " ".repeat(left) + s + " ".repeat(right);
        };

        String horizontalLine = "+" + ("-".repeat(colWidth) + "+").repeat(numVars + 2);

        // Header row: "Basis" | variables... | "RHS"
        System.out.println(horizontalLine);
        StringBuilder header = new StringBuilder("|");
        header.append(center.apply("Basis", colWidth)).append("|");
        for (String var : variables) {
            header.append(center.apply(var, colWidth)).append("|");
        }
        header.append(center.apply("RHS", colWidth)).append("|");
        System.out.println(header);
        System.out.println(horizontalLine);

        // Data rows
        for (int i = 0; i < numRows; i++) {
            StringBuilder row = new StringBuilder("|");
            row.append(center.apply(baseVariables.get(i), colWidth)).append("|");
            for (int j = 0; j < numVars; j++) {
                String val = (i < data.size() && j < data.get(i).size())
                        ? data.get(i).get(j).toString() : "0";
                row.append(center.apply(val, colWidth)).append("|");
            }
            String rhsVal = (i < rhs.size()) ? rhs.get(i).toString() : "0";
            row.append(center.apply(rhsVal, colWidth)).append("|");
            System.out.println(row);
            System.out.println(horizontalLine);
        }

        // Objective function row
        StringBuilder objRow = new StringBuilder("|");
        objRow.append(center.apply("z", colWidth)).append("|");
        for (int j = 0; j < numVars; j++) {
            String val = (j < objectiveFunctionRow.size())
                    ? objectiveFunctionRow.get(j).toString() : "0";
            objRow.append(center.apply(val, colWidth)).append("|");
        }
        String objVal = (objectiveValue != null) ? objectiveValue.toString() : "0";
        objRow.append(center.apply(objVal, colWidth)).append("|");
        System.out.println(objRow);
        System.out.println(horizontalLine);
    }

    /**
     * Get column index of variable by name
     * Cached dynamically, since it is required only by some algorithms
     * @param variableName
     * @return
     */
    public Optional<Integer> getVariableColumnIndex(String variableName) {
        if (this.columnVariableIndexes.containsKey(variableName)) {
            return Optional.of(this.columnVariableIndexes.get(variableName));
        }
        int result = this.variables.indexOf(variableName);
        if (result == -1) {
            return Optional.empty();
        } else {
            this.columnVariableIndexes.put(variableName, result);
            return Optional.of(result);
        }
    }

    /**
     * Return RHS of the current simplex table in a matrix form. Instead of [1,2,3]
     * [[1],[2],[3]]
     * @return
     */
    public List<List<BigFraction>> getRhsInMatrixForm() {
        List<List<BigFraction>> result = new ArrayList<>(this.rhs.size());
        this.rhs.forEach(rhsValue -> {
            result.add(List.of(rhsValue));
        });
        return result;
    }

    /**
     * Return column from matrix data in matrix format [[1], [2], [3]]
     * @param columnIndex
     * @return
     */
    public List<List<BigFraction>> getDataColumnInMatrixForm(int columnIndex) {
        if (columnIndex >= this.data.size()) {
            throw new IndexOutOfBoundsException("Cannot obtain data column " + columnIndex + " from matrix with " + this.data.size() + " columns.");
        }

        List<List<BigFraction>> result = new ArrayList<>(this.rhs.size());
        for (int rowIndex = 0; rowIndex < this.rhs.size(); rowIndex++) {
            result.add(List.of(this.data.get(rowIndex).get(columnIndex)));
        }
        return result;
    }
}
