use testcontainers::{Image, ImageArgs, core::WaitFor};

const STARTUP_NOTIFY: &str = "START_DONE";

#[derive(Default)]
pub struct Elementsd;

impl Image for Elementsd {
    type Args = ElementsArgs;

    fn name(&self) -> String {
        "blockstream/elementsd".into()
    }

    fn tag(&self) -> String {
        "22.0.2".into()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![
            WaitFor::message_on_stdout(STARTUP_NOTIFY),
        ]
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![18444]
    }
}


#[derive(Clone, Debug, Default)]
pub struct ElementsArgs;


impl ImageArgs for ElementsArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let args = [
            "elementsd".to_string(),
            "-regtest".to_string(),
            "-server".to_string(),
            "-printtoconsole".to_string(),
            "-rpcbind=0.0.0.0".to_string(),
            "-rpcallowip=0.0.0.0/0".to_string(),
            "-rpcport=18444".to_string(),
            "-rpcuser=user".to_string(),
            "-rpcpassword=pass".to_string(),
            format!("-startupnotify='echo {}'", STARTUP_NOTIFY),
        ];
        Box::new(args.into_iter())
    }
}
