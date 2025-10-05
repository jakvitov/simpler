
// Module containing methods concerning creating tables for the HtmlOutput

use crate::rationals::Rational;
use super::html_output::HtmlOutput;
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;

impl HtmlOutput {

    /// Add header and data about a basic simplex table to the output
    pub fn add_parsed_basic_simplex_table(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<div class=\"parsed_basic_simplex_table\">\n");
        self.body.push_str("<h2>Parsed Simplex table</h2>\n");
        self.body.push_str("<p>Simplex table parser uses irrelevant bound optimisation.</p>\n");
        if basic_simplex_table.artificial_variable_index.is_some() {
            self.body.push_str("<p>Standard form of the LP was not initially feasible. Artificial variables were added to the initial simplex table and two-phase simplex needs to be used to find initial feasible solution.</p>\n")
        }
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
        self.body.push_str("</div>\n");
    }

    /// Create HTML table from basic simplex table without any markers
    pub (super) fn create_html_table_from_basic_simplex_table(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.create_html_table_form_basic_simplex_table(basic_simplex_table, Vec::new(), None)
    }

    /// Create HTML table from basic simplex table with two rows interaction markers
    /// First numerical row of the basic simplex table has index 0
    /// Objective row has index as last ordinary row + 1 (basic_simplex_table.rows.len())
    pub (super) fn create_html_table_from_basic_simplex_table_with_row_addition_markers(&mut self, basic_simplex_table: &BasicSimplexTable, start: usize, end: usize) {
        self.create_html_table_form_basic_simplex_table(basic_simplex_table, vec![start, end], None);
    }

    /// Create HTML table from basic simplex table with one row marked with marker
    /// First numerical row of the basic simplex table has index 0
    /// Objective row has index as last ordinary row + 1 (basic_simplex_table.rows.len())
    pub (super) fn create_html_table_from_basic_simplex_table_with_one_row_marker(&mut self, basic_simplex_table: &BasicSimplexTable, row: usize) {
        self.create_html_table_form_basic_simplex_table(basic_simplex_table, vec![row], None)
    }

    /// Create HTML table from basic simplex table with one column marked with marker
    /// Column for the first variable has index 0 (basic_simplex_table.rows[0][0])
    /// Last available column has index equal to last variable column (not rhs)
    pub (super) fn create_html_table_from_basic_simplex_table_with_column_marker(&mut self, basic_simplex_table: &BasicSimplexTable, column_marker_index: usize) {
        self.create_html_table_form_basic_simplex_table(basic_simplex_table, Vec::new(), Some(column_marker_index))
    }

    /// Create HTML table from basic simplex table with one column and one row marked with marker
    /// Column for the first variable has index 0 (basic_simplex_table.rows[0][0])
    /// Last available column has index equal to last variable column (not rhs)

    pub (super) fn create_html_table_from_basic_simplex_table_with_row_and_column_marker(&mut self, basic_simplex_table: &BasicSimplexTable, row_marker_index: usize, column_marker_index: usize) {
        self.create_html_table_form_basic_simplex_table(basic_simplex_table, vec![row_marker_index], Some(column_marker_index))
    }

    /// Given vector add it as vertical table to self. If n is bigger than vec length, stretch the table to be n long with empty td.
    pub (super) fn add_vector_with_header_as_vertical_table_with_given_length(&mut self, vec: &Vec<Rational>, n: usize, header: &str) {
        self.body.push_str(self.create_overflowing_table_from_vector(vec, header, n).as_str());
    }

    /// Create html table from basic simplex table
    /// Row  markers - size = 1 - mark given row as target for some action with ←
    ///              - size = 2 - draw arrow from first index to second index row. Indexes in this
    /// vector refer to table number rows, index 0 refers to row with the first base variable
    /// markers go from the first number present to the second. [3,1] adds markers going from fourth row to the second.
    /// !Objective function is treated like one extra row, so it has rows.len() virtual index for marker putting
    /// Column marker - if present mark given column with ↑. Column marker index 0 is first column after base.
    fn create_html_table_form_basic_simplex_table(&mut self, basic_simplex_table: &BasicSimplexTable, row_markers: Vec<usize>, column_marker: Option<usize>) {
        debug_assert!(basic_simplex_table.base_variable_names.len() == basic_simplex_table.rhs.len());
        debug_assert!(basic_simplex_table.rows.len() == basic_simplex_table.base_variable_names.len());
        debug_assert!(row_markers.len() < 3);
        if row_markers.len() == 2 {
            debug_assert!(row_markers[0] != row_markers[1]);
        }
        row_markers.iter().for_each(|index| debug_assert!(*index <= basic_simplex_table.rows.len()));
        if column_marker.is_some() && !basic_simplex_table.rows.is_empty(){
            debug_assert!(column_marker.unwrap() < basic_simplex_table.rows[0].len())
        }

        self.body.push_str("<table>\n");
        //Add the row names
        self.body.push_str("<tr>");
        self.body.push_str("<th>Base</th>");
        for variable_name in basic_simplex_table.column_variable_names.keys() {
            self.body.push_str(format!("<th>{}</th>", variable_name).as_str());
        }
        self.body.push_str("<th>RHS</th>");
        if row_markers.len() > 0 {
            self.body.push_str("<th></th>");
        }
        self.body.push_str("</tr>");

        //Add the base variable and row and rhs value, we add by rows, so base variable needs to be the first element in it
        let mut base_variable_names_iterator = basic_simplex_table.base_variable_names.iter();
        let mut rhs_values_iterator = basic_simplex_table.rhs.iter();

        //Unwraps are safe, RHS and base having all items necessary should be checked in simplex table construction
        for (row_index, row_values) in basic_simplex_table.rows.iter().enumerate() {
            self.body.push_str("<tr>");
            //Fill in the base variable name as the first value
            let base_variable_name = base_variable_names_iterator.next().unwrap();
            self.body.push_str(format!("<td>{}</td>", base_variable_name).as_str());

            for row_value in row_values {
                self.body.push_str(format!("<td>{}</td>", row_value).as_str());
            }
            //Fill in the RHS value
            let rhs_value_for_row = rhs_values_iterator.next().unwrap();
            self.body.push_str(format!("<td>{}</td>", rhs_value_for_row).as_str());

            //Add the row markers if needed
            self.body.push_str(get_row_marker_symbol_for_row(&row_markers, row_index));
            self.body.push_str("</tr>");
        }

        //Fill in the objective function row
        self.body.push_str("<tr>\n");
        self.body.push_str("<td>objective</td>");
        for obj_row_value in &basic_simplex_table.objective_row {
            self.body.push_str(format!("<td>{}</td>", obj_row_value).as_str());
        }
        self.body.push_str(format!("<td>{}</td>", basic_simplex_table.objective_rhs).as_str());
        // The objective row has !virtual! index in the rows as the last one -> rows.len()
        self.body.push_str(get_row_marker_symbol_for_row(&row_markers, basic_simplex_table.rows.len()));
        self.body.push_str("<tr>\n");

        //Fill in the column markers if needed
        if let Some(column_marker_index) = column_marker {
            self.body.push_str("<tr>\n");
            self.body.push_str("<td></td>"); //Base column
            for i in 0..basic_simplex_table.get_column_count_without_rhs_and_base() {
                if i == column_marker_index {
                    self.body.push_str("<td>↑</td>");
                } else {
                    self.body.push_str("<td></td>");
                }
            }
            // In case we already did some row markers, we need to add one extra empty box for
            // the right lower corner
            if !row_markers.is_empty() {
                self.body.push_str("<td></td>")
            }
            self.body.push_str("</tr>\n");
        }

        self.body.push_str("</table>\n")
    }

    /// Create vertical table from given vector, if the given length is bigger than the vector size,
    /// stretch the table adding empty rows to match the length.
    /// Header is not considered as part of the length
    fn create_overflowing_table_from_vector(&self, values: &Vec<Rational>, header: &str, length: usize) -> String {
        let mut res = String::new();
        res.push_str("<table>\n");
        res.push_str(format!("<th>{header}</th>").as_str());
        values.iter().for_each(|value| {res.push_str(format!("<tr><td>{}</td></tr>", value.to_mmdn_with_sign()).as_str());});
        if length > values.len() {
            for i in 0..(length - values.len()) {
                res.push_str("<tr><td>&#8199;</td></tr>");
            }
        }
        res.push_str("</table>\n");
        res
    }
}

