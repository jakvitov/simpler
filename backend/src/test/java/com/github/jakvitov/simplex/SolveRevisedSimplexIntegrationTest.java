package com.github.jakvitov.simplex;

import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.config.SolverConfigurationDto;
import com.github.jakvitov.dto.solver.revised.SolveLpRevisedSimlexResponseDto;
import com.github.jakvitov.dto.solver.twophase.SolveLpTwoPhaseSimplexResponseDto;
import com.github.jakvitov.service.RevisedSimplexSolverService;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

@MicronautTest
public class SolveRevisedSimplexIntegrationTest {

    @Inject
    private RevisedSimplexSolverService revisedSimplexSolverService;

    @Test
    public void solve_revised_simplex_minimalization_testmin_succeeds() {
        String input = """
                NAME          TESTMIN
                ROWS
                 N  COST
                 L  C1
                 L  C2
                 L  C3
                COLUMNS
                    X1        COST      3
                    X1        C1        1
                    X1        C2        2
                    X1        C3        1
                    X2        COST      2
                    X2        C1        1
                    X2        C2        1
                    X2        C3        3
                RHS
                    RHS1      C1        4
                    RHS1      C2        5
                    RHS1      C3        7
                ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MIN, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(BigFraction.ZERO);
        assert response.getResultVariableValues().get("S_1").equals(new BigFraction(4));
        assert response.getResultVariableValues().get("S_2").equals(new BigFraction(5));
        assert response.getResultVariableValues().get("S_3").equals(new BigFraction(7));
    }

    @Test
    public void solve_revised_simplex_maximalization_simplecase_succeeds() {
        String input = """
                #Simple LP with x1=2, x2=2, z=4
                #Two iterations solution
                NAME          SIMPLELP
                ROWS
                 N  OBJ
                 L  C1
                 L  C2
                COLUMNS
                    X1        OBJ     1
                    X1        C1      1
                    X1        C2      0
                    X2        OBJ     1
                    X2        C1      0
                    X2        C2      1
                RHS
                    RHS1      C1      2
                    RHS1      C2      2
                ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(new BigFraction(4));
        assert response.getResultVariableValues().get("X1").equals(new BigFraction(2));
        assert response.getResultVariableValues().get("X2").equals(new BigFraction(2));
    }

    @Test
    public void solve_revised_simplex_unbounded_succeeds() {
        String input = """
                #Unbounded case reached in first iteration
                NAME          UNBOUNDED
                ROWS
                 N  OBJ
                 L  C1
                COLUMNS
                    X1        OBJ     1
                    X1        C1     -1
                    X2        OBJ     0
                    X2        C1      1
                RHS
                    RHS1      C1      1
                ENDATA
               """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.UNBOUNDED);
        assert response.getSolutionObjectiveFunctionValue() == null;
        assert response.getResultVariableValues() == null;
    }

    @Test
    public void solve_revised_simplex_maximalization_large_succeeds() {
        String input = """
                NAME          LARGE
                ROWS
                 N  OBJ
                 L  C1
                 L  C2
                 L  C3
                COLUMNS
                    X1        OBJ       2
                    X1        C1        1
                    X1        C2        1
                    X1        C3       -1
                    X2        OBJ       1
                    X2        C1        1
                    X2        C2       -1
                    X2        C3        1
                    X3        OBJ       1
                    X3        C1        1
                    X3        C2       -1
                    X3        C3        1
                RHS
                    RHS1      C1       10
                    RHS1      C2        5
                    RHS1      C3      100
                ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(new BigFraction(35, 2));
        assert response.getResultVariableValues().get("X1").equals(new BigFraction(15, 2));
        assert response.getResultVariableValues().get("X2").equals(new BigFraction(5, 2));
        assert response.getResultVariableValues().get("S_3").equals(new BigFraction(105));
    }

    @Test
    public void solve_revised_simplex_max_iterations_integration_test() {
        String input = """
                NAME          LARGE
                ROWS
                 N  OBJ
                 L  C1
                 L  C2
                 L  C3
                COLUMNS
                    X1        OBJ       2
                    X1        C1        1
                    X1        C2        1
                    X1        C3       -1
                    X2        OBJ       1
                    X2        C1        1
                    X2        C2       -1
                    X2        C3        1
                    X3        OBJ       1
                    X3        C1        1
                    X3        C2       -1
                    X3        C3        1
                RHS
                    RHS1      C1       10
                    RHS1      C2        5
                    RHS1      C3      100
                ENDATA
                """;
        SolverConfigurationDto config = new SolverConfigurationDto();
        config.setRevisedMaxIterations(1L);
        config.setRevisedMaxBaseCycles(5L);
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, config, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.MAX_ITERATIONS);
    }

    @Test
    public void solve_revised_simplex_max_base_cycles_integration_test() {
        String input = """
                NAME          LARGE
                ROWS
                 N  OBJ
                 L  C1
                 L  C2
                 L  C3
                COLUMNS
                    X1        OBJ       2
                    X1        C1        1
                    X1        C2        1
                    X1        C3       -1
                    X2        OBJ       1
                    X2        C1        1
                    X2        C2       -1
                    X2        C3        1
                    X3        OBJ       1
                    X3        C1        1
                    X3        C2       -1
                    X3        C3        1
                RHS
                    RHS1      C1       10
                    RHS1      C2        5
                    RHS1      C3      100
                ENDATA
                """;
        SolverConfigurationDto config = new SolverConfigurationDto();
        config.setRevisedMaxIterations(10L);
        config.setRevisedMaxBaseCycles(0L);
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, config, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.CYCLE);
    }

    @Test
    public void solve_revised_simplex_bounds_integration_test() {
        String input = """
                NAME          BOUNDSBS
                ROWS
                 N  OBJ
                 L  C1
                 L  C2
                COLUMNS
                    X1        OBJ       1
                    X1        C1        1
                    X1        C2       -1
                    X2        OBJ      -1
                    X2        C1        1
                    X2        C2       -1
                RHS
                    RHS1      C1       10
                    RHS1      C2       50
                BOUNDS
                 UP BND1      X1      100
                ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(new BigFraction(10));
        assert response.getResultVariableValues().get("X1").equals(new BigFraction(10));
        assert response.getResultVariableValues().get("S_2").equals(new BigFraction(60));
        assert response.getResultVariableValues().get("S_3").equals(new BigFraction(90));
    }

    @Test
    public void solve_revised_phase_simplex_largetp_succeeds() {
        String input = """
                NAME          LARGETP
                                      ROWS
                                       N  OBJ
                                       E  C1
                                       G  C2
                                       L  C3
                                      COLUMNS
                                          X1      OBJ     1/2    C1      1      C2      -1/3
                                          X1      C3      2
                                          X2      OBJ     -3/4   C1      2      C2      1
                                          X2      C3      -1/2
                                          X3      OBJ     1      C1      -1     C2      1/2
                                          X3      C3      1
                                          X4      OBJ     -2     C1      1/3    C2      2
                                          X4      C3      -1
                                      RHS
                                          RHS1    C1      3
                                          RHS1    C2      -2
                                          RHS1    C3      5/2
                                      ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        SolveLpRevisedSimlexResponseDto response = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(new BigFraction(19, 12));
        assert response.getResultVariableValues().get("X2").equals(new BigFraction(11, 3));
        assert response.getResultVariableValues().get("X3").equals(new BigFraction(13, 3));
        assert response.getResultVariableValues().get("S_1").equals(new BigFraction(47, 6));
    }



}
