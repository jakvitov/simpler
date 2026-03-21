package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.HashMap;
import java.util.Map;

/**
 * Simplex table dto containing information about normalization of non-leaving variable rows in Gaussian full eliminitaion.
 * Simplex table dto is given after normalization;
 */
@Serdeable
@Data
public class SimplexTableRowsNormalizationDto {

    SimplexTableDto simplexTableDto;
    //Row index -> coefficient
    Map<Integer, BigFraction> coefficients = new HashMap<>();
    //Separate coefficient for objective row normalization;
    BigFraction objectiveRowCoefficient;
    Integer leavingVariableIndex;

}
