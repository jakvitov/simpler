package com.github.jakvitov.dto.solver.basic;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.List;

/**
 * Part of simplex iteration. Simplex table with t-vector and entering + leaving variable and t-vector
 */

@Serdeable
@Data
public class SimplexTableLeavingEnteringVariableDto {

    private SimplexTableDto simplexTableDto;
    @Nullable
    private List<BigFraction> tVector;
    @Nullable
    private Integer leavingVariableIndex;
    @Nullable
    private Integer enteringVariableIndex;
}
