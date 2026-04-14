package com.github.jakvitov.controller;

import com.github.jakvitov.dto.verification.MpsVerificationInputDto;
import com.github.jakvitov.dto.verification.MpsVerificationResponseDto;
import com.github.jakvitov.service.MpsVerificationService;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.annotation.Body;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Post;
import jakarta.inject.Inject;

@Controller("/api/simpler/mps/verify")
public class MpsVerificationController {

    @Inject
    private MpsVerificationService mpsVerificationService;

    @Post
    public HttpResponse<MpsVerificationResponseDto> verifyMps(@Body MpsVerificationInputDto mpsVerificationInputDto) {
        return HttpResponse.ok(mpsVerificationService.verifyMps(mpsVerificationInputDto));
    }

}
