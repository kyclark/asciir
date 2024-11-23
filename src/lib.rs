use anyhow::{bail, Result};
use clap::{Args, Parser};
use transpose::transpose;

#[derive(Debug, Parser)]
#[command(version, about)]
/// Print ASCII table/values
pub struct Cli {
    #[command(flatten)]
    mode: Mode,

    /// Input value(s)
    #[arg(value_name = "VAL")]
    values: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[group(multiple = false)]
struct Mode {
    /// Binary mode
    #[arg(short, long)]
    binary: bool,

    /// Hexadecimal mode
    #[arg(short = 'x', long)]
    hexadecimal: bool,
}

#[derive(Default, Clone, Copy)]
enum Base {
    Binary,
    #[default]
    Decimal,
    Hexadecimal,
}

#[derive(Debug, PartialEq)]
struct Convert {
    codepoint: u8,
    character: char,
}

// --------------------------------------------------
pub fn run(args: Cli) -> Result<()> {
    let base = if args.mode.binary {
        Base::Binary
    } else if args.mode.hexadecimal {
        Base::Hexadecimal
    } else {
        Base::Decimal
    };
    let values = args.values;
    if values.is_empty() {
        print_table(base)
    } else {
        for val in values {
            match convert(&val, base) {
                Ok(translated) => {
                    let show = if val == translated.character.to_string() {
                        match base {
                            Base::Binary => format!("{:07b}", translated.codepoint),
                            Base::Decimal => format!("{:3}", translated.codepoint),
                            Base::Hexadecimal => format!("{:02x}", translated.codepoint),
                        }
                    } else {
                        translated.character.to_string()
                    };
                    println!("{val:>3} = {show}");
                }
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn convert(val: &str, base: Base) -> Result<Convert> {
    let radix = match base {
        Base::Binary => 2,
        Base::Decimal => 10,
        Base::Hexadecimal => 16,
    };
    match u8::from_str_radix(val, radix) {
        Ok(codepoint) => {
            if (33..127).contains(&codepoint) {
                Ok(Convert {
                    codepoint,
                    character: codepoint as char,
                })
            } else {
                bail!("Codepoint {codepoint} not in the range 33-126")
            }
        }
        _ => {
            let chars: Vec<_> = val.chars().collect();
            if chars.len() == 1 {
                let character = *chars.first().unwrap();
                if character.is_ascii() {
                    Ok(Convert {
                        codepoint: character as u8,
                        character,
                    })
                } else {
                    bail!("{val} is not an ASCII value")
                }
            } else {
                bail!(r#"Input "{val}" must be a single character"#)
            }
        }
    }
}

// --------------------------------------------------
fn ascii_table(base: Base) -> Vec<String> {
    let range: Vec<u32> = (33..=127).collect();
    let mut nums = vec![0; 95];
    transpose(&range, &mut nums, 19, 5);
    let vals: Vec<String> = nums
        .iter()
        .map(|&i| {
            let val = if i == 127 {
                "DEL".to_string()
            } else {
                std::char::from_u32(i).unwrap().to_string()
            };
            match base {
                Base::Binary => format!("{i:07b}: {}", val),
                Base::Decimal => format!("{i:3}: {}", val),
                Base::Hexadecimal => format!("{i:02x}: {}", val),
            }
        })
        .collect();

    vals.chunks(5).map(|v| v.join("  ")).collect()
}

// --------------------------------------------------
fn print_table(base: Base) {
    println!("{}", ascii_table(base).join("\n"))
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{ascii_table, convert, Base, Convert};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let res = convert("0", Base::Decimal);
        assert!(res.is_err());

        let res = convert("127", Base::Decimal);
        assert!(res.is_err());

        let res = convert("256", Base::Decimal);
        assert!(res.is_err());

        let res = convert("", Base::Decimal);
        assert!(res.is_err());

        let res = convert("😁", Base::Decimal);
        assert!(res.is_err());

        let res = convert("33", Base::Decimal);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Convert {
                codepoint: 33,
                character: '!'
            }
        );

        let res = convert("!", Base::Decimal);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Convert {
                codepoint: 33,
                character: '!'
            }
        );

        let res = convert("126", Base::Decimal);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Convert {
                codepoint: 126,
                character: '~'
            }
        );

        let res = convert("~", Base::Decimal);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Convert {
                codepoint: 126,
                character: '~'
            }
        );
    }

    #[test]
    fn test_ascii_table() {
        let table = vec![
            r##" 33: !   52: 4   71: G   90: Z  109: m"##,
            r##" 34: "   53: 5   72: H   91: [  110: n"##,
            r##" 35: #   54: 6   73: I   92: \  111: o"##,
            r##" 36: $   55: 7   74: J   93: ]  112: p"##,
            r##" 37: %   56: 8   75: K   94: ^  113: q"##,
            r##" 38: &   57: 9   76: L   95: _  114: r"##,
            r##" 39: '   58: :   77: M   96: `  115: s"##,
            r##" 40: (   59: ;   78: N   97: a  116: t"##,
            r##" 41: )   60: <   79: O   98: b  117: u"##,
            r##" 42: *   61: =   80: P   99: c  118: v"##,
            r##" 43: +   62: >   81: Q  100: d  119: w"##,
            r##" 44: ,   63: ?   82: R  101: e  120: x"##,
            r##" 45: -   64: @   83: S  102: f  121: y"##,
            r##" 46: .   65: A   84: T  103: g  122: z"##,
            r##" 47: /   66: B   85: U  104: h  123: {"##,
            r##" 48: 0   67: C   86: V  105: i  124: |"##,
            r##" 49: 1   68: D   87: W  106: j  125: }"##,
            r##" 50: 2   69: E   88: X  107: k  126: ~"##,
            r##" 51: 3   70: F   89: Y  108: l  127: DEL"##,
        ];
        assert_eq!(ascii_table(Base::Decimal), table);
    }
}
