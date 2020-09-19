#[macro_use]
mod grass_macros;

grass_test!(
    basic_toplevel,
    "@media foo {\n  a {\n    color: red;\n  }\n}\n"
);
grass_error!(
    no_params,
    "@media {\n  a {\n    color: red;\n  }\n}\n",
    "Error: Expected identifier."
);
grass_test!(
    basic_nested,
    "a {\n  @media foo {\n  color: red;\n  }\n}\n",
    "@media foo {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(empty_body, "@media (min-width: 2px) {}", "");
grass_test!(
    newlines_are_not_emitted_for_child_styles,
    "a {
        @media screen {
            b {
                color: red;
            }
            c {
                color: green;
            }
        }
    }",
    "@media screen {\n  a b {\n    color: red;\n  }\n  a c {\n    color: green;\n  }\n}\n"
);
grass_test!(
    multiple_identifiers_in_query,
    "@media not screen {
        a {
            color: red;
        }
    }",
    "@media not screen {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(
    multiple_identifiers_in_query_second_is_and,
    "@media print and (foo: 1 2 3) {
        a {
            color: red;
        }
    }",
    "@media print and (foo: 1 2 3) {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(
    single_identifier_inside_parens,
    "@media (color) {a {color: red;}}",
    "@media (color) {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(
    quoted_colon_in_parens,
    "@media screen and (\":\") {
        a {
            color: red;
        }
    }",
    "@media screen and (:) {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(
    multiline_comments_everywhere,
    "@media/**/foo/**/and/**/(/**/bar/**/)/**/{
        a {
            color: red;
        }
    }",
    "@media foo and (bar) {\n  a {\n    color: red;\n  }\n}\n"
);
grass_test!(
    comparison_in_query,
    "@media (100px < 400px) {
        a {
            interpolation: in-parens
        }
    }",
    "@media (100px < 400px) {\n  a {\n    interpolation: in-parens;\n  }\n}\n"
);
grass_test!(
    interpolated_comparison_in_query,
    "@media (#{100px < 400px}) {
        a {
            interpolation: in-parens
        }
    }",
    "@media (true) {\n  a {\n    interpolation: in-parens;\n  }\n}\n"
);
grass_test!(
    single_eq_in_query,
    "@media (height=600px) {
        a {
            b: c
        }
    }
    ",
    "@media (height = 600px) {\n  a {\n    b: c;\n  }\n}\n"
);
grass_test!(
    double_eq_in_query,
    "@media (height==600px) {
        a {
            b: c
        }
    }
    ",
    "@media (false) {\n  a {\n    b: c;\n  }\n}\n"
);
grass_error!(
    media_feature_missing_closing_paren,
    "@media foo and (bar:a",
    "Error: expected \")\"."
);
grass_error!(
    media_feature_missing_curly_brace_after_hash,
    "@media foo and # {}",
    "Error: expected \"{\"."
);
