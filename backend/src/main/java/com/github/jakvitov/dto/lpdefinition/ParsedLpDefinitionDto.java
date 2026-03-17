package com.github.jakvitov.dto.lpdefinition;

import com.github.jakvitov.simplex.OptimisationTarget;
import io.micronaut.serde.annotation.Serdeable;

import java.util.List;

@Serdeable
public record ParsedLpDefinitionDto(List<LpDefinitionLineDto> lines, List<LpDefinitionBoundDto> bounds,
                                    String warningMessage /* Used for additional info to user, such as multiple RHS not supported*/) {
}
