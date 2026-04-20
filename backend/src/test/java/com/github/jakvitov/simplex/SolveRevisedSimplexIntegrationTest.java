package com.github.jakvitov.simplex;

import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.service.RevisedSimplexSolverService;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.junit.jupiter.api.Test;

@MicronautTest
public class SolveRevisedSimplexIntegrationTest {

    @Inject
    private RevisedSimplexSolverService revisedSimplexSolverService;

    @Test
    public void solve_revised_simplex_succeeds_for_non_phase_one_lp() {
        String mpsData = """
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
        SolveLpRequestDto solveLpRequestDto = new SolveLpRequestDto(mpsData, OptimisationTarget.MAX, SimplexVariant.REVISED, null, null);
        revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
    }

}
