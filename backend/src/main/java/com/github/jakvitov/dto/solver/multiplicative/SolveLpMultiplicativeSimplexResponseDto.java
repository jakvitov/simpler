package com.github.jakvitov.dto.solver.multiplicative;

import com.github.jakvitov.dto.solver.AbstractSolutionResponseDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import lombok.EqualsAndHashCode;

@EqualsAndHashCode(callSuper = true)
@Data
@Serdeable
public class SolveLpMultiplicativeSimplexResponseDto extends AbstractSolutionResponseDto {

    @Nullable
    private MultiplicativeSimplexPhaseOneSolutionDto multiplicativeSimplexPhaseOneSolutionDto;

    @Nullable
    private MultiplicativeSimplexPhaseTwoSolutionDto multiplicativeSimplexPhaseTwoSolutionDto;

}
