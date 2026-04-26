package com.github.jakvitov.controller;

import com.github.jakvitov.dto.error.LastErrorResponseDto;
import com.github.jakvitov.dto.solver.SolveLpErrorResponse;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.basic.SolveLpBasicSimplexResponseDto;
import com.github.jakvitov.service.ErrorManagementService;
import com.github.jakvitov.simplex.OptimisationTarget;
import com.github.jakvitov.simplex.SimplexVariant;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import jakarta.inject.Inject;

import java.util.Optional;

@Controller("/api/simpler/error/last")
public class LastErrorController {

    @Inject
    private ErrorManagementService errorManagementService;

    @Get
    @Operation(summary = "Return information about last error")
    public HttpResponse<LastErrorResponseDto> getHealthCheckResponseDto() {
        Optional<LastErrorResponseDto> lastErrorResponseDto = errorManagementService.getLastErrorResponse();
        if (lastErrorResponseDto.isPresent()) {
            return HttpResponse.ok(lastErrorResponseDto.get());
        } else {
            return HttpResponse.notFound();
        }
    }

}
