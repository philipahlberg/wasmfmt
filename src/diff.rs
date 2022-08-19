use std::fmt;

/// Created from two different strings.
/// Presents the difference when formatted.
pub struct Diff<'src, 'fmt> {
    source: &'src str,
    formatted: &'fmt str,
}

impl<'src, 'fmt> Diff<'src, 'fmt> {
    pub fn from(source: &'src str, formatted: &'fmt str) -> Option<Self> {
        if source != formatted {
            Some(Self { source, formatted })
        } else {
            None
        }
    }
}

impl<'src, 'fmt> fmt::Display for Diff<'src, 'fmt> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Difference found.\n")?;
        f.write_str("Source:\n")?;
        f.write_str(self.source)?;
        f.write_str("\n")?;
        f.write_str("Formatted:\n")?;
        f.write_str(self.formatted)
    }
}

#[cfg(test)]
mod test {
    use super::Diff;
    use unindent::Unindent;

    #[test]
    fn check_should_return_none_for_no_diff() {
        let result = Diff::from("foo", "foo");
        assert!(matches!(result, None));
    }

    #[test]
    fn check_should_return_some_for_some_diff() {
        let result = Diff::from("foo", "bar");
        assert!(matches!(result, Some(..)));
    }

    #[test]
    fn diff_should_display_source_and_formatted() {
        let diff = Diff::from("foo", "bar").unwrap();
        let result = format!("{}", diff);
        assert_eq!(
            result,
            "
            Difference found.
            Source:
            foo
            Formatted:
            bar
            "
            .trim()
            .unindent()
        );
    }
}
