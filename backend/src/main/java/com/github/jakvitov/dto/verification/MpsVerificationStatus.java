package com.github.jakvitov.dto.verification;

import io.micronaut.serde.annotation.Serdeable;

@Serdeable
public enum MpsVerificationStatus {

    OK,
    VERIFICATION_FAILED,

}
