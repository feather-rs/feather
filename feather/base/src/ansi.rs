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

    fn format(&self, color: u8) -> String {
        format!("\x1b[{};{}m", self.format, color)
    }

    pub fn black(&self) -> String {
        self.format(30)
    }

    pub fn red(&self) -> String {
        self.format(31)
    }

    pub fn green(&self) -> String {
        self.format(32)
    }

    pub fn yellow(&self) -> String {
        self.format(33)
    }

    pub fn blue(&self) -> String {
        self.format(34)
    }

    pub fn magenta(&self) -> String {
        self.format(35)
    }

    pub fn cyan(&self) -> String {
        self.format(36)
    }

    pub fn white(&self) -> String {
        self.format(37)
    }

    pub fn reset(&self) -> &'static str {
        "\x1b[0m"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_style() {
        let style = AnsiStyle::regular();
        assert_eq!(style.black(), "\x1b[0;30m");
        assert_eq!(style.red(), "\x1b[0;31m");
        assert_eq!(style.green(), "\x1b[0;32m");
        assert_eq!(style.yellow(), "\x1b[0;33m");
        assert_eq!(style.blue(), "\x1b[0;34m");
        assert_eq!(style.magenta(), "\x1b[0;35m");
        assert_eq!(style.cyan(), "\x1b[0;36m");
        assert_eq!(style.white(), "\x1b[0;37m");
        assert_eq!(style.reset(), "\x1b[0m");

        let style = AnsiStyle::bold();
        assert_eq!(style.black(), "\x1b[1;30m");
        assert_eq!(style.red(), "\x1b[1;31m");
        assert_eq!(style.green(), "\x1b[1;32m");
        assert_eq!(style.yellow(), "\x1b[1;33m");
        assert_eq!(style.blue(), "\x1b[1;34m");
        assert_eq!(style.magenta(), "\x1b[1;35m");
        assert_eq!(style.cyan(), "\x1b[1;36m");
        assert_eq!(style.white(), "\x1b[1;37m");
        assert_eq!(style.reset(), "\x1b[0m");

        let style = AnsiStyle::underline();
        assert_eq!(style.black(), "\x1b[4;30m");
        assert_eq!(style.red(), "\x1b[4;31m");
        assert_eq!(style.green(), "\x1b[4;32m");
        assert_eq!(style.yellow(), "\x1b[4;33m");
        assert_eq!(style.blue(), "\x1b[4;34m");
        assert_eq!(style.magenta(), "\x1b[4;35m");
        assert_eq!(style.cyan(), "\x1b[4;36m");
        assert_eq!(style.white(), "\x1b[4;37m");
        assert_eq!(style.reset(), "\x1b[0m");
    }
}
