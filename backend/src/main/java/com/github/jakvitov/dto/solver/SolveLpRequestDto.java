package com.github.jakvitov.dto.solver;

import com.github.jakvitov.dto.solver.config.SolverConfigurationDto;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexVariant;
import io.micronaut.core.annotation.Nullable;
import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public record SolveLpRequestDto(String data, OptimisationTarget optimisationTarget, SimplexVariant method, @Nullable SolverConfigurationDto solverConfiguration, String version) {}
