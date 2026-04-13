package com.github.jakvitov.service;

import com.github.jakvitov.dto.solver.config.SolverConfigurationDto;
import io.micronaut.context.annotation.Value;
import io.micronaut.core.annotation.Nullable;
import jakarta.inject.Singleton;

@Singleton
public class SolverConfigurationService {

    @Value("${simpler.simplex.basic.max.iterations}")
    private Long bsMaxIterations;

    @Value("${simpler.simplex.basic.simplex.max.base.cycles}")
    private Long bsMaxCycles;

    @Value("${simpler.simplex.two-phase.max.iterations}")
    private Long tpMaxIterations;

    @Value("${simpler.simplex.two-phase.max.base.cycles}")
    private Long tpMaxCycles;

    @Value("${simpler.simplex.revised.max.iterations}")
    private Long rsMaxIterations;

    @Value("${simpler.simplex.revised.max.base.cycles}")
    private Long rsMaxCycles;

    public Long getConfig(SolverConfigurationConstants type, @Nullable SolverConfigurationDto inputConfiguration) {
        return switch (type) {
            case BS_MAX_ITER -> {
                if (inputConfiguration == null || inputConfiguration.getBasicSimplexMaxIterations() == null) {
                    yield bsMaxIterations;
                } else {
                    yield inputConfiguration.getBasicSimplexMaxIterations();
                }
            }
            case BS_MAX_CYCLE -> {
                if (inputConfiguration == null || inputConfiguration.getBasicSimplexMaxBaseCycles() == null) {
                    yield bsMaxCycles;
                } else {
                    yield inputConfiguration.getBasicSimplexMaxBaseCycles();
                }
            }
            case TP_MAX_ITER -> {
                if (inputConfiguration == null || inputConfiguration.getTwoPhaseMaxIterations() == null) {
                    yield tpMaxIterations;
                } else {
                    yield inputConfiguration.getTwoPhaseMaxIterations();
                }
            }
            case TP_MAX_CYCLE -> {
                if (inputConfiguration == null || inputConfiguration.getTwoPhaseMaxBaseCycles() == null) {
                    yield tpMaxCycles;
                } else {
                    yield inputConfiguration.getTwoPhaseMaxBaseCycles();
                }
            }
            case RS_MAX_ITER -> {
                if (inputConfiguration == null || inputConfiguration.getRevisedMaxIterations() == null) {
                    yield rsMaxIterations;
                } else {
                    yield inputConfiguration.getRevisedMaxIterations();
                }
            }
            case RS_MAX_CYCLE -> {
                if (inputConfiguration == null || inputConfiguration.getRevisedMaxBaseCycles() == null) {
                    yield rsMaxCycles;
                } else {
                    yield inputConfiguration.getRevisedMaxBaseCycles();
                }
            }
        };
    }

    public enum SolverConfigurationConstants {
        BS_MAX_ITER,
        BS_MAX_CYCLE,
        TP_MAX_ITER,
        TP_MAX_CYCLE,
        RS_MAX_ITER,
        RS_MAX_CYCLE,
    }

}
