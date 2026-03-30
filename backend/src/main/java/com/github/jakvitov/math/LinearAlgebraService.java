package com.github.jakvitov.math;

import jakarta.inject.Singleton;
import org.hipparchus.fraction.BigFraction;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

@Singleton
public class LinearAlgebraService {

    public Optional<List<List<BigFraction>>> getMatrixInversion(List<List<BigFraction>> inputMatrix) {
        int n = inputMatrix.size();

        // Validate square matrix
        for (List<BigFraction> row : inputMatrix) {
            if (row.size() != n) {
                return Optional.empty();
            }
        }

        // Build augmented matrix [A | I]
        List<List<BigFraction>> augmented = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            List<BigFraction> row = new ArrayList<>();
            // Copy original matrix
            for (int j = 0; j < n; j++) {
                row.add(inputMatrix.get(i).get(j));
            }
            // Append identity matrix
            for (int j = 0; j < n; j++) {
                row.add(i == j ? BigFraction.ONE : BigFraction.ZERO);
            }
            augmented.add(row);
        }

        // Gauss-Jordan elimination
        for (int col = 0; col < n; col++) {

            // Find pivot row (partial pivoting — find first non-zero in column)
            int pivotRow = -1;
            for (int row = col; row < n; row++) {
                if (augmented.get(row).get(col).compareTo(BigFraction.ZERO) != 0) {
                    pivotRow = row;
                    break;
                }
            }

            // If no pivot found, matrix is singular — not invertible
            if (pivotRow == -1) {
                return Optional.empty();
            }

            // Swap current row with pivot row
            if (pivotRow != col) {
                List<BigFraction> temp = augmented.get(col);
                augmented.set(col, augmented.get(pivotRow));
                augmented.set(pivotRow, temp);
            }

            // Scale pivot row so that pivot element becomes 1
            BigFraction pivotVal = augmented.get(col).get(col);
            for (int j = 0; j < 2 * n; j++) {
                augmented.get(col).set(j, augmented.get(col).get(j).divide(pivotVal));
            }

            // Eliminate all other rows in this column
            for (int row = 0; row < n; row++) {
                if (row == col) continue;
                BigFraction factor = augmented.get(row).get(col);
                if (factor.compareTo(BigFraction.ZERO) == 0) continue;
                for (int j = 0; j < 2 * n; j++) {
                    BigFraction updated = augmented.get(row).get(j)
                            .subtract(factor.multiply(augmented.get(col).get(j)));
                    augmented.get(row).set(j, updated);
                }
            }
        }

        // Extract the right half of the augmented matrix (the inverse)
        List<List<BigFraction>> inverse = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            List<BigFraction> row = new ArrayList<>();
            for (int j = n; j < 2 * n; j++) {
                row.add(augmented.get(i).get(j));
            }
            inverse.add(row);
        }

        return Optional.of(inverse);
    }

    public Optional<List<List<BigFraction>>> multiplyMatrices(
            List<List<BigFraction>> a,
            List<List<BigFraction>> b) {

        if (a.isEmpty() || b.isEmpty()) {
            return Optional.empty();
        }

        int aRows = a.size();
        int aCols = a.getFirst().size();
        int bRows = b.size();
        int bCols = b.getFirst().size();

        // Validate that all rows are consistent within each matrix
        for (List<BigFraction> row : a) {
            if (row.size() != aCols) return Optional.empty();
        }
        for (List<BigFraction> row : b) {
            if (row.size() != bCols) return Optional.empty();
        }

        // Core compatibility check: A's columns must equal B's rows
        if (aCols != bRows) {
            return Optional.empty();
        }

        // Multiply: result is (aRows x bCols)
        List<List<BigFraction>> result = new ArrayList<>();
        for (int i = 0; i < aRows; i++) {
            List<BigFraction> row = new ArrayList<>();
            for (int j = 0; j < bCols; j++) {
                BigFraction sum = BigFraction.ZERO;
                for (int k = 0; k < aCols; k++) {
                    sum = sum.add(a.get(i).get(k).multiply(b.get(k).get(j)));
                }
                row.add(sum);
            }
            result.add(row);
        }

        return Optional.of(result);
    }

}
