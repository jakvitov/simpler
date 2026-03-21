package com.github.jakvitov.service;

import com.github.jakvitov.dto.lpdefinition.*;
import com.github.jakvitov.dto.verification.MpsVerificationInputDto;
import com.github.jakvitov.dto.verification.MpsVerificationResponseDto;
import com.github.jakvitov.dto.verification.MpsVerificationStatus;
import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.mps.*;
import com.github.jakvitov.simplex.SimplexTable;
import io.micronaut.http.server.exceptions.ContentLengthExceededHandler;
import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

@Singleton
public class MpsVerificationService {

    private final ContentLengthExceededHandler contentLengthExceededHandler;

    public MpsVerificationService(ContentLengthExceededHandler contentLengthExceededHandler) {
        this.contentLengthExceededHandler = contentLengthExceededHandler;
    }

    public MpsVerificationResponseDto verifyMps(MpsVerificationInputDto mpsVerificationInputDto) {
        List<String> errors = new ArrayList<>();
        if (mpsVerificationInputDto.data().isBlank()) {
            errors.add("Input MPS data is blank.");
            return new MpsVerificationResponseDto(MpsVerificationStatus.VERIFICATION_FAILED, errors, null);
        }
        try {
            MpsData mpsData = MpsData.parse(mpsVerificationInputDto.data());
            mpsData.validate();
            ParsedLpDefinitionDto parsedLpDefinitionDto = createParsedLpDefinitionDtoFromMpsData(mpsData);
            return new MpsVerificationResponseDto(MpsVerificationStatus.OK, errors, parsedLpDefinitionDto);
        }
        catch (MpsParsingException mpe) {
            errors.addAll(List.of(mpe.reasons));
        }
        catch (Exception e) {
            errors.add(e.getMessage());
        }
        return new MpsVerificationResponseDto(MpsVerificationStatus.VERIFICATION_FAILED, errors, null);
    }

    public ParsedLpDefinitionDto createParsedLpDefinitionDtoFromMpsData(MpsData mpsData) {
        List<String> warningMessages = new ArrayList<>();

        if (mpsData.rhs.isEmpty()) {
            throw new ParsedLpDefinitionCreationException("No RHS found in the MPS data. Problem needs at least one correct RHS to be valid!");
        } else if (mpsData.rhs.size() > 1) {
            warningMessages.add("Multiple RHS found in the MPS definition.\nOnly one is supported in LP overview and only the first one will be displayed.");
        }
        if (mpsData.bounds.size() > 1) {
            warningMessages.add("Multiple BOUNDS found in the MPS definition.\nOnly one is supported in LP overview and only the first one will be displayed.");
        }
        Map<String, BigFraction> rhs = mpsData.rhs.entrySet().iterator().next().getValue();

        LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>> bounds;
        if (mpsData.bounds.isEmpty()) {
            bounds = new LinkedHashMap<>();
        } else {
            bounds = mpsData.bounds.entrySet().iterator().next().getValue();
        }

        //Since we only parse lp definition, multiple or none objective rows is allowed
        List<String> objectiveRowsName = mpsData.rows.entrySet().stream().filter(rowEntry -> rowEntry.getValue().equals(RowType.N)).map(Map.Entry::getKey).toList();

        List<LpDefinitionLineDto> lpDefinitionLines = new ArrayList<>();
        //Add non-objective rows values
        for (Map.Entry<String, RowType> row: mpsData.rows.entrySet()) {
            if (row.getValue().equals(RowType.N)) {
                continue;
            }
            List<LpDefinitionLineVariableValue> variableValues = new ArrayList<>();
            for (Map.Entry<String, Map<String, BigFraction>> mpsDataVariableValues: mpsData.columns.entrySet()) {
                String variableName = mpsDataVariableValues.getKey();
                BigFraction variableValue;
                if (mpsDataVariableValues.getValue().get(row.getKey()) != null) {
                    variableValue = mpsDataVariableValues.getValue().get(row.getKey());
                } else {
                    variableValue = BigFraction.ZERO;
                }

                LpDefinitionLineVariableValue lpDefinitionLineVariableValue = new LpDefinitionLineVariableValue(variableName, variableValue);
                variableValues.add(lpDefinitionLineVariableValue);
            }
            LpDefinitionLineDto lpDefinitionLineDto = new LpDefinitionLineDto(variableValues, LpDefinitionInequalitySign.fromMpsRowType(row.getValue()), rhs.get(row.getKey()));
            lpDefinitionLines.add(lpDefinitionLineDto);
        }
        //Fill in objective rows at the end
        for (String objectiveRowName: objectiveRowsName) {
            List<LpDefinitionLineVariableValue> variableValues = new ArrayList<>();
            for (Map.Entry<String, Map<String, BigFraction>> mpsDataVariableValues: mpsData.columns.entrySet()) {
                String variableName = mpsDataVariableValues.getKey();
                BigFraction variableValue;
                if (mpsDataVariableValues.getValue().get(objectiveRowName) != null) {
                    variableValue = mpsDataVariableValues.getValue().get(objectiveRowName);
                } else {
                    variableValue = BigFraction.ZERO;
                }

                LpDefinitionLineVariableValue lpDefinitionLineVariableValue = new LpDefinitionLineVariableValue(variableName, variableValue);
                variableValues.add(lpDefinitionLineVariableValue);
            }
            LpDefinitionLineDto lpDefinitionLineDto = new LpDefinitionLineDto(variableValues, LpDefinitionInequalitySign.N, null);
            lpDefinitionLines.add(lpDefinitionLineDto);
        }

        List<LpDefinitionBoundDto> lpDefinitionBounds = new ArrayList<>();
        //variable -> bound type -> value
        for (Map.Entry<String, LinkedHashMap<BoundType, BigFraction>> variableBound: bounds.entrySet()) {
            BigFraction upperbound = null;
            BigFraction lowerbound = null;
            if (variableBound.getValue().get(BoundType.UP) != null) {
                upperbound = variableBound.getValue().get(BoundType.UP);
            }
            if (variableBound.getValue().get(BoundType.LO) != null) {
                lowerbound = variableBound.getValue().get(BoundType.LO);
            }
            LpDefinitionBoundDto lpDefinitionBoundDto = new LpDefinitionBoundDto(variableBound.getKey(), upperbound, lowerbound);
            lpDefinitionBounds.add(lpDefinitionBoundDto);
        }
        return new ParsedLpDefinitionDto(lpDefinitionLines, lpDefinitionBounds, String.join("\n", warningMessages));
    }

}
