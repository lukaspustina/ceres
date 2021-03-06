use clap::{App, Arg, ArgMatches, SubCommand};
use centerdevice::CenterDevice;
use centerdevice::client::AuthorizedClient;
use centerdevice::client::search::{Document, NamedSearch, Search, SearchResult};
use failure::Fail;
use std::convert::TryInto;
use std::collections::HashMap;

use config::{CeresConfig as Config};
use run_config::RunConfig;
use modules::{Result as ModuleResult, Error as ModuleError, ErrorKind as ModuleErrorKind, Module};
use modules::centerdevice::AuthorizedClientExt;
use modules::centerdevice::errors::*;
use output::OutputType;
use output::centerdevice::search::*;

pub const NAME: &str = "search";

pub struct SubModule;

impl Module for SubModule {
    fn build_sub_cli() -> App<'static, 'static> {
        SubCommand::with_name(NAME)
            .about("Search documents in CenterDevice")
            .arg(Arg::with_name("filename")
                .long("filename")
                .short("f")
                .takes_value(true)
                .multiple(true)
                .help("Adds filename to search"))
            .arg(Arg::with_name("tags")
                .long("tag")
                .short("t")
                .takes_value(true)
                .multiple(true)
                .help("Adds tag to search"))
            .arg(Arg::with_name("public_collections")
                .long("public-collections")
                .short("p")
                .help("Includes public collections in search"))
            .arg(Arg::with_name("fulltext")
                .index(1)
                .multiple(true)
                .help("Adds fulltext to search"))
            .arg(Arg::with_name("resolve-ids")
                .long("resolve-ids")
                .short("R")
                .help("Resolves ids"))
            .arg(Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .default_value("human")
                .possible_values(&["human", "json", "plain"])
                .help("Selects output format"))
    }

    fn call(cli_args: Option<&ArgMatches>, run_config: &RunConfig, config: &Config) -> ModuleResult<()> {
        let args = cli_args.unwrap(); // Safe unwrap
        do_call(args, run_config, config)
            .map_err(|e| ModuleError::with_chain(e, ModuleErrorKind::ModuleFailed(NAME.to_owned())))
    }
}

fn do_call(args: &ArgMatches, run_config: &RunConfig, config: &Config) -> Result<()> {
    let profile = match run_config.active_profile.as_ref() {
        "default" => config.get_default_profile(),
        s => config.get_profile(s),
    }.chain_err(|| ErrorKind::FailedToParseCmd("profile".to_string()))?;
    let centerdevice = profile.centerdevice.as_ref().ok_or_else(
        || Error::from_kind(ErrorKind::NoCenterDeviceInProfile)
    )?;

    let output_type = args.value_of("output").unwrap() // Safe
        .parse::<OutputType>()
        .chain_err(|| ErrorKind::FailedToParseOutputType)?;

    let fulltext_str; // Borrow checker
    let mut search = Search::new();
    if let Some(filenames) = args.values_of("filenames") {
        search = search.filenames(filenames.collect());
    }
    if let Some(tags) = args.values_of("tags") {
        search = search.tags(tags.collect());
    }
    if args.is_present("public_collections") {
        search = search.named_searches(NamedSearch::PublicCollections);
    }
    if let Some(fulltext) = args.values_of("fulltext") {
        let fulltext: Vec<_> = fulltext.collect();
        fulltext_str = fulltext.as_slice().join(" ");
        search = search.fulltext(&fulltext_str);
    }
    debug!("{:#?}", search);

    let client: AuthorizedClient = centerdevice.try_into()?;
    info!("Searching documents at {}.", centerdevice.base_domain);
    let result = search_documents(&client, search)?;
    info!("Successfully found {} and retrieved {} documents.", result.hits, result.documents.len());

    if args.is_present("resolve-ids") {
        info!("Retrieving users from {}.", centerdevice.base_domain);
        let user_map = client.get_user_map()?;
        info!("Outputting search results with resolved ids");
        output_results(output_type, &result.documents, Some(&user_map))?;
    } else {
        info!("Outputting search results");
        output_results(output_type, &result.documents, None)?;
    }

    Ok(())
}

fn search_documents(client: &AuthorizedClient, search: Search) -> Result<SearchResult> {
    let result = client
        .search_documents(search)
        .map_err(|e| Error::with_chain(e.compat(), ErrorKind::FailedToAccessCenterDeviceApi));
    debug!("Search result {:#?}", result);

    result
}

fn output_results(output_type: OutputType, results: &[Document], user_map: Option<&HashMap<String, String>>) -> Result<()> {
    let mut stdout = ::std::io::stdout();

    match output_type {
        OutputType::Human => {
            let output = TableOutputSearchResult { user_map };

            output
                .output(&mut stdout, results)
                .chain_err(|| ErrorKind::FailedOutput)
        },
        OutputType::Json => {
            let output = JsonOutputSearchResult;

            output
                .output(&mut stdout, results)
                .chain_err(|| ErrorKind::FailedOutput)
        },
        OutputType::Plain => {
            let output = PlainOutputSearchResult;

            output
                .output(&mut stdout, results)
                .chain_err(|| ErrorKind::FailedOutput)
        },
    }
}
