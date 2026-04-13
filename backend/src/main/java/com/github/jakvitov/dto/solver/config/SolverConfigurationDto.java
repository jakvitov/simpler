package com.github.jakvitov.dto.solver.config;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;

@Data
@Serdeable
public class SolverConfigurationDto {

    private Long basicSimplexMaxIterations;
    private Long basicSimplexMaxBaseCycles;
    private Long twoPhaseMaxIterations;
    private Long twoPhaseMaxBaseCycles;
    private Long revisedMaxIterations;
    private Long revisedMaxBaseCycles;

}
