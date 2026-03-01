package com.github.jakvitov.mps;

/**
 * Exception linked to internal validity problems with MPS data.
 * @see com.github.jakvitov.mps.MpsData
 */
public class MpsValidationException extends RuntimeException {

    public MpsValidationException(String message) {
        super(message);
    }

}
