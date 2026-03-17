package com.github.jakvitov.dto.lpdefinition;

import io.micronaut.serde.annotation.Serdeable;
import jakarta.annotation.Nullable;
import org.hipparchus.fraction.BigFraction;

@Serdeable
public record LpDefinitionBoundDto(String variableName, @Nullable BigFraction upperbound, @Nullable BigFraction lowerbound) {
}
