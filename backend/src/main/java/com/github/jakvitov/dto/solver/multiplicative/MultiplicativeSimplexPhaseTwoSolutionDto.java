package com.github.jakvitov.dto.solver.multiplicative;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

import java.util.ArrayList;
import java.util.List;

@Data
@Serdeable
public class MultiplicativeSimplexPhaseTwoSolutionDto {

    private List<String> initialFeasibleBase;
    private List<MultiplicativeSimplexIterationDto> iterations = new ArrayList<>();

}
