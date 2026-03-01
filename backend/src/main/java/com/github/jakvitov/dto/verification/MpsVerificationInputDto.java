package com.github.jakvitov.dto.verification;

import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public record MpsVerificationInputDto(String data) {}
