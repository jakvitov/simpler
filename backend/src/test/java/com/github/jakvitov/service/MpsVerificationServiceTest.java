package com.github.jakvitov.service;

import com.github.jakvitov.dto.lpdefinition.ParsedLpDefinitionDto;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.RowType;
import io.micronaut.test.extensions.junit5.annotation.MicronautTest;
import jakarta.inject.Inject;
import org.hipparchus.fraction.BigFraction;
import org.junit.jupiter.api.Test;

import java.util.LinkedHashMap;
import java.util.Map;

@MicronautTest
public class MpsVerificationServiceTest {

    @Inject
    private MpsVerificationService mpsVerificationService;

    @Test
    public void create_parsed_mpsData_definition_from_mps_data_succeeds() {
        MpsData mpsData = new MpsData();
        mpsData.name = "Test mpsData";

        mpsData.rows = new LinkedHashMap<>();
        mpsData.rows.put("obj", RowType.N);
        mpsData.rows.put("c1", RowType.L);
        mpsData.rows.put("c2", RowType.L);

        Map<String, BigFraction> xCol = new LinkedHashMap<>();
        xCol.put("obj", new BigFraction(1));
        xCol.put("c1", new BigFraction(1));
        xCol.put("c2", new BigFraction(3));

        Map<String, BigFraction> yCol = new LinkedHashMap<>();
        yCol.put("obj", new BigFraction(1));
        yCol.put("c1", new BigFraction(2));
        yCol.put("c2", new BigFraction(1));

        mpsData.columns = new LinkedHashMap<>();
        mpsData.columns.put("x", xCol);
        mpsData.columns.put("y", yCol);

        Map<String, BigFraction> rhsValues = new LinkedHashMap<>();
        rhsValues.put("c1", new BigFraction(4));
        rhsValues.put("c2", new BigFraction(5));

        mpsData.rhs = new LinkedHashMap<>();
        mpsData.rhs.put("rhs", rhsValues);

        mpsData.bounds = new LinkedHashMap<>();

        ParsedLpDefinitionDto parsedLpDefinitionDto = mpsVerificationService.createParsedLpDefinitionDtoFromMpsData(mpsData);

        assert parsedLpDefinitionDto.bounds().isEmpty();
        assert parsedLpDefinitionDto.lines().size() == 3;
        assert parsedLpDefinitionDto.warningMessage() == null || parsedLpDefinitionDto.warningMessage().isEmpty();
    }


}
