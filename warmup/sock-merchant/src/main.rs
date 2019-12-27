use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
enum InputReadError<Value>
where
    Value: FromStr + 'static,
    <Value as FromStr>::Err: Debug,
{
    Io(std::io::Error),
    Utf8Parse(std::string::FromUtf8Error),
    LenParse(std::num::ParseIntError),
    ValueParse(<Value as FromStr>::Err),
}

fn read_input<Value: std::str::FromStr, R: Read>(
    reader: &mut R,
) -> Result<Box<[Value]>, InputReadError<Value>>
where
    Value: FromStr + 'static,
    <Value as FromStr>::Err: Debug,
{
    let mut buf_reader = BufReader::new(reader);
    let mut len_read_buffer = Vec::with_capacity(8);

    buf_reader
        .read_until(b'\n', &mut len_read_buffer)
        .map_err(InputReadError::Io)?;

    let len = String::from_utf8(len_read_buffer)
        .map_err(InputReadError::Utf8Parse)?
        .trim()
        .parse::<usize>()
        .map_err(InputReadError::LenParse)?;

    (0..len)
        .map(|_| -> Result<Value, InputReadError<Value>> {
            let mut value_buffer = Vec::with_capacity(8);
            buf_reader
                .read_until(b' ', &mut value_buffer)
                .map_err(InputReadError::Io)?;

            String::from_utf8(value_buffer)
                .map_err(InputReadError::Utf8Parse)?
                .trim()
                .parse::<Value>()
                .map_err(InputReadError::ValueParse)
        })
        .collect::<Result<Vec<Value>, InputReadError<Value>>>()
        .map(Vec::into_boxed_slice)
}

fn count_pairs(socks: &[i32]) -> usize {
    let mut by_colors = BTreeMap::<i32, usize>::new();
    socks.iter().for_each(|v| {
        let val: &mut usize = by_colors.entry(*v).or_insert(0);
        *val += 1;
    });

    by_colors.values().map(|v| v / 2).sum()
}

fn main() {
    let mut stdin = std::io::stdin();
    let socks: Box<[i32]> = read_input(&mut stdin).unwrap();

    print!("{}", count_pairs(&socks));
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[test]
    fn sock_merchant_example_data() {
        let mut input = File::open("data/test-input.txt").unwrap();
        let socks: Box<[i32]> = read_input(&mut input).unwrap();

        assert_eq!(count_pairs(&socks), 3);
    }
}
