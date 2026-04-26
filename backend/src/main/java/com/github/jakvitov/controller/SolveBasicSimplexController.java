package com.github.jakvitov.controller;

import com.github.jakvitov.dto.solver.SolveLpErrorResponse;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.dto.solver.basic.SolveLpBasicSimplexResponseDto;
import com.github.jakvitov.mps.MpsParsingException;
import com.github.jakvitov.service.BasicSimplexSolverService;
import com.github.jakvitov.service.ErrorManagementService;
import com.github.jakvitov.simplex.SimplexTableTransformationError;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Body;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Post;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import io.swagger.v3.oas.annotations.responses.ApiResponses;
import jakarta.inject.Inject;

import java.util.ArrayList;
import java.util.List;

@Controller("/api/simpler/solve-lp/basic")
public class SolveBasicSimplexController {

    @Inject
    private BasicSimplexSolverService basicSimplexSolverService;

    @Inject
    private ErrorManagementService errorManagementService;

    @Post
    @Operation(summary = "Solve LP using basic simplex")
    @ApiResponses({
            @ApiResponse(
                    responseCode = "200",
                    description = "Successful solution",
                    content = @Content(
                            mediaType = "application/json",
                            schema = @Schema(implementation = SolveLpBasicSimplexResponseDto.class)
                    )
            ),
            @ApiResponse(
                    responseCode = "400",
                    description = "Invalid input"
            ),
            @ApiResponse(
                    responseCode = "500",
                    description = "Server error",
                    content = @Content(
                            schema = @Schema(implementation = SolveLpErrorResponse.class)
                    )
            )
    })
    public HttpResponse<?> solveBasicSimplex(@Body SolveLpRequestDto solveLpRequestDto) {
        try {
            return HttpResponse.ok(basicSimplexSolverService.handleSolveBasicSimplexRequest(solveLpRequestDto));
        }
        catch (MpsParsingException mpe) {
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(List.of(mpe.reasons), false);
            return HttpResponse.badRequest(errorResponse);
        } catch (SimplexTableTransformationError stte) {
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(List.of(stte.getMessage()), false);
            return HttpResponse.badRequest(errorResponse);
        }
        catch (Exception e) {
            errorManagementService.registerLastExceptionAndLog(e, solveLpRequestDto);
            List<String> errors = new ArrayList<>(1);
            errors.add(e.getMessage());
            SolveLpErrorResponse errorResponse = new SolveLpErrorResponse(errors, false);
            return HttpResponse.serverError(errorResponse);
        }
    }


}
