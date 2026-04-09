package com.github.jakvitov.dto.solver.revised;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.List;

@Data
@Serdeable
public class NonBasicVariableCurrentReducedCostCalculationDto {
    private String variableName;
    //cHat_j
    private BigFraction cJ;
    private BigFraction nonBasicVariableReducedCost;
    private List<List<BigFraction>> aJ;
    //cJ - nonBasicVariableReducedCost
    private BigFraction result;
}
