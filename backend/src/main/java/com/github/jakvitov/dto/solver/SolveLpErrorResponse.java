package com.github.jakvitov.dto.solver;

import io.micronaut.serde.annotation.Serdeable;

import java.util.List;

/**
 * Common error response for error during solution of LP
 * @param errors
 * @param success
 */

@Serdeable
public record SolveLpErrorResponse(List<String> errors, boolean success) {
}
