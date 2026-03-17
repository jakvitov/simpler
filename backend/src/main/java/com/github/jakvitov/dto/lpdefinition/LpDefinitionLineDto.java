package com.github.jakvitov.dto.lpdefinition;

import io.micronaut.serde.annotation.Serdeable;
import org.hipparchus.fraction.BigFraction;

import java.util.List;

@Serdeable
public record LpDefinitionLineDto(List<LpDefinitionLineVariableValue> variableValues, LpDefinitionInequalitySign inequalitySign, BigFraction rhs) {
}
