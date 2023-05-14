## Contributing to Bolt

### Version bump checklist
* install cargo-bump

* sync makefile and justfile

* Makefile VERSION

* run `make bump-version`

* Yew dependencies

* tauri.conf.json (package->version)
* tauri dependencies

* CLI README
* CLI dependencies

* bolt_core/common dependencies
* bolt_core/common VERSION (prelude->VERSION)

* bolt_core/core dependencies

* bolt_core/http dependencies

* bolt_core/servers dependencies

* bolt_core/tcp dependencies

* bolt_core/udp dependencies

* bolt_core/ws dependencies

* Run `make run` and `make run-cli` to ensure they compile and run

* run `make publish-libs`

* run `make publish-cli`