package com.github.jakvitov.service;

import com.github.jakvitov.dto.solver.revised.NonBasicVariableCurrentReducedCostCalculationDto;
import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;

@MicronautTest
public class RevisedSimplexSolverServiceTest {

    @Inject
    private RevisedSimplexSolverService revisedSimplexSolverService;

    @Test
    public void get_result_variable_values_succeeds() {
        List<List<BigFraction>> xB = List.of(List.of(BigFraction.ONE), List.of(BigFraction.TWO));
        List<String> currentBasis = List.of("X_1", "X_2");

        Map<String, BigFraction> res = revisedSimplexSolverService.getResultVariableValues(xB, currentBasis);
        assert res.size() == 2;
        assert res.get("X_1").equals(BigFraction.ONE);
        assert res.get("X_2").equals(BigFraction.TWO);
    }

    @Test
    public void is_unbounded_succeeds_for_unbounded_solution() {
        List<List<BigFraction>> d = List.of(List.of(BigFraction.ZERO), List.of(BigFraction.TWO.negate()));
        assert revisedSimplexSolverService.isUnbounded(d);
    }

    @Test
    public void is_unbounded_succeeds_for_non_unbounded_solution() {
        List<List<BigFraction>> d = List.of(List.of(BigFraction.ZERO), List.of(BigFraction.TWO));
        assert !revisedSimplexSolverService.isUnbounded(d);
    }

    @Test
    public void compute_ratio_vector_succeeds() {
        List<List<BigFraction>> d = List.of(List.of(BigFraction.ZERO), List.of(BigFraction.TWO));
        List<List<BigFraction>> xB = List.of(List.of(BigFraction.ONE.negate()), List.of(BigFraction.TWO.negate()));

        List<Optional<BigFraction>> tVec = revisedSimplexSolverService.computeRatioVector(d, xB);
        assert tVec.size() == 2;
        assert tVec.getFirst().isEmpty();
        assert tVec.get(1).get().equals(BigFraction.ONE.negate());
    }

