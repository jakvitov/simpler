package com.github.jakvitov.simplex;

import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.twophase.SolveLpTwoPhaseSimplexResponseDto;
import com.github.jakvitov.service.TwoPhaseSimplexSolverService;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.junit.jupiter.api.Test;

@MicronautTest
public class SolveTwoPhaseSimplexIntegrationTest {

    @Inject
    TwoPhaseSimplexSolverService twoPhaseSimplexSolverService;

    @Test
    public void solve_two_phase_simplex_succeeds() {
        String inputMps = """
                NAME          TEST1
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
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(inputMps, OptimisationTarget.MAX, SimplexVariant.TWO_PHASE, null, null);

        SolveLpTwoPhaseSimplexResponseDto response = twoPhaseSimplexSolverService.handleSolveTwoPhaseSimplexRequest(solveLpRequestDto);
        System.out.println(response);
    }

}
