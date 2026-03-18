package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.dto.solver.SolutionStatus;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Serdeable
@Data
public class SolveLpBasicSimplexResponseDto {

    private SolutionStatus solutionStatus;

    private SimplexTableDto initialSimplexTable;

    private List<BasicSimplexIterationDto> iterations = new ArrayList<>();

    private SimplexTableDto finalSimplexTable;

    //Null when SolutionStatus is not SOLVED
    @Nullable
    private Map<String, BigFraction> resultVariableValues;

    //Null when SolutionStatus is not SOLVED
    @Nullable
    private BigFraction solutionObjectiveFunctionValue;

    private boolean success = true;

}
