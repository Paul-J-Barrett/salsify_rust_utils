## Salsify Rust Utilities

Comand line utilities to export data and create other functionality from the Salsify APIs using the Rust programming language.

Original prototypes were written in Python with Pandas, requests, numpy, time, json modules. The exports didn't require Pandas. 

We are currently trying to setup a simplified workflows for our Content team to go from Amazon Catalog to Salsify and back again To Amazon.

Link to the Salsify API (Export) https://developers.salsify.com/reference/start-export-run

To use these utilities initially you will need two environmental variables setup.
SALSIFYTOK  which is your bearer api token. See documentation on Salsify site to get this information.
SALSIFYORGID


If using linux or OSX then add this to your .bashrc or .zshrc or other profile.
export SALSIFYTOK='Your token here'
export SALSIFYORGID='Your Salsify Organization ID starts with s-'

If you are using windows hit start then type "Environment" then click the one for user. Then add SALSIFYTOK and SALSIFYORGID with the appropriate values.


Commands will be using clap so typing command in shell without params should prompt you for the minimum requried values. During initial testing I might set defaults.

To get help:
command -h 

### Initial Commands Planned

I will add functionality and commands as needed.

* exportSalsifyIds
* deleteProducts (uses list created by exportSalsifyIds, we needed to delete all products and Salsify has no capability to do this in reasonable amount of effort.)
* exportProducts
* exportDigitialAssets
* exportAttributes
* exportAttribute_values (enumerated values of properties)

### Possible future roadmap
* importing
* creating attributes
* creating products
* creating, importing enumeration values
* ...