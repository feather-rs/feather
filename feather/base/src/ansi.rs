pub struct AnsiStyle {
    format: u8,
}

impl AnsiStyle {
    pub fn regular() -> AnsiStyle {
        AnsiStyle { format: 0 }
    }

    pub fn bold() -> AnsiStyle {
        AnsiStyle { format: 1 }
    }

    pub fn underline() -> AnsiStyle {
        AnsiStyle { format: 4 }
    }

    fn format(&self, color: u8) -> String { format!("\033[{};{}m", self.format, color) }

    pub fn black(&self) -> String { self.format(30) }

    pub fn red(&self) -> String { self.format(31) }

    pub fn green(&self) -> String { self.format(32) }

    pub fn yellow(&self) -> String { self.format(33) }

    pub fn blue(&self) -> String { self.format(34) }

    pub fn magenta(&self) -> String { self.format(35) }

    pub fn cyan(&self) -> String { self.format(36) }

    pub fn white(&self) -> String { self.format(37) }

    pub fn reset(&self) -> &'static str { "\033[0m" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_style() {
        let style = AnsiStyle::regular();
        assert_eq!(style.black(), "\033[0;30m");
        assert_eq!(style.red(), "\033[0;31m");
        assert_eq!(style.green(), "\033[0;32m");
        assert_eq!(style.yellow(), "\033[0;33m");
        assert_eq!(style.blue(), "\033[0;34m");
        assert_eq!(style.magenta(), "\033[0;35m");
        assert_eq!(style.cyan(), "\033[0;36m");
        assert_eq!(style.white(), "\033[0;37m");
        assert_eq!(style.reset(), "\033[0m");

        let style = AnsiStyle::bold();
        assert_eq!(style.black(), "\033[1;30m");
        assert_eq!(style.red(), "\033[1;31m");
        assert_eq!(style.green(), "\033[1;32m");
        assert_eq!(style.yellow(), "\033[1;33m");
        assert_eq!(style.blue(), "\033[1;34m");
        assert_eq!(style.magenta(), "\033[1;35m");
        assert_eq!(style.cyan(), "\033[1;36m");
        assert_eq!(style.white(), "\033[1;37m");
        assert_eq!(style.reset(), "\033[0m");

        let style = AnsiStyle::underline();
        assert_eq!(style.black(), "\033[4;30m");
        assert_eq!(style.red(), "\033[4;31m");
        assert_eq!(style.green(), "\033[4;32m");
        assert_eq!(style.yellow(), "\033[4;33m");
        assert_eq!(style.blue(), "\033[4;34m");
        assert_eq!(style.magenta(), "\033[4;35m");
        assert_eq!(style.cyan(), "\033[4;36m");
        assert_eq!(style.white(), "\033[4;37m");
        assert_eq!(style.reset(), "\033[0m");
    }
}