package com.github.jakvitov.controller;

import com.github.jakvitov.dto.HealthCheckResponseDto;
import com.github.jakvitov.dto.solver.SolveLpErrorResponse;
import com.github.jakvitov.dto.solver.basic.SolveLpBasicSimplexResponseDto;
import io.micronaut.context.annotation.Value;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;

@Controller("/api/simpler/health")
public class HealthCheckController {

    @Value("${simpler.backend.version:UNKNOWN}")
    private String backendVersion;

    private String status = "UP";

    @Get
    @Operation(summary = "Get health info")
    public HttpResponse<HealthCheckResponseDto> getHealthCheckResponseDto() {
        var response =  new HealthCheckResponseDto(status, backendVersion);
        return HttpResponse.ok(response);
    }
}
