use itertools::Itertools;

pub(crate) fn indent(string: &str) -> String {
    string
        .lines()
        .map(|line| "\t".to_owned() + line)
        .intersperse("\n".to_owned())
        .collect()
}

#[cfg(test)]
mod test {
    use super::indent;

    #[test]
    fn indent_works() {
        let source = "foo\nbar\n\tbaz";
        assert_eq!(indent(source), "\tfoo\n\tbar\n\t\tbaz");
    }
}
