package com.github.jakvitov.dto.solver.revised;

import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.List;
import java.util.Map;

@Serdeable
@Data
public class RevisedSimplexIterationDto {

    private List<String> currentBasis;
    //B of i-th iteration
    private List<List<BigFraction>> initialBasisMatrix;
    //B^(-1) of i-th iteration
    private List<List<BigFraction>> initialBasisMatrixInverse;

    //RHS
    private List<List<BigFraction>> xB;

    // c_B^T   Reduced costs of current basis variables in original simplex table
    private List<List<BigFraction>> originalSimplexTableReducedCosts;

    // y^t = c_b^T * B^(-1)
    private List<List<BigFraction>> yT;

    //variable column index in initial simplex table -> its current reduced cost
    private Map<Integer, BigFraction> nonBasicVariablesCurrentReducedCosts;

    @Nullable
    private Integer enteringVariableIndex;

    // d
    //todo check this comes as undefined/null and not empty to FE
    @Nullable
    private List<List<BigFraction>> directionVector;

    @Nullable
    List<BigFraction> ratioVector;

    @Nullable
    Integer leavingVariableIndex;

    @Nullable
    private List<String> updatedBasis;
}
