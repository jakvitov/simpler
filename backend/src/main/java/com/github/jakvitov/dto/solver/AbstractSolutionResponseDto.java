package com.github.jakvitov.dto.solver;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.Map;

@Data
@Serdeable
public class AbstractSolutionResponseDto {

    private SolutionStatus solutionStatus;

    private SimplexTableDto initialSimplexTable;

    //Null when SolutionStatus is not SOLVED
    @Nullable
    private Map<String, BigFraction> resultVariableValues;

    //Null when SolutionStatus is not SOLVED
    @Nullable
    private BigFraction solutionObjectiveFunctionValue;

    private boolean success = true;

}
