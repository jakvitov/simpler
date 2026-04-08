package com.github.jakvitov.dto.solver.revised;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.Map;

@Serdeable
@Data
public class SolveLpRevisedSimlexResponseDto {

    private SolutionStatus solutionStatus;
    private SimplexTableDto initialSimplexTable;

    @Nullable
    private RevisedSimplexPhaseOneSolutionDto revisedSimplexPhaseOneSolution;

    @Nullable
    private RevisedSimplexPhaseTwoSolutionDto revisedSimplexPhaseTwoSolutionDto;

    @Nullable
    private Map<String, BigFraction> resultVariableValues;
    @Nullable
    private BigFraction solutionObjectiveFunctionValue;

    //Artificial field for FE rendering
    private boolean success = true;

}
