use nu_test_support::nu;

#[test]
fn test_parse_subexpression_with_range() {
    let actual = nu!(
        cwd: ".",
        r#"
        let foo = 3
        echo (echo 1..$foo | each { $it }) | to json
        "#
    );
    assert_eq!(actual.out, "[1,2,3]")
}

#[test]
fn create_nothing_in_table() {
    let actual = nu!(
        cwd: ".",
        r#"
        echo [[column]; [$nothing]] | to json
        "#
    );
    assert_eq!(actual.out, "{\"column\":null}");
}

#[test]
fn compare_to_nothing() {
    let actual = nu!(
        cwd: ".",
        r#"
        let f = $nothing
        if $f == $nothing {echo $true} {echo $false}
        "#
    );
    assert_eq!(actual.out, "true");
}

#[test]
fn compare_nothing_and_boolean() {
    let actual = nu!(
        cwd: ".",
        r#"
        if $true == $nothing {echo $true} {echo $false}
        "#
    );
    assert_eq!(actual.out, "false");
    let actual = nu!(
        cwd: ".",
        r#"
        if $false == $nothing {echo $true} {echo $false}
        "#
    );
    assert_eq!(actual.out, "false");
}
