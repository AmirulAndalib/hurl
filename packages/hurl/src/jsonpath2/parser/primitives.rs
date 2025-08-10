/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use hurl_core::reader::Reader;

use super::{ParseError, ParseErrorKind, ParseResult};

pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<()> {
    // does not return a value
    // non recoverable reader
    // => use combinator recover to make it recoverable
    let start = reader.cursor();
    if reader.is_eof() {
        let kind = ParseErrorKind::Expecting(s.to_string());
        let error = ParseError::new(start.pos, kind);
        return Err(error);
    }
    for c in s.chars() {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::Expecting(s.to_string());
                let error = ParseError::new(start.pos, kind);
                return Err(error);
            }
            Some(x) => {
                if x != c {
                    let kind = ParseErrorKind::Expecting(s.to_string());
                    let error = ParseError::new(start.pos, kind);
                    return Err(error);
                } else {
                    continue;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::{CharPos, Pos};

    use super::*;

    #[test]
    fn test_literal() {
        let mut reader = Reader::new("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("hello ");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(0));

        let mut reader = Reader::new("hi");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );

        assert_eq!(reader.cursor().index, CharPos(2));

        let mut reader = Reader::new("he");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(2));
    }
}
