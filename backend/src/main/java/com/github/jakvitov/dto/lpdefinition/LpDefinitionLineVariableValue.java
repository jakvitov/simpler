package com.github.jakvitov.dto.lpdefinition;

import io.micronaut.serde.annotation.Serdeable;
import org.hipparchus.fraction.BigFraction;

@Serdeable
public record LpDefinitionLineVariableValue(String variableName, BigFraction value) {
}
