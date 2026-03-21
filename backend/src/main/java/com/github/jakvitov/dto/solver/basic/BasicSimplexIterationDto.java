package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

@Serdeable
@Data
public class BasicSimplexIterationDto {

    SimplexTableLeavingEnteringVariableDto simplexTableLeavingEnteringVariableDto;
    SimplexTableLeavingRowNormalisationDto simplexTableLeavingRowNormalisationDto;
    SimplexTableRowsNormalizationDto simplexTableRowsNormalizationDto;
    SimplexTableDto simplexTableAfterVariableSwitch;

}
