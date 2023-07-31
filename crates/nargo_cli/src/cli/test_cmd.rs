use std::io::Write;

use acvm::{acir::native_types::WitnessMap, Backend};
use clap::Args;
use nargo::{ops::execute_circuit, package::Package};
use noirc_driver::{compile_no_check, CompileOptions};
use noirc_frontend::{graph::CrateName, hir::Context, node_interner::FuncId};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    cli::check_cmd::check_crate_and_report_errors, errors::CliError, find_package_manifest,
    manifest::resolve_workspace_from_toml, prepare_package,
};

use super::{compile_cmd::optimize_circuit, NargoConfig};

/// Run the tests for this program
#[derive(Debug, Clone, Args)]
pub(crate) struct TestCommand {
    /// If given, only tests with names containing this string will be run
    test_name: Option<String>,

    /// Display output of `println` statements
    #[arg(long)]
    show_output: bool,

    /// The name of the package to test
    #[clap(long)]
    package: Option<CrateName>,

    #[clap(flatten)]
    compile_options: CompileOptions,
}

pub(crate) fn run<B: Backend>(
    backend: &B,
    args: TestCommand,
    config: NargoConfig,
) -> Result<(), CliError<B>> {
    let test_name: String = args.test_name.unwrap_or_else(|| "".to_owned());

    let toml_path = find_package_manifest(&config.program_dir)?;
    let workspace = resolve_workspace_from_toml(&toml_path, args.package)?;

    for package in &workspace {
        run_tests(backend, package, &test_name, args.show_output, &args.compile_options)?;
    }

    Ok(())
}

fn run_tests<B: Backend>(
    backend: &B,
    package: &Package,
    test_name: &str,
    show_output: bool,
    compile_options: &CompileOptions,
) -> Result<(), CliError<B>> {
    let (mut context, crate_id) = prepare_package(package);
    check_crate_and_report_errors(&mut context, crate_id, compile_options.deny_warnings)?;

    let test_functions = context.get_all_test_functions_in_crate_matching(&crate_id, test_name);

    println!("[{}] Running {} test functions", package.name, test_functions.len());
    let mut failing = 0;

    let writer = StandardStream::stderr(ColorChoice::Always);
    let mut writer = writer.lock();

    for (test_name, test_function) in test_functions {
        write!(writer, "[{}] Testing {test_name}... ", package.name)
            .expect("Failed to write to stdout");
        writer.flush().expect("Failed to flush writer");

        match run_test(backend, &test_name, test_function, &context, show_output, compile_options) {
            Ok(_) => {
                writer
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .expect("Failed to set color");
                writeln!(writer, "ok").expect("Failed to write to stdout");
            }
            // Assume an error was already printed to stdout
            Err(_) => failing += 1,
        }
        writer.reset().expect("Failed to reset writer");
    }

    if failing == 0 {
        write!(writer, "[{}] ", package.name).expect("Failed to write to stdout");
        writer.set_color(ColorSpec::new().set_fg(Some(Color::Green))).expect("Failed to set color");
        writeln!(writer, "All tests passed").expect("Failed to write to stdout");
    } else {
        let plural = if failing == 1 { "" } else { "s" };
        return Err(CliError::Generic(format!("[{}] {failing} test{plural} failed", package.name)));
    }

    writer.reset().expect("Failed to reset writer");
    Ok(())
}

fn run_test<B: Backend>(
    backend: &B,
    test_name: &str,
    main: FuncId,
    context: &Context,
    show_output: bool,
    config: &CompileOptions,
) -> Result<(), CliError<B>> {
    let mut program = compile_no_check(context, show_output, config, main)
        .map_err(|_| CliError::Generic(format!("Test '{test_name}' failed to compile")))?;
    // Note: We could perform this test using the unoptimized ACIR as generated by `compile_no_check`.
    program.circuit = optimize_circuit(backend, program.circuit).unwrap().0;

    // Run the backend to ensure the PWG evaluates functions like std::hash::pedersen,
    // otherwise constraints involving these expressions will not error.
    match execute_circuit(backend, program.circuit, WitnessMap::new()) {
        Ok(_) => Ok(()),
        Err(error) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let mut writer = writer.lock();
            writer.set_color(ColorSpec::new().set_fg(Some(Color::Red))).ok();
            writeln!(writer, "failed").ok();
            writer.reset().ok();
            Err(error.into())
        }
    }
}
