# General
package
  create
  - bin or lib
    - module

package > crate > module

- we4i00p
- we3i03p

openshift-cloud-credential-operator
ExtremelyHighIndividualControlPlaneCPU

Alessando Capozzi
Antono Iuzzolino
Flavio Gandiglio
Theodoros 


CsvAbnormalFailedOver2Min -> Falso allarme





foobar
├── Cargo.toml (package)
├── build.rs
└── src/
    ├── main.rs (bin create)
    ├── util.rs (module)
    ├── lib.rs (lib create)
    └── bin/
        └── alt.rs (bin create)

# Module

mod garden;

- Inline, within curly brackets that replace the semicolon following mod garden
- In the file src/garden.rs
- In the file src/garden/mod.rs

# Submodules

declare mod vegetables; in src/garden.rs. 
- Inline, directly following mod vegetables, within curly brackets instead of the semicolon
- In the file src/garden/vegetables.rs
- In the file src/garden/vegetables/mod.rs