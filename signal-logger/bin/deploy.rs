/// Deploy SignalLogger to Casper Testnet.
///
/// Required env vars:
///   CASPER_NODE_ADDRESS  — e.g. https://node.testnet.cspr.cloud/rpc
///   CASPER_CHAIN_NAME    — casper-test
///   CASPER_SECRET_KEY    — path to your PEM secret key file
///
/// Run with:
///   cargo odra deploy -b casper-livenet --bin deploy
fn main() {
    use signal_logger::SignalLoggerDeployer;

    let env = odra_casper_livenet_env::env();
    let contract = SignalLoggerDeployer::init(&env);
    println!("SignalLogger deployed at: {}", contract.address());
}
