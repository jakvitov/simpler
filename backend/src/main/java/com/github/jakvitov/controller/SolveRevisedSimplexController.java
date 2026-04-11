package com.github.jakvitov.controller;

import com.github.jakvitov.dto.solver.SolveLpErrorResponse;
import com.github.jakvitov.dto.solver.SolveLpRequestDto;
import com.github.jakvitov.mps.MpsParsingException;
import com.github.jakvitov.service.ErrorManagementService;
import com.github.jakvitov.service.RevisedSimplexSolverService;
import com.github.jakvitov.simplex.SimplexTableTransformationError;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Body;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Post;
import jakarta.inject.Inject;

import java.util.ArrayList;
import java.util.List;

@Controller("/solve-lp/revised")
public class SolveRevisedSimplexController {

    @Inject
    private RevisedSimplexSolverService revisedSimplexSolverService;

    @Inject
    private ErrorManagementService errorManagementService;

    @Post
    public HttpResponse<?> solveBasicSimplex(@Body SolveLpRequestDto solveLpRequestDto) {
        try {
            var res = revisedSimplexSolverService.handleSolveRevisedSimplexRequest(solveLpRequestDto);
            return HttpResponse.ok(res);
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
