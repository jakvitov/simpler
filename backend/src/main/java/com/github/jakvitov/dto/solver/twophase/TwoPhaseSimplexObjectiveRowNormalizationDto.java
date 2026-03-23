package com.github.jakvitov.dto.solver.twophase;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.Map;

@Data
@Serdeable
public class TwoPhaseSimplexObjectiveRowNormalizationDto {

    private SimplexTableDto simplexTableDto;
    private Map<Integer, BigFraction> coefficients;

}
