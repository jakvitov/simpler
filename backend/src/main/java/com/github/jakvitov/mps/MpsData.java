package com.github.jakvitov.mps;

import com.github.jakvitov.utils.StringUtils;
import org.hipparchus.fraction.BigFraction;

import java.util.*;
import java.util.stream.Collectors;

/**
 * Class representing data contained in a MPS format input
 */
public class MpsData {

    public String name;
    public LinkedHashMap<String, RowType> rows;
    //Variable name -> row name -> value
    public LinkedHashMap<String, Map<String, BigFraction>> columns;
    //Rhs name -> Row name -> value
    public Map<String, Map<String, BigFraction>> rhs;
    //Bound name -> variable -> bound type -> value
    public Map<String, LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>>> bounds;


    /**
     * Parse given string as MPS data. This does not perform any validation, just plain parse. All produced errors are linked
     * to inability to create MpsData object, not its internal validity.
     * @param input
     * @return
     * @throws MpsParsingException
     */
    public static MpsData parse(String input) throws MpsParsingException {
        String[] lines = input.split("\n");

        List<String> prefilteredLines = Arrays.stream(lines).filter(line -> !line.startsWith("#") && !line.isBlank())
                .map(line -> {
                    int commentIndex = line.indexOf("#");
                    if (commentIndex != -1) {
                        return line.substring(0, commentIndex);
                    }
                    return line;
                })
                .map(line -> line.replaceAll("(?m)^\\s+|\\s+$", "") /*All whitespaces at the beginning and end of lines */)
                .toList();

        MpsData mpsData = new MpsData();
        mpsData.name = mpsData.parseName(prefilteredLines);
        mpsData.rows = mpsData.parseRows(prefilteredLines);
        mpsData.columns = mpsData.parseColumns(prefilteredLines);
        mpsData.rhs = mpsData.parseRhs(prefilteredLines);
        mpsData.bounds = mpsData.parseBounds(prefilteredLines);

        return mpsData;
    }

    /**
     * Validate internal structure of the MPS data. Check that all required values are present and
     * data can be parsed into a simplex table or alike.
     * @throws MpsValidationException
     */
    public void validate() throws MpsValidationException {
        if (name == null || name.isBlank()) {
            throw new MpsValidationException("NAME section is missing in the provided Mps input!");
        }

        //Validate ROWS
        if (rows == null || rows.isEmpty()) {
            throw new MpsValidationException("ROWS section appears to be empty or missing!");
        }
        //Exactly one target row is present
        List<Map.Entry<String, RowType>> targetRows = rows.entrySet().stream().filter(entry -> entry.getValue().equals(RowType.N)).toList();
        if (targetRows.isEmpty()) {
            throw new MpsValidationException("No target row provided (N). Problem must contain exactly one.");
        } else if (targetRows.size() > 1) {
            throw new MpsValidationException("Multiple target rows provided and only one is currently supported. Target rows are: " + targetRows.stream().map(Map.Entry::getKey).collect(Collectors.joining()));
        }
        String targetRowName = targetRows.getFirst().getKey();

        //Any constraint row is present
        List<String> constraintRowsNames = rows.entrySet().stream().filter(entry -> !entry.getValue().equals(RowType.N))
                .map(Map.Entry::getKey).toList();
        if (constraintRowsNames.isEmpty()) {
            throw  new MpsValidationException("Please provide at least one constraining row for the problem! Only objective row was found.");
        }

        //Validate COLUMNS
        if (columns == null || columns.isEmpty()) {
            throw new MpsValidationException("COLUMNS section appears to be empty or missing!");
        }
        //Each variable has existing columns defined
        //Slack surplus and artificial variables reserved names violation check.
        columns.forEach((variableName, variableValues) -> {
            if (variableName.startsWith("S_") || variableName.startsWith("A_")) {
                throw new MpsValidationException("Variable names starting with S_ or A_ are reserved for added variables. Failing variable name: " + variableName);
            }

            Collection<String> variableRowNames = variableValues.keySet();
            List<String> undefinedRowNames = variableRowNames.stream().filter((rowName) -> !rows.containsKey(rowName)).toList();

            if (!undefinedRowNames.isEmpty()) {
                throw new MpsValidationException("COLUMNS section, variable " + variableName + " contains ROW value definition for non-existing row/rows: " + String.join(",", undefinedRowNames));
            }
        });

        //Validate RHS
        if (rhs == null || rhs.isEmpty()) {
            throw new MpsValidationException("RHS section appears to be empty or missing!");
        }
        //Each RHS contains all non-target rows and none RHs rows contain value for a target row
        rhs.forEach((rhsName, rhsValues) -> {
           List<String> missingConstraintRows = constraintRowsNames.stream().filter((constrRowName) -> !rhsValues.containsKey(constrRowName)).toList();
           if (!missingConstraintRows.isEmpty()) {
               throw new MpsValidationException("RHS " + rhsName + " is missing value definition for a constraint ROWs: " + String.join(", ", missingConstraintRows));
           }

           if (rhsValues.containsKey(targetRowName)) {
               throw new MpsValidationException("RHS " + rhsName + " contains value for a target row " + targetRowName + ". This is not supported.");
           }

        });

        //Validate Bounds
        //Each bound contains only predefined variables in COLUMNS
        bounds.forEach((boundName, boundValues) -> {
            List<String> undefinedVariableNames = boundValues.keySet().stream().filter(boundVariable -> !columns.containsKey(boundVariable)).toList();
            if (!undefinedVariableNames.isEmpty()) {
                throw new MpsValidationException("BOUND " + boundName + " contains variables, that are undefined in the COLUMNS section: " + String.join(", ", undefinedVariableNames));
            }
        });
    }

