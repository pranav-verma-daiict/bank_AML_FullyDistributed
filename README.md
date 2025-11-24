## bank_AML_FullyDistributed
This is the fully distributed veriosn of the AML solution.
## There will be two parts of the distributed version


### First part with 90% of the requirements being satisfied.  
  Below is what it does:

  Feature,         How It’s Achieved,         Privacy Guarantee
  5 independent banks,    Each runs the same binary with cargo run --release <id>,    No coordinator
  No trusted third party,    Only Redis as dumb bulletin board (like a shared drive),    Redis sees only encrypted blobs
  Bank-salted client IDs,    "hash_id(person, bank_id) → same person has different encrypted ID at each bank",    Zero intersection leakage — Bank A cannot tell if Bank B has the same client
  Fully homomorphic aggregation,    "TFHE smart_add_assign on encrypted (sum, count)",     No one ever sees individual scores
  Perfect selective disclosure,    "Each bank only ""reveals"" averages for clients it generated (simulation)",    Bank 4 learns nothing about clients it doesn't have
  Bulletin board pattern,    "Redis list ""records"" — append-only,     tamper-evident",Regulators can audit forever
  Zero metadata leakage,    No direct bank-to-bank messages — only through Redis,    No traffic analysis possible

  What it does not do (will be done in the next iteration):

  Missing Feature,            Why It's Missing,When It Will Be Added
  Real threshold decryption (3-out-of-5),"Currently,     aggregation produces encrypted (sum, count) but we simulate the final reveal with fake numbers",When you say threshold
  Banks actually locating their own clients in the aggregate,    "Right now, we skip the smart_eq search and just print fake averages",Will be added in threshold version
  Real distributed key generation (DKG),    All banks use independent keys → aggregation works only because we re-encrypt under one key (Bank 0),Will be replaced with real DKG
  Partial decryption shares over P2P,    No bank-to-bank communication yet,Will be added with Tokio TCP/QUIC
  Anonymous posting of partial shares,    Not needed yet,Will be added.

### Second part is where I will try to fix these features and hope it work as per our requirements.
I am Pausing this version for the moment. Will work on the next iteration of the project see that at "**bank-threshold-csv**" version.
