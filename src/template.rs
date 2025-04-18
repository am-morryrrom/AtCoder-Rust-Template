use proconio::input;

#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: i32,
        BC: [i32; 2],
        S: String
    }
    println!("{} {}", A + BC[0] + BC[1], S);
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use cli_test_dir::*;

    const TASK: &'static str = "a";

    #[test]
    fn test_001() {
        test(001);
    }

    #[test]
    fn test_002() {
        test(002);
    }

    fn test(case: i64) {
        let fin = format!("./data/{}/in_{:>03}.txt", TASK, case);
        let fout = format!("./data/{}/out_{:>03}.txt", TASK, case);

        let mut fin = File::open(fin)
            .expect("Input file not found");
        let mut fout = File::open(fout)
            .expect("Output file not found");

        let mut input = String::new();
        let mut output = String::new();
        fin.read_to_string(&mut input)
            .expect("Something went wrong reading input file");
        fout.read_to_string(&mut output)
            .expect("Something went wrong reading output file");

        let testdir = TestDir::new(format!("./{}", TASK).as_str(), "");
        let result = testdir
            .cmd()
            .output_with_stdin(input)
            .tee_output()
            .expect_success();
        assert_eq!(result.stdout_str(), output.replace("\r", ""));
        assert!(result.stderr_str().is_empty());
    }
}