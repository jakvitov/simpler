package com.github.jakvitov.dto.solver.twophase;

import com.github.jakvitov.dto.solver.AbstractSolutionResponseDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import lombok.EqualsAndHashCode;

@EqualsAndHashCode(callSuper = true)
@Data
@Serdeable
public class SolveLpTwoPhaseSimplexResponseDto extends AbstractSolutionResponseDto {

    @Nullable
    private TwoPhaseSimplexPhaseOneSolutionDto phaseOneSolutionDto;
    @Nullable
    private TwoPhaseSimplexPhaseTwoSolutionDto phaseTwoSolutionDto;
}
