# date-version

## Examples

Point to a git repo that reports `4.1.0-2-g0ebd049` from `git describe`.

```
date-version path/to/repo --date
```

returns: `4.7707.0`, or `major.days.0`.  Days is number of days since 2000.01.01.


```
date-version path/to/repo --date-split
```

returns: `4.30.2`, or `major.days1.days2`.  Days is split into two u8 to overwrite minor and patch.

```
date-version path/to/repo --date --drop-patch --revisions
```

returns: `4.7707.2`, or `major.days.commitsSinceTag`.  Patch is dropped, commits since last tag is added on.

### Reference

```
Date Version 0.1.0
Conor Patrick <conor@solokeys.com>
Generate a version string that is based on the latest tag in a git repo and it's creation date.  Because usually users
just want to know how recent a version is.

USAGE:
    date-version [FLAGS] [PATH]

FLAGS:
        --date                    Output version as major.days.0 (u16.u16.0), where major is from the last tag in the
                                  git repo, and days is number of days since 2000.01.01.
        --date-split              Output version as major.days1.days2 (u16.u8.u8), where major is from the last tag in
                                  the git repo, and ((days1 << 8) | days2) is number of days since 2000.01.01.
        --drop-major              Drop the major from the output.
        --drop-minor              Drop the minor from the output.
        --drop-patch              Drop the patch from the output.
        --enforce-u8              Fail if any version component does not fit in a u8.
    -h, --help                    Prints help information. Use --help for more details.
        --revisions               add .revisions (.u16) to the end of the output, where revisions is number of commits
                                  since last tag.
        --revisions-prerelease    add -revisions (-u16) to the end of the output, where revisions is number of commits
                                  since last tag.
    -V, --version                 Prints version information

ARGS:
    <PATH>    Path to git repo.
```



