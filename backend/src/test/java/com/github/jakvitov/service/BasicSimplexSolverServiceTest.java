package com.github.jakvitov.service;

import com.github.jakvitov.dto.solver.basic.SimplexTableRowsNormalizationDto;
import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Optional;

@MicronautTest
public class BasicSimplexSolverServiceTest {

    @Inject
    private BasicSimplexSolverService basicSimplexSolverService;

    @Test
    public void convert_objective_row_to_minimalization_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "S_1", "A_1");
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(BigFraction.ONE, BigFraction.TWO, BigFraction.ZERO, BigFraction.ZERO));

        basicSimplexSolverService.convertObjectiveRowForMinimalization(simplexTable);

        List<BigFraction> correctTransformed = List.of(BigFraction.ONE.negate(), BigFraction.TWO.negate(), BigFraction.ZERO, BigFraction.ZERO);
        assert simplexTable.objectiveFunctionRow.equals(correctTransformed);
    }

    @Test
    public void is_unbounded_succeeds_for_unbounded_solution() {
        SimplexTable simplexTable = new SimplexTable();
        List<List<BigFraction>> data = new ArrayList<>();
        //negative only entering variable column
        for (int i = 0; i < 5; i ++) {
            data.add(List.of(new BigFraction(i).negate()));
            simplexTable.baseVariables.add("X" + i);
        }
        simplexTable.data = data;
        assert basicSimplexSolverService.isUnbounded(simplexTable, 0);
    }

    @Test
    public void is_unbounded_succeeds_for_non_unbounded_solution() {
        SimplexTable simplexTable = new SimplexTable();
        List<List<BigFraction>> data = new ArrayList<>();
        //negative only entering variable column
        for (int i = 0; i < 5; i ++) {
            data.add(List.of(new BigFraction(i)));
            simplexTable.baseVariables.add("X" + i);
        }
        simplexTable.data = data;
        assert !basicSimplexSolverService.isUnbounded(simplexTable, 0);
    }

    @Test
    public void get_solution_variable_values_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        for (int i = 0; i < 5; i ++) {
            simplexTable.baseVariables.add("X" + i);
            simplexTable.rhs.add(new BigFraction(i));
        }

        Map<String, BigFraction> res = basicSimplexSolverService.getSolutionVariableValues(simplexTable);

        for (int i = 0; i < 5; i ++) {
            assert res.get("X" + i).equals(new BigFraction(i));
        }
    }

    @Test
    public void switch_leaving_entering_variables_succeeds() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.variables = List.of("X1", "X2", "S_1", "A_1");
        simplexTable.baseVariables = new ArrayList<>(List.of("X1", "S_1"));
        basicSimplexSolverService.switchLeavingEnteringVariables(1, 1, simplexTable);

        assert simplexTable.baseVariables.getFirst().equals("X1");
        assert simplexTable.baseVariables.get(1).equals("X2");
    }

    @Test
    public void normalise_rows_by_leaving_variable_row_succeeds() {
        SimplexTable simplexTable = new SimplexTable();

        for (int i = 2; i > 0; i --) {
            simplexTable.data.add(new ArrayList<>(List.of(new BigFraction(i), new BigFraction(i), new BigFraction(i))));
        }
        simplexTable.rhs = new ArrayList<>(List.of(new BigFraction(2), new BigFraction(1)));
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(new BigFraction(3), new BigFraction(3), new BigFraction(3)));
        simplexTable.objectiveValue = new BigFraction(3);

        SimplexTableRowsNormalizationDto normalizationDto = basicSimplexSolverService.normaliseRowsByLeavingVariableRow(1, 0, simplexTable);
        //Check simplex table normalised
        simplexTable.data.getFirst().forEach((num) -> {
            assert num.equals(BigFraction.ZERO);
        });
        simplexTable.data.get(1).forEach((num) -> {
            assert num.equals(BigFraction.ONE);
        });
        simplexTable.objectiveFunctionRow.forEach((num) -> {
            assert num.equals(BigFraction.ZERO);
        });

        assert simplexTable.rhs.getFirst().equals(BigFraction.ZERO);
        assert simplexTable.rhs.get(1).equals(BigFraction.ONE);
        simplexTable.objectiveValue.equals(BigFraction.ZERO);

        //Check result normalization dto
        normalizationDto.getSimplexTableDto().getData().getFirst().forEach((num) -> {
            assert num.equals(BigFraction.ZERO);
        });
        normalizationDto.getSimplexTableDto().getData().get(1).forEach((num) -> {
            assert num.equals(BigFraction.ONE);
        });
        normalizationDto.getSimplexTableDto().getObjectiveFunctionRow().forEach((num) -> {
            assert num.equals(BigFraction.ZERO);
        });
        assert normalizationDto.getSimplexTableDto().getRhs().getFirst().equals(BigFraction.ZERO);
        assert normalizationDto.getSimplexTableDto().getRhs().get(1).equals(BigFraction.ONE);
        assert normalizationDto.getSimplexTableDto().getObjectiveValue().equals(BigFraction.ZERO);

        assert normalizationDto.getCoefficients().size() == 1;
        assert normalizationDto.getCoefficients().get(0).equals(BigFraction.TWO.negate());
        assert normalizationDto.getObjectiveRowCoefficient().equals(new BigFraction(3).negate());
        assert normalizationDto.getLeavingVariableIndex() == 1;
    }

    @Test
    public void normalise_leaving_variable_row_succeeds() {
        SimplexTable simplexTable = new SimplexTable();

        for (int i = 2; i > 0; i --) {
            simplexTable.data.add(new ArrayList<>(List.of(new BigFraction(i), new BigFraction(i), new BigFraction(i))));
        }
        simplexTable.rhs = new ArrayList<>(List.of(new BigFraction(2), new BigFraction(1)));
        simplexTable.objectiveFunctionRow = new ArrayList<>(List.of(new BigFraction(3), new BigFraction(3), new BigFraction(3)));
        simplexTable.objectiveValue = new BigFraction(3);

        BigFraction coefficient = basicSimplexSolverService.normaliseLeavingVariableRow(0, 0, simplexTable);

        assert coefficient.equals(new BigFraction(1, 2));

        simplexTable.data.getFirst().forEach((num) -> {
            assert num.equals(BigFraction.ONE);
        });
        assert simplexTable.rhs.getFirst().equals(BigFraction.ONE);
    }

    @Test
    public void get_leaving_variable_index_succeeds_for_valid_t_vec() {
        List<Optional<BigFraction>> tVec = List.of(Optional.empty(), Optional.of(BigFraction.ONE), Optional.of(BigFraction.ONE.negate()));
        int index = basicSimplexSolverService.getLeavingVariableIndex(tVec);
        assert index == 1;
    }

    @Test
    public void get_leaving_variable_index_fails_for_invalid_t_vec() {
        List<Optional<BigFraction>> tVec = List.of(Optional.empty(), Optional.of(BigFraction.ONE.negate()), Optional.of(BigFraction.ONE.negate()));
        try {
            int index = basicSimplexSolverService.getLeavingVariableIndex(tVec);
        } catch (IllegalStateException e) {
            return;
        }
        assert false;
    }

    @Test
    public void compute_t_vector_succeeds() {
        SimplexTable simplexTable = new SimplexTable();

        simplexTable.data.add(List.of(BigFraction.ONE.negate()));
        simplexTable.rhs.add(BigFraction.ONE);

        for (int i = 1; i < 5; i ++) {
            simplexTable.data.add(List.of(new BigFraction(i)));
            simplexTable.rhs.add(new BigFraction(i));
        }

        List<Optional<BigFraction>> tVec = basicSimplexSolverService.computeTVector(0, simplexTable);
        assert tVec.getFirst().equals(Optional.empty());
        for (int i = 1; i < tVec.size(); i ++) {
            assert tVec.get(i).get().equals(BigFraction.ONE);
        }
    }

    @Test
    public void get_entering_variable_index_succeeds_for_suboptimal_simplex_table() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE.negate(), BigFraction.ONE);
        assert basicSimplexSolverService.getEnteringVariableIndex(simplexTable) == 0;
    }

    @Test
    public void get_entering_variable_index_fails_for_optimal_simplex_table() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE, BigFraction.ONE);
        try {
            basicSimplexSolverService.getEnteringVariableIndex(simplexTable);
        } catch (IllegalStateException e) {
            return;
        }
        assert false;
    }

    @Test
    public void is_simplex_table_solved_succeeds_for_solved_simplex_table() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE, BigFraction.ONE);
        assert basicSimplexSolverService.isSimplexTableSolved(simplexTable);
    }

    @Test
    public void is_simplex_table_solved_succeeds_for_unsolved_simplex_table() {
        SimplexTable simplexTable = new SimplexTable();
        simplexTable.objectiveFunctionRow = List.of(BigFraction.ONE.negate(), BigFraction.ONE);
        assert !basicSimplexSolverService.isSimplexTableSolved(simplexTable);
    }
}
