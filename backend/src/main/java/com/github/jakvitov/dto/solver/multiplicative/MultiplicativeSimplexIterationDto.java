package com.github.jakvitov.dto.solver.multiplicative;

import com.github.jakvitov.dto.solver.revised.NonBasicVariableCurrentReducedCostCalculationDto;
import com.github.jakvitov.dto.solver.revised.RevisedSimplexIterationDto;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import lombok.EqualsAndHashCode;
import org.hipparchus.fraction.BigFraction;

import java.util.List;

@EqualsAndHashCode(callSuper = true)
@Data
@Serdeable
public class MultiplicativeSimplexIterationDto extends RevisedSimplexIterationDto {

    @Nullable
    private List<List<BigFraction>> elementaryMatrix;

    @Nullable
    private List<List<BigFraction>> elementaryMatrixInverse;

    @Nullable
    private List<List<BigFraction>> nextIterationBasisInverse;
}
