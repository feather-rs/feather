use anyhow::{anyhow, Result};
use std::fmt::Write;
use std::{fmt::Display, str::CharIndices};

pub trait Sprintf {
    fn fmt<W: Write>(&self, output: &mut W, args: &[&dyn Display]) -> Result<()>;
}

impl Sprintf for str {
    fn fmt<W: Write>(&self, output: &mut W, args: &[&dyn Display]) -> Result<()> {
        fn fmt<W: Write>(code: char, output: &mut W, arg: &dyn Display) -> Result<()> {
            match code {
                's' | 'd' | 'i' => Ok(write!(output, "{}", arg)?),
                _ => Err(anyhow!("Unexpected format code: {}", code)),
            }
        }

        fn get_arg<T>(args: &[T], index: usize) -> Result<&T> {
            args.get(index)
                .ok_or(anyhow!("Argument does not exist: {}", index))
        }

        fn get_code(chars: &mut CharIndices) -> Result<(usize, char)> {
            chars
                .next()
                .ok_or(anyhow!("Excepted format code, recived nothing"))
        }

        let chars = &mut self.char_indices();
        let mut index = 0;

        while let Some((_, c)) = chars.next() {
            match c {
                '%' => {
                    let (start, code) = get_code(chars)?;
                    match code {
                        '%' => output.write_char('%')?,
                        '0'..='9' => {
                            let (end, end_c) = chars
                                .find(|(_, c)| *c == '$')
                                .ok_or(anyhow!("Expected '$' at the end of index code"))?;
                            let index: usize = self[start..end + end_c.len_utf8()].parse()?;
                            let (_, code) = get_code(chars)?;
                            fmt(code, output, get_arg(args, index)?)?;
                        }
                        code => {
                            fmt(code, output, get_arg(args, index)?)?;
                            index += 1;
                        }
                    }
                }
                c => output.write_char(c)?,
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! sprintf {
    ($output:expr, $format:expr $(,$args:expr)*$(,)?) => {
        ::text::Sprintf::fmt($format, $output, &[$(&$args),*])
    };
}

#[cfg(test)]
mod tests {
    use crate::sprintf;
    use anyhow::Result;

    #[test]
    fn escape() -> Result<()> {
        let mut output = String::new();
        sprintf!(&mut output, "%%%%%s", "hello world!")?;
        assert_eq!(output, "%%hello world!");
        Ok(())
    }
}
