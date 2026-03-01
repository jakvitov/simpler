package com.github.jakvitov.dto.solver;

import com.github.jakvitov.simplex.SimplexVariant;
import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public record SolveLpRequestDto(String data, String optimisationTarget, SimplexVariant method) {}
