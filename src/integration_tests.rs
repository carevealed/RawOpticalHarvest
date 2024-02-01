#[cfg(test)]
mod tests
{
    use assert_cmd::Command;

    // Disabled for interactivity
    // #[test]
    // fn test_cahuca_dry()
    // {
    //     let mut cmd = Command::cargo_bin("carroh").unwrap();
    //     let assert = cmd
    //         .arg("-d")
    //         .arg("demo/cahuca.csv")
    //         .arg("demo/out")
    //         .arg("ram0")
    //         .assert();

    //     assert.success();
    // }

    // #[test]
    // fn test_casfjazz_dry()
    // {
    //     let mut cmd = Command::cargo_bin("carroh").unwrap();
    //     let assert = cmd
    //         .arg("-d")
    //         .arg("demo/casfjazz.csv")
    //         .arg("demo/out")
    //         .arg("ram0")
    //         .assert();

    //     assert.success();
    // }

    #[test]
    fn test_non_existent_csv()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert =
            cmd.arg("demo/does_not_exist.csv").arg("demo/out").assert();

        assert.failure().stderr(
            r#"Error: "\"demo/does_not_exist.csv\" could not be found, but is expected to exist."
"#,
        );
    }
    #[test]
    fn test_single_argument()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert = cmd.arg("demo/simple_column_tester.csv").assert();

        assert.failure().stderr(
            r#"error: the following required arguments were not provided:
  <Output Parent Directory>

Usage: carroh <Input CSV> <Output Parent Directory> [ROM Device]

For more information, try '--help'.
"#,
        );
    }

    // Disabled for interactivity
    // #[test]
    // fn test_filesystem()
    // {
    //     let mut cmd = Command::cargo_bin("carroh").unwrap();
    //     let cmd_res = cmd
    //         .arg("demo/file with spaces.csv")
    //         .arg("demo/out").output();

    //     fs::remove_dir_all("demo/out/1_1").unwrap();

    //     match cmd_res {
    //         Ok(_) => {},
    //         Err(e) => panic!("{e:?}"),
    //     }

    // }

    #[test]
    fn test_existing_output()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert = cmd.arg("demo/out_exists.csv").arg("demo/out").assert();

        assert.failure().stderr(r#"Error: "\"demo/out/1_exists\" should not already exist, but does."
"#);
    }

    #[test]
    fn test_help()
    {
        let mut cmd = Command::cargo_bin("carroh").unwrap();
        let assert = cmd.arg("-h").assert();

        assert.success().stdout(
            r#"California Revealed Raw Optical Harvest

Usage: carroh [OPTIONS] <Input CSV> <Output Parent Directory> [ROM Device]

Arguments:
  <Input CSV>                Path to the CSV file we want to process
  <Output Parent Directory>  Output parent directory
  [ROM Device]               Device to use as ISO generation source.  If none is provided, the user will be prompted to select a device

Options:
  -d, --dry-run     Don't actually create or modify any files
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help (see more with '--help')
  -V, --version     Print version
"#,
        );
    }
}
