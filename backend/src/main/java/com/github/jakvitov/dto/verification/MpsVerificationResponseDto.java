package com.github.jakvitov.dto.verification;

import com.github.jakvitov.dto.lpdefinition.ParsedLpDefinitionDto;
import io.micronaut.serde.annotation.Serdeable;

import java.util.List;

@Serdeable
public record MpsVerificationResponseDto(MpsVerificationStatus status, List<String> errors, ParsedLpDefinitionDto parsedLpDefinition){}
