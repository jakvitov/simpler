package com.github.jakvitov.dto;

import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.core.annotation.Introspected;
import io.micronaut.serde.annotation.Serdeable;
import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.List;

@Serdeable
@Introspected
public class SimplexTableDto {

    private List<String> variables;
    private List<String> baseVariables;

    private List<List<BigFraction>> data;
    private List<BigFraction> rhs;
    private List<BigFraction> objectiveFunctionRow;
    private BigFraction objectiveValue;

    // Copy constructor
    public SimplexTableDto(SimplexTable other) {

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

    public SimplexTableDto() {
    }

    public List<String> getVariables() {
        return variables;
    }

    public void setVariables(List<String> variables) {
        this.variables = variables;
    }

    public List<String> getBaseVariables() {
        return baseVariables;
    }

    public void setBaseVariables(List<String> baseVariables) {
        this.baseVariables = baseVariables;
    }

    public List<List<BigFraction>> getData() {
        return data;
    }

    public void setData(List<List<BigFraction>> data) {
        this.data = data;
    }

    public List<BigFraction> getRhs() {
        return rhs;
    }

    public void setRhs(List<BigFraction> rhs) {
        this.rhs = rhs;
    }

    public List<BigFraction> getObjectiveFunctionRow() {
        return objectiveFunctionRow;
    }

    public void setObjectiveFunctionRow(List<BigFraction> objectiveFunctionRow) {
        this.objectiveFunctionRow = objectiveFunctionRow;
    }

    public BigFraction getObjectiveValue() {
        return objectiveValue;
    }

    public void setObjectiveValue(BigFraction objectiveValue) {
        this.objectiveValue = objectiveValue;
    }
}