/// Return correct row marker symbol for row based on the row index and specified markers
/// If none is marker is required for this row, but markers exist -> return empty <td>
/// If no markers are supplied for the table, we omit adding those tds
fn get_row_marker_symbol_for_row(row_markers: &[usize], row_index: usize) -> &str {
    //Add the row markers if needed
    if row_markers.len() == 2 && row_markers[0] == row_index && row_markers[0] < row_markers[1] {
        "<td>↓</td>"
    }
    else if row_markers.len() == 2 && row_markers[0] == row_index && row_markers[0] > row_markers[1] {
        "<td>↑</td>"
    }
    else if row_markers.len() == 2  && row_markers[1] == row_index && row_markers[0] < row_markers[1] {
        "<td>↲</td>"
    }
    else if row_markers.len() == 2  && row_markers[1] == row_index && row_markers[0] > row_markers[1] {
        "<td>↰</td>"
    }
    else if row_markers.len() == 1 && row_markers[0] == row_index {
        "<td>←</td>"
    }
    else if !row_markers.is_empty() {
        "<td></td>"
    }
    else {
        ""
    }
}


#[cfg(test)]
mod tests {
    use crate::document::html_output::HtmlOutput;

    #[test]
    fn basic_simplex_table_to_html_table_without_markers_succeeds() {
        let simplex_table = crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing();
        let mut document = HtmlOutput::empty();
        document.create_html_table_from_basic_simplex_table(&simplex_table);
        let output = document.to_string();
        let start = output.find("<table>").unwrap();
        let end = output.find("</table>").unwrap() + "</table>".len();
        let table_code  = output[start..end].trim().replace("\n", "");
        assert_eq!(table_code, "<table><tr><th>Base</th><th>x1</th><th>x2</th><th>S1</th><th>S2</th><th>RHS</th></tr><tr><td>S1</td><td>1</td><td>2</td><td>1</td><td>0</td><td>2</td></tr><tr><td>S2</td><td>2</td><td>1</td><td>0</td><td>1</td><td>3</td></tr><tr><td>objective</td><td>-1</td><td>-2</td><td>0</td><td>0</td><td>0</td><tr></table>");
    }

