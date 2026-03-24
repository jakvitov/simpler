package com.github.jakvitov.dto.solver.twophase;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.Map;

@Data
@Serdeable
public class SolveLpTwoPhaseSimplexResponseDto {

    private SimplexTableDto initialSimplexTable;
    private TwoPhaseSimplexPhaseOneSolutionDto phaseOneSolutionDto;
    private TwoPhaseSimplexPhaseTwoSolutionDto phaseTwoSolutionDto;
    private SolutionStatus solutionStatus;
    //Null when SolutionStatus is not SOLVED
    @Nullable
    private Map<String, BigFraction> resultVariableValues;
    //Null when SolutionStatus is not SOLVED
    @Nullable
    private BigFraction solutionObjectiveFunctionValue;
}