    private String parseName(List<String> prefilteredLines) {
        List<String> nameLines = prefilteredLines.stream().filter(line -> line.startsWith("NAME")).toList();

        if (nameLines.size() > 1) {
            throw new MpsParsingException(MpsSections.NAME, "Multiple NAME sections found!", nameLines);
        } else if (nameLines.isEmpty()) {
            throw new MpsParsingException(MpsSections.NAME, "No NAME section found!");
        }
        String nameLine = nameLines.getFirst();
        String[] parts = nameLine.split("\\s+");
        return String.join(" ", Arrays.stream(parts).toList().subList(1, parts.length));
    }

    private LinkedHashMap<String, RowType> parseRows(List<String> prefilteredLines) {
        LinkedHashMap<String, RowType> rowsResult = new LinkedHashMap<>();

        List<String> rowSectionData = getSectionLines(MpsSections.ROWS, prefilteredLines);

        for (String rowSectionLine: rowSectionData) {
            String[] parts = rowSectionLine.split("\\s+");
            if (parts.length != 2) {
                throw new MpsParsingException(MpsSections.ROWS, "Invalid ROWS section found!", rowSectionLine);
            }

            try {
                RowType rowType = RowType.valueOf(parts[0]);
                String rowName = parts[1];
                rowsResult.put(rowName, rowType);
            } catch (IllegalArgumentException e) {
                throw new MpsParsingException(MpsSections.ROWS, "ROWS section row type contains invalid value.", rowSectionLine);
            }
        }
        return rowsResult;
    }

    private LinkedHashMap<String, Map<String, BigFraction>> parseColumns(List<String> prefilteredLines) {
        List<String> columnsSectionData = getSectionLines(MpsSections.COLUMNS, prefilteredLines);

        LinkedHashMap<String, Map<String, BigFraction>> columnsSection = new LinkedHashMap<>();
        for (String columnsSectionDataLine: columnsSectionData) {
            String[] parts = columnsSectionDataLine.split("\\s+");

            //If the parts of the columns row are even, it cannot be valid
            if (parts.length == 0 || parts.length % 2 != 1) {
                throw new MpsParsingException(MpsSections.COLUMNS, "Invalid COLUMNS section found!", columnsSectionDataLine);
            }

            String variableName = parts[0];
            Map<String, BigFraction> variableValues = columnsSection.get(variableName);
            if (variableValues == null) {
                variableValues = new HashMap<>();
            }

            //We iterate by two. Taking row name and variable value in each iteration
            //Loop is safe, since we checked, that parts length is even
            for (int i = 1; i < parts.length; i += 2) {
                String rowName = parts[i];
                String strValue = parts[i + 1];

                try {
                    BigFraction value = StringUtils.parseBigFraction(strValue);

                    if (variableValues.containsKey(rowName)) {
                        throw new MpsParsingException(MpsSections.COLUMNS, "Variable " + variableName + " value already defined for row " + rowName +".", columnsSectionDataLine);
                    }

                    variableValues.put(rowName, value);
                } catch (NumberFormatException e) {
                    throw new MpsParsingException(MpsSections.COLUMNS, "Could not parse rational value" + strValue + " in the input.", columnsSectionDataLine);
                }

            }
            columnsSection.put(variableName, variableValues);
        }

        return columnsSection;
    }

    private Map<String, Map<String, BigFraction>> parseRhs(List<String> prefilteredLines) {
        List<String> rhsLines = getSectionLines(MpsSections.RHS, prefilteredLines);

        Map<String, Map<String, BigFraction>> rhsSection = new HashMap<>();
        for (String rhsSectionLine: rhsLines) {
            String[] parts = rhsSectionLine.split("\\s+");
            //If the parts of the RHS row are even, it cannot be valid
            if (parts.length % 2 != 1) {
                throw new MpsParsingException(MpsSections.RHS, "Invalid RHS section found!", rhsSectionLine);
            }

            String rhsName = parts[0];
            Map<String, BigFraction> rhsValues = rhsSection.get(rhsName);
            if (rhsValues == null) {
                rhsValues = new HashMap<>();
            }

            for (int i = 1; i < parts.length; i += 2) {
                String rowName = parts[i];
                String strValue = parts[i + 1];

                if (rhsValues.containsKey(rowName)) {
                    throw new MpsParsingException(MpsSections.RHS, "Value for row " + rowName +  " is already defined for RHS " + rhsName, rhsSectionLine);
                }

                try {
                    BigFraction value = StringUtils.parseBigFraction(strValue);
                    rhsValues.put(rowName, value);
                } catch (NumberFormatException e) {
                    throw new MpsParsingException(MpsSections.RHS, "Could not parse rational value" + strValue + " in the input.", rhsSectionLine);
                }
            }

            rhsSection.put(rhsName, rhsValues);
        }

        return rhsSection;
    }

