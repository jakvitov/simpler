package com.github.jakvitov.dto.solver.revised;

import com.github.jakvitov.dto.solver.AbstractSolutionResponseDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import lombok.EqualsAndHashCode;

@EqualsAndHashCode(callSuper = true)
@Serdeable
@Data
public class SolveLpRevisedSimlexResponseDto extends AbstractSolutionResponseDto {

    @Nullable
    private RevisedSimplexPhaseOneSolutionDto revisedSimplexPhaseOneSolution;

    @Nullable
    private RevisedSimplexPhaseTwoSolutionDto revisedSimplexPhaseTwoSolutionDto;


}
