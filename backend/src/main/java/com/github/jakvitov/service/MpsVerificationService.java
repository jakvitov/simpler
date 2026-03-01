package com.github.jakvitov.service;

import com.github.jakvitov.dto.verification.MpsVerificationInputDto;
import com.github.jakvitov.dto.verification.MpsVerificationResponseDto;
import com.github.jakvitov.dto.verification.MpsVerificationStatus;
import com.github.jakvitov.dto.SimplexTableDto;
import com.github.jakvitov.mps.MpsData;
import com.github.jakvitov.mps.MpsDataTransformedBounds;
import com.github.jakvitov.mps.MpsParsingException;
import com.github.jakvitov.simplex.SimplexTable;
import jakarta.inject.Singleton;

import java.util.ArrayList;
import java.util.List;

@Singleton
public class MpsVerificationService {

    public MpsVerificationResponseDto verifyMps(MpsVerificationInputDto mpsVerificationInputDto) {
        List<String> errors = new ArrayList<>();
        if (mpsVerificationInputDto.data().isBlank()) {
            errors.add("Input MPS data is blank.");
            return new MpsVerificationResponseDto(MpsVerificationStatus.VERIFICATION_FAILED, errors, null);
        }
        try {
            MpsData mpsData = MpsData.parse(mpsVerificationInputDto.data());
            mpsData.validate();
            MpsDataTransformedBounds mpsDataTransformedBounds = new MpsDataTransformedBounds(mpsData);
            SimplexTable simplexTable = SimplexTable.fromMpsData(mpsDataTransformedBounds);
            var result = new MpsVerificationResponseDto(MpsVerificationStatus.OK, errors, new SimplexTableDto(simplexTable));
            return result;
        }
        catch (MpsParsingException mpe) {
            errors.addAll(List.of(mpe.reasons));
        }
        catch (Exception e) {
            errors.add(e.getMessage());
        }
        return new MpsVerificationResponseDto(MpsVerificationStatus.VERIFICATION_FAILED, errors, null);
    }

}
