package com.github.jakvitov.service;

import com.github.jakvitov.dto.solver.twophase.TwoPhaseSimplexObjectiveRowNormalizationDto;
import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

@MicronautTest
public class TwoPhaseSimplexSolverServiceTest {

    @Inject
    private TwoPhaseSimplexSolverService twoPhaseSimplexSolverService;

    @Test
    public void compute_artificial_objective_function_row_value_succeeds_for_artificial_variables_present() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.baseVariables = List.of("X1", "S1", "A_1", "A_2");
        simplexTable.rhs = List.of(BigFraction.ONE, BigFraction.ONE, new BigFraction(10), new BigFraction(10));
        assert twoPhaseSimplexSolverService.computeArtificialObjectiveFunctionRowValue(simplexTable).equals(new BigFraction(20));
    }

    @Test
    public void compute_artificial_objective_function_row_value_succeeds_for_artificial_variables_not_present() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.baseVariables = List.of("X1", "S1", "D_1", "Z_2");
        simplexTable.rhs = List.of(BigFraction.ONE, BigFraction.ONE, new BigFraction(10), new BigFraction(10));
        assert twoPhaseSimplexSolverService.computeArtificialObjectiveFunctionRowValue(simplexTable).equals(BigFraction.ZERO);
    }

    @Test
    public void adjust_objective_row_to_current_basis_phase_two_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.baseVariables = List.of("X1", "X2");
        simplexTable.variables = List.of("X1", "X2", "S1");

        simplexTable.data = List.of(List.of(BigFraction.ONE, BigFraction.ZERO, BigFraction.TWO),
                List.of(BigFraction.ZERO, BigFraction.ONE, BigFraction.THREE_FIFTHS));
        simplexTable.rhs = List.of(BigFraction.ONE, BigFraction.ONE);
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.ONE, BigFraction.TWO, BigFraction.ZERO));
        simplexTable.objectiveValue = BigFraction.ZERO;

        TwoPhaseSimplexObjectiveRowNormalizationDto twoPhaseSimplexObjectiveRowNormalizationDto = twoPhaseSimplexSolverService.adjustObjectiveRowToCurrentBasisPhaseTwo(simplexTable);

        assert simplexTable.objectiveFunctionRow.getFirst().equals(BigFraction.ZERO);
        assert simplexTable.objectiveFunctionRow.get(1).equals(BigFraction.ZERO);
        assert simplexTable.objectiveValue.equals(new BigFraction(3).negate());

        assert twoPhaseSimplexObjectiveRowNormalizationDto.getCoefficients().get(0).equals(BigFraction.ONE.negate());
        assert twoPhaseSimplexObjectiveRowNormalizationDto.getCoefficients().get(1).equals(BigFraction.TWO.negate());
    }

    @Test
    public void get_leaving_variable_index_for_phase_one_succeeds_for_valid_t_vec() {
        List<Optional<BigFraction>> tVec = List.of(Optional.empty(), Optional.of(BigFraction.ONE.negate()));
        assert twoPhaseSimplexSolverService.getLeavingVariableIndexForPhaseOne(tVec) == 1;
    }

    @Test
    public void get_leaving_variable_index_for_phase_one_fails_for_invalid_t_vec() {
        List<Optional<BigFraction>> tVec = List.of(Optional.empty(), Optional.empty());
        try {
            twoPhaseSimplexSolverService.getLeavingVariableIndexForPhaseOne(tVec);
        } catch (IllegalStateException e) {
            return;
        }
        assert false;
    }

    @Test
    public void remove_artificial_variables_after_phase_one_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = new ArrayList<>(List.of("S_1", "A_1"));

        simplexTable.data.add(new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE)));
        simplexTable.data.add(new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE)));

        simplexTable.baseVariables.add("S_1");
        simplexTable.rhs.add(BigFraction.ZERO);
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE));
        simplexTable.objectiveValue = BigFraction.ZERO;

        List<BigFraction> originalObjectiveRow = new ArrayList<>(List.of(BigFraction.TWO, BigFraction.ONE));
        twoPhaseSimplexSolverService.removeArtificialVariablesAfterPhaseOne(simplexTable, originalObjectiveRow);

        assert simplexTable.variables.size() == 1;
        assert simplexTable.baseVariables.size() == 1;
        assert simplexTable.objectiveFunctionRow.size() == 1;
        assert originalObjectiveRow.size() == 1;
    }

    @Test
    public void remove_artificial_variables_after_phase_one_does_nothing_without_artificial_variables() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = new ArrayList<>(List.of("S_1", "D_1"));

        simplexTable.data.add(new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE)));
        simplexTable.data.add(new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE)));

        simplexTable.baseVariables.add("S_1");
        simplexTable.rhs.add(BigFraction.ZERO);
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.ZERO, BigFraction.ONE));
        simplexTable.objectiveValue = BigFraction.ZERO;

        List<BigFraction> originalObjectiveRow = new ArrayList<>(List.of(BigFraction.TWO, BigFraction.ONE));
        twoPhaseSimplexSolverService.removeArtificialVariablesAfterPhaseOne(simplexTable, originalObjectiveRow);

        assert simplexTable.variables.size() == 2;
        assert simplexTable.baseVariables.size() == 1;
        assert simplexTable.objectiveFunctionRow.size() == 2;
        assert originalObjectiveRow.size() == 2;
    }

    @Test
    public void setup_objective_row_before_phase_one_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "S_1", "A_1");
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.ONE, BigFraction.TWO, new BigFraction(3)));
        twoPhaseSimplexSolverService.setupObjectiveRowBeforePhaseOne(simplexTable);

        assert simplexTable.objectiveFunctionRow.getFirst().equals(BigFraction.ZERO);
        assert simplexTable.objectiveFunctionRow.get(1).equals(BigFraction.ZERO);
        assert simplexTable.objectiveFunctionRow.get(2).equals(BigFraction.ONE);
    }

    @Test
    public void normalize_artificial_variables_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "A_1");
        simplexTable.baseVariables = List.of("A_1");
        simplexTable.data = List.of(List.of(BigFraction.ONE, BigFraction.ONE));
        simplexTable.rhs = List.of(BigFraction.ONE);
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.TWO, BigFraction.ONE));
        simplexTable.objectiveValue = BigFraction.ZERO;

        TwoPhaseSimplexObjectiveRowNormalizationDto twoPhaseSimplexObjectiveRowNormalizationDto = twoPhaseSimplexSolverService.normalizeArtificialVariables(simplexTable);

        assert twoPhaseSimplexObjectiveRowNormalizationDto.getSimplexTableDto().getObjectiveFunctionRow().equals(List.of(BigFraction.ONE, BigFraction.ZERO));
        //Assert actual value of artificial objective function is here instead of computed value
        assert twoPhaseSimplexObjectiveRowNormalizationDto.getSimplexTableDto().getObjectiveValue().equals(BigFraction.ONE);
        assert twoPhaseSimplexObjectiveRowNormalizationDto.getCoefficients().size() == 1;
        assert twoPhaseSimplexObjectiveRowNormalizationDto.getCoefficients().get(0).equals(BigFraction.ONE.negate());
    }
}
