pub trait HtmlConvertibleError {
    fn to_html_string(&self) -> String;

    fn get_error_name(&self) -> String;
}