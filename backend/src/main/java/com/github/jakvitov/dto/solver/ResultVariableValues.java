package com.github.jakvitov.dto.solver;

import io.micronaut.serde.annotation.Serdeable;
import lombok.Data;
import org.hipparchus.fraction.BigFraction;

import java.util.HashMap;
import java.util.Map;

@Data
@Serdeable
public class ResultVariableValues {

    private Map<String, BigFraction> problemVariables = new HashMap<>();
    private Map<String, BigFraction> slackSurplusVariables = new HashMap<>();

}
