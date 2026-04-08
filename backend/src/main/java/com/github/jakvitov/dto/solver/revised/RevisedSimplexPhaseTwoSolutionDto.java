package com.github.jakvitov.dto.solver.revised;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

import java.util.ArrayList;
import java.util.List;

@Data
@Serdeable
public class RevisedSimplexPhaseTwoSolutionDto {

    private List<String> initialFeasibleBase;
    private List<RevisedSimplexIterationDto> iterations = new ArrayList<>();

}
