package com.github.jakvitov.mps;

import java.util.Set;

public class MpsUtils {

    public static Set<String> SECTION_NAMES = Set.of(MpsSections.NAME.toString(), MpsSections.ROWS.toString(), MpsSections.COLUMNS.toString(), MpsSections.RHS.toString(), MpsSections.OBJSENSE.toString(), MpsSections.BOUNDS.toString(), MpsSections.ENDATA.toString());

}
