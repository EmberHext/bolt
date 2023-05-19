## Contributing to Bolt

### Version bump checklist
* install cargo-bump

* sync makefile and justfile

* Makefile VERSION

* run `make bump-version`

* tauri.conf.json (package->version)

* CLI README

* bolt_core/common VERSION (prelude->VERSION)

* Run `make run` and `make run-cli` to ensure they compile and run

* Push changes to remote repository

* run `make publish-cli`