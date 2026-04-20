package com.github.jakvitov.simplex;

import com.github.jakvitov.dto.solver.SolutionStatus;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.basic.SolveLpBasicSimplexResponseDto;
import com.github.jakvitov.service.BasicSimplexSolverService;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

@MicronautTest
public class SolveBasicSimplexIntegrationTest {

    @Inject
    BasicSimplexSolverService basicSimplexSolverService;

    @Test
    public void solve_basic_simplex_fails_for_invalid_row_constraint() {
        String inputMps = """
                NAME          TESTPROB
                ROWS
                 N  COST
                 L  LIM1
                 G  LIM2
                 E  MYEQN
                COLUMNS
                    XONE      COST                 1   LIM1                 1
                    XONE      LIM2                 1
                    YTWO      COST                 4   LIM1                 1
                    YTWO      MYEQN               -1
                    ZTHREE    COST                 9   LIM2                 1
                    ZTHREE    MYEQN                1
                RHS
                    RHS1      LIM1                 5   LIM2                10
                    RHS1      MYEQN                7
                BOUNDS
                 UP BND1      XONE                 4
                 LO BND1      YTWO                -1
                 UP BND1      YTWO                 1
                ENDATA
                """;
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(inputMps, OptimisationTarget.MAX, SimplexVariant.BASIC_SIMPLEX, null, null);
        try {
            basicSimplexSolverService.handleSolveBasicSimplexRequest(solveLpRequestDto);
        } catch (SimplexTableTransformationError stte) {
            return;
        }
        assert false;
    }

    @Test
    public void solve_basic_simplex_minimalization_testmin_succeeds() {
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
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(input, OptimisationTarget.MIN, SimplexVariant.BASIC_SIMPLEX, null, null);
        SolveLpBasicSimplexResponseDto response = basicSimplexSolverService.handleSolveBasicSimplexRequest(solveLpRequestDto);
        assert response.getSolutionStatus().equals(SolutionStatus.SOLVED);
        assert response.getSolutionObjectiveFunctionValue().equals(BigFraction.ZERO);
        assert response.getResultVariableValues().get("S_1").equals(new BigFraction(4));
        assert response.getResultVariableValues().get("S_2").equals(new BigFraction(5));
        assert response.getResultVariableValues().get("S_3").equals(new BigFraction(7));
    }

}
