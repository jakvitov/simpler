package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

/**
 * Simplex table with row being transformed by BigFraction.
 * Given simplex table is already row transformed by the value.
 */
@Data
@Serdeable
public class SimplexTableLeavingRowNormalisationDto {

    SimplexTableDto simplexTableDto;
    int rowNormalizationIndex;
    BigFraction by;

}
