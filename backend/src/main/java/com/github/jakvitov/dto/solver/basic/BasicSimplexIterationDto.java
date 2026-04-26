package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

@Serdeable
@Data
public class BasicSimplexIterationDto {

    SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto;
    @Nullable
    SimplexTableLeavingRowNormalisationDto simplexTableLeavingRowNormalisationDto;
    @Nullable
    SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto;
    @Nullable
    SimplexTableDto simplexTableAfterVariableSwitch;

}
