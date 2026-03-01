package com.github.jakvitov.dto.verification;

import com.github.jakvitov.dto.SimplexTableDto;
import io.micronaut.serde.annotation.Serdeable;

import java.util.List;

@Serdeable
public record MpsVerificationResponseDto(MpsVerificationStatus status, List<String> errors, SimplexTableDto initialSimplexTable){}