    private Map<String, LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>>> parseBounds(List<String> prefilteredLines) {
        List<String> boundsSectionLines = getSectionLines(MpsSections.BOUNDS, prefilteredLines);

        if (boundsSectionLines.isEmpty()) {
            return new HashMap<>(0);
        }

        //Bound name -> variable -> bound type -> value
        Map<String, LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>>> boundSection = new HashMap<>();

        for (String boundsSectionLine: boundsSectionLines) {
            String[] parts = boundsSectionLine.split("\\s+");
            if (parts.length != 4) {
                throw new MpsParsingException(MpsSections.BOUNDS, "Row has invalid number of elements! One bound definition is allowed per line.", boundsSectionLines);
            }

            String boundTypeStr = parts[0];
            String boundName = parts[1];
            String variableName = parts[2];
            String valueStr = parts[3];

            //Variable name -> bound type -> value
            LinkedHashMap<String, LinkedHashMap<BoundType, BigFraction>> boundValues = boundSection.get(boundName);
            if (boundValues == null) {
                boundValues = new LinkedHashMap<>();
            }

            LinkedHashMap<BoundType, BigFraction> variableValueBounds = boundValues.get(variableName);
            if (variableValueBounds == null) {
                variableValueBounds = new LinkedHashMap<>();
            }


            try {
                BoundType boundType = BoundType.valueOf(boundTypeStr);
                BigFraction value = StringUtils.parseBigFraction(valueStr);

                if (variableValueBounds.get(boundType) != null) {
                    throw new MpsParsingException(MpsSections.BOUNDS, "Bound " + boundName + " already has defined " + boundType + " for variable " + variableName, boundsSectionLine);
                }

                variableValueBounds.put(boundType, value);

            } catch (NumberFormatException nfe) {
                throw new MpsParsingException(MpsSections.BOUNDS, "Could not parse rational value" + valueStr + " in the input.", boundsSectionLine);
            } catch (IllegalArgumentException iae) {
                throw new MpsParsingException(MpsSections.BOUNDS, "Invalid bound type found in bound " + boundName + ", allowed values are UP, LO.", boundsSectionLine);
            }

            boundValues.put(variableName, variableValueBounds);
            boundSection.put(boundName, boundValues);
        }

        return boundSection;
    }

    /**
     * Given section type and input lines. Return sublist of original lines containing ordered data of that section
     * in the input.
     * Return data does not contain any tags. Just the raw data lines.
     * @param section
     * @param prefilteredLines
     * @return
     */
    private List<String> getSectionLines(MpsSections section, List<String> prefilteredLines) {
        List<Integer> startIndexes = StringUtils.allIndexesOf(prefilteredLines, section.toString());

        if (startIndexes.isEmpty()) {
            if (section.equals(MpsSections.BOUNDS)) {
                return new ArrayList<>();
            }
            throw new MpsParsingException(section, "No " + section + " section tags found!");
        } else if (startIndexes.size() > 1) {
            throw new MpsParsingException(section, "Multiple " + section + " section beginnings found!");
        }

        Integer startIndex = startIndexes.getFirst();

        int endIndex = findBeginningOfTheNextSection(prefilteredLines, startIndex);

        if (endIndex == -1) {
            throw new MpsParsingException(section, section + " section is not properly ended with another sections beginning!", prefilteredLines.subList(startIndex, prefilteredLines.size()));
        } else if (endIndex == startIndex + 1) {
            //Sublist at endIndex + 1 is safe, since it was previously found in the lines as existing index
            throw new MpsParsingException(section, section + " section appears to be empty!", prefilteredLines.subList(startIndex, endIndex + 1));
        }
        return prefilteredLines.subList(startIndex + 1, endIndex);
    }



    /**
     * Return the index, at which the next section in prefilteredLines starts.
     * Return -1 in case of no such section found.
     * @param prefilteredLines
     * @param startIndex
     * @return
     */
    private int findBeginningOfTheNextSection(List<String> prefilteredLines, int startIndex) {
        int endIndex = -1;

        //Find the beginning of the next section
        for (int i = startIndex + 1; i < prefilteredLines.size(); i++) {
            if (prefilteredLines.get(i).startsWith("OBJSENSE") || MpsUtils.SECTION_NAMES.contains(prefilteredLines.get(i))) {
                endIndex = i;
                break;
            }
        }
        return endIndex;
    }



}