    #[test]
    fn basic_simplex_table_to_html_table_with_row_addition_markers_downwards_succeeds() {
        let simplex_table = crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing();
        let mut document = HtmlOutput::empty();
        document.create_html_table_from_basic_simplex_table_with_row_addition_markers(&simplex_table, 0, 2);
        let output = document.to_string();
        let start = output.find("<table>").unwrap();
        let end = output.find("</table>").unwrap() + "</table>".len();
        let table_code  = output[start..end].trim().replace("\n", "");
        assert_eq!(table_code, "<table><tr><th>Base</th><th>x1</th><th>x2</th><th>S1</th><th>S2</th><th>RHS</th><th></th></tr><tr><td>S1</td><td>1</td><td>2</td><td>1</td><td>0</td><td>2</td><td>↓</td></tr><tr><td>S2</td><td>2</td><td>1</td><td>0</td><td>1</td><td>3</td><td></td></tr><tr><td>objective</td><td>-1</td><td>-2</td><td>0</td><td>0</td><td>0</td><td>↲</td><tr></table>");
    }

    #[test]
    fn basic_simplex_table_to_html_table_with_row_addition_markers_upwards_succeeds() {
        let simplex_table = crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing();
        let mut document = HtmlOutput::with_application_header();
        document.create_html_table_from_basic_simplex_table_with_row_addition_markers(&simplex_table, 2, 0);

        let output = document.to_string();
        let start = output.find("<table>").unwrap();
        let end = output.find("</table>").unwrap() + "</table>".len();
        let table_code  = output[start..end].trim().replace("\n", "");
        assert_eq!(table_code, "<table><tr><th>Base</th><th>x1</th><th>x2</th><th>S1</th><th>S2</th><th>RHS</th><th></th></tr><tr><td>S1</td><td>1</td><td>2</td><td>1</td><td>0</td><td>2</td><td>↰</td></tr><tr><td>S2</td><td>2</td><td>1</td><td>0</td><td>1</td><td>3</td><td></td></tr><tr><td>objective</td><td>-1</td><td>-2</td><td>0</td><td>0</td><td>0</td><td>↑</td><tr></table>");
    }

    #[test]
    fn basic_simple_table_to_html_table_with_column_marker_succeeds() {
        let simplex_table = crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing();
        let mut document = HtmlOutput::with_application_header();
        document.create_html_table_from_basic_simplex_table_with_column_marker(&simplex_table, 2);
        let output = document.to_string();
        let start = output.find("<table>").unwrap();
        let end = output.find("</table>").unwrap() + "</table>".len();
        let table_code  = output[start..end].trim().replace("\n", "");
        assert_eq!(table_code, "<table><tr><th>Base</th><th>x1</th><th>x2</th><th>S1</th><th>S2</th><th>RHS</th></tr><tr><td>S1</td><td>1</td><td>2</td><td>1</td><td>0</td><td>2</td></tr><tr><td>S2</td><td>2</td><td>1</td><td>0</td><td>1</td><td>3</td></tr><tr><td>objective</td><td>-1</td><td>-2</td><td>0</td><td>0</td><td>0</td><tr><tr><td></td><td></td><td></td><td>↑</td><td></td></tr></table>");
    }

    #[test]
    fn basic_simple_table_to_html_table_with_one_row_marker_succeeds() {
        let simplex_table = crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing();
        let mut document = HtmlOutput::with_application_header();
        document.create_html_table_from_basic_simplex_table_with_one_row_marker(&simplex_table, 1);
        let output = document.to_string();
        let start = output.find("<table>").unwrap();
        let end = output.find("</table>").unwrap() + "</table>".len();
        let table_code  = output[start..end].trim().replace("\n", "");
        assert_eq!(table_code, "<table><tr><th>Base</th><th>x1</th><th>x2</th><th>S1</th><th>S2</th><th>RHS</th><th></th></tr><tr><td>S1</td><td>1</td><td>2</td><td>1</td><td>0</td><td>2</td><td></td></tr><tr><td>S2</td><td>2</td><td>1</td><td>0</td><td>1</td><td>3</td><td>←</td></tr><tr><td>objective</td><td>-1</td><td>-2</td><td>0</td><td>0</td><td>0</td><td></td><tr></table>");
    }

    #[test]
    fn t_vector_to_table_suceeds() {
        //todo write this test
    }

}