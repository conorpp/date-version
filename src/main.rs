use clap::{self, crate_authors, crate_version, App, Arg };
use git2::Repository;

pub const OFFSET_SECONDS_1970_TO_2000: i64 = 946713600;

fn main() {
    let matches = App::new("Date Version")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Generate a version string that is based on the latest tag in a git repo and it's creation date.  Because usually users just want to know how recent a version is.")
        .help_message("Prints help information. Use --help for more details.")
            .arg(Arg::with_name("PATH")
                .value_name("PATH")
                .help("Path to git repo.")
                .takes_value(true))
            .arg(Arg::with_name("date")
                    .long("date")
                    .help("Output version as major.days.0 (u16.u16.0), where major is from the last tag in the git repo, and days is number of days since 2000.01.01.")
                    .takes_value(false))
            .arg(Arg::with_name("date-split")
                    .long("date-split")
                    .help("Output version as major.days1.days2 (u16.u8.u8), where major is from the last tag in the git repo, and ((days1 << 8) | days2) is number of days since 2000.01.01.")
                    .takes_value(false))
            .arg(Arg::with_name("revisions")
                    .long("revisions")
                    .help("add .revisions (.u16) to the end of the output, where revisions is number of commits since last tag.")
                    .takes_value(false))
            .arg(Arg::with_name("revisions-prerelease")
                    .long("revisions-prerelease")
                    .help("add -revisions (-u16) to the end of the output, where revisions is number of commits since last tag.")
                    .takes_value(false))
            .arg(Arg::with_name("drop-major")
                    .long("drop-major")
                    .help("Drop the major from the output.")
                    .takes_value(false))
            .arg(Arg::with_name("drop-minor")
                    .long("drop-minor")
                    .help("Drop the minor from the output.")
                    .takes_value(false))
            .arg(Arg::with_name("drop-patch")
                    .long("drop-patch")
                    .help("Drop the patch from the output.")
                    .takes_value(false))
            .arg(Arg::with_name("enforce-u8")
                    .long("enforce-u8")
                    .help("Fail if any version component does not fit in a u8.")
                    .takes_value(false))

            .get_matches();


    match try_main(matches) {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        },
        _ => {}
    }
}

fn try_main(matches: clap::ArgMatches<'_>) -> anyhow::Result<()> {
    let path = matches.value_of("PATH").unwrap_or(".");

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => return Err(anyhow::anyhow!("failed to open git repo: {}", e)),
    };

    let describe_string = match repo.describe(&git2::DescribeOptions::new()) {
        Ok(describe) => {
            describe.format(None).unwrap()
        },
        Err(e) => return Err(anyhow::anyhow!("failed to describe git repo {}: {}", path, e)),
    };

    let git_version = semver::Version::parse(&describe_string).unwrap();
    // println!("{}", git_version);
    let major = git_version.major;
    let mut minor = git_version.minor;
    let mut patch = git_version.patch;
    let mut revisions: u64 = 0;

    let prerelease = git_version.pre.clone();
    if prerelease.len() > 0 {
        let chunks = describe_string.split("-").collect::<Vec<&str>>();
        if chunks.len() > 1 {
            revisions = chunks[1].parse().unwrap();
        } 
    }

    let reference = repo.find_reference(&format!("refs/tags/{}.{}.{}", major, minor, patch)).unwrap();

    let tag_oid = reference.target().unwrap();
    let tag = repo.find_tag(tag_oid).unwrap();

    let tagger = match tag.tagger() {
        Some(tagger) => tagger,
        None => return Err(anyhow::anyhow!("No author information in last tag.  Cannot pull date.")),
    };

    let timestamp = tagger.when().seconds() - OFFSET_SECONDS_1970_TO_2000;
    assert!(timestamp > 0);

    let days = timestamp / 60 / 60 / 24;

    let showing_revisions = matches.is_present("revisions") || matches.is_present("revisions-prerelease");

    if matches.is_present("date") {
        minor = days as u64;
        patch = 0;
    }

    if matches.is_present("date-split") {
        if days >= (1<<16) {
            return Err(anyhow::anyhow!("number of days since 2000.01.01 cannot fit in u16.  It's been a long time!: {}", days));
        }
        minor = ((days as u64) & 0xff00) >> 8;
        patch = ((days as u64) & 0xff) >> 0;
    }


    if major >= (1<<16) || minor >= (1<<16) || patch >= (1<<16) || (showing_revisions && revisions > (1<<16)){
        return Err(anyhow::anyhow!("cannot version component into a u16: {}.{}.{}.{}", major,minor,patch,revisions));
    }

    if matches.is_present("enforce-u8") {
        if major >= (1<<8) || minor >= (1<<8) || patch >= (1<<8) || (showing_revisions && revisions > (1<<8)){
            return Err(anyhow::anyhow!("cannot version component into a u8: {}.{}.{}.{}", major,minor,patch,revisions));
        }
    }

    let mut print_dot = false;
    if !matches.is_present("drop-major") {
        print!("{}", major);
        print_dot = true;
    }

    if !matches.is_present("drop-minor") {
        if print_dot {
            print!(".");
        }
        print!("{}", minor);
        print_dot = true;
    }

    if !matches.is_present("drop-patch") {
        if print_dot {
            print!(".");
        }
        print!("{}", patch);
    }

    if showing_revisions {
        if matches.is_present("revisions") {
            print!(".");
        } else {
            print!("-");
        }
        print!("{}", revisions);
    }

    println!();



    Ok(())
}
