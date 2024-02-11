## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work. To do so, please run the following command:

```bash 
    make install
```

The make install command tests and tries to install all required developer dependencies.
if the automatic install fails, the script will show a link with further installation instructions.

After all dependencies have been installed, the following commands are ready to use.

```text 
make
    make build   	Builds the code base incrementally (fast) for dev.
    make check   	Checks the code base for security vulnerabilities.
    make clean   	Cleans generated files and folders.
    make fix   		Fixes linting issues as reported by clippy.
    make format   	Formats call code according to cargo fmt style.
    make install   	Tests and installs all make script dependencies.
    make release   	Builds the code base for release.
    make test   	Tests across all crates.
    make run   		Runs the default binary.
    make update   	Update rust, pull git, and build the project.
```

The scripts called by each make command are located in the [script folder.](scripts)