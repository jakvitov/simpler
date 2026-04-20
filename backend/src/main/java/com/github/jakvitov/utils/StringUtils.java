package com.github.jakvitov.utils;

import jakarta.annotation.Nonnull;
import org.hipparchus.fraction.BigFraction;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.List;

public class StringUtils {

    @Nonnull
    public static List<Integer> allIndexesOf(List<? extends Object> input, Object itemToFind) {
        List<Integer> indexes = new ArrayList<>();
        for (int i = 0; i < input.size(); i++) {
            if (input.get(i).equals(itemToFind)) {
                indexes.add(i);
            }
        }
        return indexes;
    }

    /**
     * Parse BigFraction in format "BigInteger/BigInteger".
     * @param input
     * @return
     * @throws NumberFormatException
     */
    public static BigFraction parseBigFraction(String input) throws NumberFormatException {
        try {
            if (input.contains("/")) {
                String[] parts = input.split("/");
                BigInteger numerator = new BigInteger(parts[0]);
                BigInteger denominator = new BigInteger(parts[1]);
                return new BigFraction(numerator, denominator);
            }
            else if (input.contains(".")) {
                double bd = Double.parseDouble(input);
                return new BigFraction(bd);
            }
            else {
                return new BigFraction(new BigInteger(input));
            }
        } catch (Exception e) {
            throw new NumberFormatException(e.getMessage());
        }
    }

}
