package com.github.jakvitov.dto;

import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public record HealthCheckResponseDto (String status, String version) {
}
