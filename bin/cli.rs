use my_test::SignalLogger;
use odra::host::NoArgs;
use odra::prelude::Addressable;
use odra_cli::{
    deploy::DeployScript,
    DeployedContractsContainer, DeployerExt,
    OdraCli,
};
use odra::host::HostEnv;

pub struct SignalLoggerDeployScript;

impl DeployScript for SignalLoggerDeployScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut DeployedContractsContainer,
    ) -> Result<(), odra_cli::deploy::Error> {
        let contract = SignalLogger::load_or_deploy(env, NoArgs, container, 500_000_000_000)?;
        eprintln!("✅ Contract address: {:?}", contract.address());
        Ok(())
    }
}

fn main() {
    OdraCli::new()
        .deploy(SignalLoggerDeployScript)
        .run();
}