    @Test
    public void get_entering_variable_index_succeeds_for_suboptimal_input() {
        Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = new HashMap<>();
        nonBasicVariablesCurrentReducedCosts.put(0, BigFraction.ZERO);
        nonBasicVariablesCurrentReducedCosts.put(1, BigFraction.ONE);
        nonBasicVariablesCurrentReducedCosts.put(2, BigFraction.ONE.negate());

        Optional<Integer> res = revisedSimplexSolverService.getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);
        assert res.isPresent();
        assert res.get() == 2;
    }

    @Test
    public void get_entering_variable_index_succeeds_for_optimal_input() {
        Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts = new HashMap<>();
        nonBasicVariablesCurrentReducedCosts.put(0, BigFraction.ZERO);
        nonBasicVariablesCurrentReducedCosts.put(1, BigFraction.ONE);
        nonBasicVariablesCurrentReducedCosts.put(2, BigFraction.ONE);

        Optional<Integer> res = revisedSimplexSolverService.getEnteringVariableIndex(nonBasicVariablesCurrentReducedCosts);
        assert res.isEmpty();
    }

    @Test
    public void compute_non_basic_variables_current_reduced_costs_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.baseVariables = List.of("S_1", "S_2");
        simplexTable.variables = List.of("x_1", "x_2", "S_1", "S_2");
        simplexTable.rhs = List.of(BigFraction.ONE, BigFraction.ONE);
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE.negate(), BigFraction.ONE.negate(), BigFraction.ZERO, BigFraction.ZERO);
        simplexTable.objectiveValue = BigFraction.ZERO;
        simplexTable.data = List.of(List.of(BigFraction.ONE, BigFraction.ZERO, BigFraction.ONE, BigFraction.ZERO),
                List.of(BigFraction.ZERO.negate(), BigFraction.ONE.negate(), BigFraction.ZERO, BigFraction.ONE));

        List<String> currentBasis = List.of("S_1", "S_2");

        List<List<BigFraction>> yT = List.of(List.of(BigFraction.ZERO, BigFraction.ZERO));

        Map<Integer, NonBasicVariableCurrentReducedCostCalculationDto> res = revisedSimplexSolverService.computeNonBasicVariablesCurrentReducedCosts(simplexTable, currentBasis, yT);

        assert res.size() == 2;
        assert res.get(0).getCJ().equals(BigFraction.ONE.negate());
        assert res.get(0).getNonBasicVariableReducedCost().equals(BigFraction.ZERO);
        assert res.get(0).getAJ().getFirst().getFirst().equals(BigFraction.ONE);
        assert res.get(0).getAJ().get(1).getFirst().equals(BigFraction.ZERO);

        assert res.get(1).getCJ().equals(BigFraction.ONE.negate());
        assert res.get(1).getNonBasicVariableReducedCost().equals(BigFraction.ZERO);
        assert res.get(1).getAJ().getFirst().getFirst().equals(BigFraction.ZERO);
        assert res.get(1).getAJ().get(1).getFirst().equals(BigFraction.ONE.negate());
    }

    @Test
    public void get_non_basic_variables_column_indexes_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "X3");
        List<String> currentBasis = List.of("X1", "X2");

        List<Integer> res = revisedSimplexSolverService.getNonBasicVariablesColumnIndexes(simplexTable, currentBasis);
        assert res.size() == 1;
        assert res.getFirst() == 2;
    }

    @Test
    public void get_non_basic_variables_column_indexes_fails_for_unknown_basis_variable() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "X3");
        List<String> currentBasis = List.of("X1", "92972772");

        try {
            List<Integer> res = revisedSimplexSolverService.getNonBasicVariablesColumnIndexes(simplexTable, currentBasis);
        } catch (IllegalStateException e) {
            return;
        }
        assert false;
    }

    @Test
    public void get_reduced_costs_from_simplex_table_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "X3");
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE, BigFraction.TWO, BigFraction.MINUS_ONE);

        List<String> currentBase = List.of("X1", "X2");
        List<List<BigFraction>> res = revisedSimplexSolverService.getReducedCostsFromSimplexTable(simplexTable, currentBase);

        assert res.size() == 1;
        assert res.getFirst().getFirst().equals(BigFraction.ONE);
        assert res.getFirst().get(1).equals(BigFraction.TWO);
    }

    @Test
    public void get_reduced_costs_from_simplex_table_fails_for_unknown_basis_variable() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "X3");
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE, BigFraction.TWO, BigFraction.MINUS_ONE);

        List<String> currentBase = List.of("X1", "DKFJKDLJFKLDJF");
        try {
            List<List<BigFraction>> res = revisedSimplexSolverService.getReducedCostsFromSimplexTable(simplexTable, currentBase);
        } catch (IllegalStateException e) {
            return;
        }
        assert false;
    }

    @Test
    public void get_basis_matrix_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.baseVariables = List.of("S_1", "S_2");
        simplexTable.variables = List.of("x_1", "x_2", "S_1", "S_2");
        simplexTable.rhs = List.of(BigFraction.ONE, BigFraction.ONE);
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE.negate(), BigFraction.ONE.negate(), BigFraction.ZERO, BigFraction.ZERO);
        simplexTable.objectiveValue = BigFraction.ZERO;
        simplexTable.data = List.of(List.of(BigFraction.ONE, BigFraction.ZERO, BigFraction.ONE, BigFraction.ZERO),
                List.of(BigFraction.ZERO.negate(), BigFraction.ONE.negate(), BigFraction.ZERO, BigFraction.ONE));

        List<String> currentBase = List.of("x_1", "x_2");
        List<List<BigFraction>> res = revisedSimplexSolverService.getBasisMatrix(simplexTable, currentBase);

        assert res.size() == 2;
        assert res.getFirst().equals(List.of(BigFraction.ONE, BigFraction.ZERO));
        assert res.get(1).equals(List.of(BigFraction.ZERO, BigFraction.ONE.negate()));

    }



}
