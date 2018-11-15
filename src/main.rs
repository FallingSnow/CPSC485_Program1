#![feature(vec_resize_with, vec_resize_default, self_struct_ctor)]

use console::Term;
use std::cmp::min;
use std::io::Write;

type Result<T> = core::result::Result<T, std::io::Error>;

#[derive(Debug)]
struct TwoDMatrix(Vec<Vec<usize>>);

impl TwoDMatrix {
    fn new(x: usize, y: usize) -> Self {
        let mut matrix: Vec<Vec<usize>> = vec![];

        matrix.resize_with(x, || {
            let mut vec = vec![];
            vec.resize_default(y);
            vec
        });

        Self(matrix)
    }
}

impl std::fmt::Display for TwoDMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "     ")?;
        for i in 0..self.0.len() {
            write!(f, "{:^6}", i)?;
        }
        writeln!(f)?;

        for i in 0..self.0.len() {
            writeln!(f, "     {}", "-".repeat(6 * self.0[i].len()))?;
            write!(f, "  {} |", i)?;
            for j in 0..self.0[i].len() {
                write!(f, " {:3} :", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "     {}", "-".repeat(6 * self.0[0].len()))?;
        Ok(())
    }
}

fn edit_distance_matrix(st1: &String, st2: &String) -> Result<TwoDMatrix> {
    let mut matrix = TwoDMatrix::new(st1.len() + 1, st2.len() + 1);

    let inner = &mut matrix.0;

    for i in 0..inner.len() {
        for j in 0..inner[i].len() {
            inner[i][j] = {
                if i == 0 {
                    j
                } else if j == 0 {
                    i
                } else if st1.chars().nth(i - 1).unwrap() == st2.chars().nth(j - 1).unwrap() {
                    inner[i - 1][j - 1]
                } else {
                    min(min(inner[i][j - 1], inner[i - 1][j - 1]), inner[i - 1][j]) + 1
                }
            }
        }
    }

    Ok(matrix)
}

fn alignment(st1: &String, st2: &String) -> Result<(String, String)> {
    let (longer, shorter) = match st1.len() <= st2.len() {
        false => (st1, st2),
        true => (st2, st1),
    };

    let out1 = longer.clone();
    let mut out2 = String::default();

    let mut alignment = 0;
    let mut len_diff = longer.len() - shorter.len();

    for i in 0..longer.len() {
        if len_diff > 0 && (longer.chars().nth(i - alignment) != shorter.chars().nth(i)) {
            out2.push_str("_");
            len_diff -= 1;
            alignment += 1;
        } else {
            out2.push(shorter.chars().nth(i - alignment).unwrap());
        }
    }

    Ok((out1, out2))
}

fn main() -> Result<()> {
    let mut term = Term::stdout();

    term.write(b"Please enter first string: ")?;
    let string1 = term.read_line()?;

    term.write(b"Please enter second string: ")?;
    let string2 = term.read_line()?;
    term.write_line("")?;

    let distance_matrix = edit_distance_matrix(&string1, &string2)?;

    term.write_line("The matrix:")?;
    term.write_line("")?;
    term.write_line(&format!("{}", distance_matrix))?;
    term.write_line("")?;

    term.write_line(&format!(
        "The edit distance is: {}",
        distance_matrix.0[string1.len()][string2.len()]
    ))?;
    term.write_line("")?;

    term.write_line("Alignment is:")?;

    let (al1, al2) = alignment(&string1, &string2)?;
    term.write_line(&al1)?;
    term.write_line(&al2)?;

    term.write_line("")?;

    Ok(())
}
