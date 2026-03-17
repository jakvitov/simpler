package com.github.jakvitov.dto.lpdefinition;

/**
 * Exception to be thrown when ParsedLpDefinitionDto is being created from MpsData
 */
public class ParsedLpDefinitionCreationException extends RuntimeException {

    public ParsedLpDefinitionCreationException(String message) {
        super(message);
    }

}
