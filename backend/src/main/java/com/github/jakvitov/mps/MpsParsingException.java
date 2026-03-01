package com.github.jakvitov.mps;

import java.util.ArrayList;
import java.util.List;

/**
 * Exception indicating, that error occurred during MPS parsing
 */
public class MpsParsingException extends RuntimeException{

    public String[] reasons;

    public MpsParsingException(String[] reasons) {
        super(reasons.length > 0 ? reasons[0] : "Unknown MPS parsing error occurred.");
        this.reasons = reasons;
    }

    public MpsParsingException(List<String> reasons) {
        super(!reasons.isEmpty() ? reasons.getFirst() : "Unknown MPS parsing error occurred.");
        //Todo check this classcast later
        this.reasons = reasons.toArray(new String[0]);
    }

    public MpsParsingException(MpsSections section, String title, List<String> problemLines) {
        List<String> reasons = new ArrayList<>(4 + problemLines.size());
        reasons.add("Error occurred during parsing of the MPS input.");
        reasons.add("Failing section: " + section.toString());
        reasons.add(title);
        reasons.add("Problem data: ");
        reasons.addAll(problemLines);
        this.reasons = reasons.toArray(new String[0]);
    }

    public MpsParsingException(MpsSections section, String title, String problemLine) {
        List<String> reasons = new ArrayList<>(4);
        reasons.add("Error occurred during parsing of the MPS input.");
        reasons.add("Failing section: " + section.toString());
        reasons.add(title);
        reasons.add("Problem data: ");
        reasons.add(problemLine);
        this.reasons = reasons.toArray(new String[0]);
    }

    public MpsParsingException(MpsSections section, String title) {
        List<String> reasons = new ArrayList<>(3);
        reasons.add("Error occurred during parsing of the MPS input.");
        reasons.add("Failing section: " + section.toString());
        reasons.add(title);
        this.reasons = reasons.toArray(new String[0]);
    }

}
