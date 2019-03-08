use primitives::{Ed25519AuthorityId, ed25519};
use node_template_runtime::{
	AccountId, GenesisConfig, ConsensusConfig, TimestampConfig, BalancesConfig,
	SudoConfig, IndicesConfig, FeesConfig,
};
use substrate_service;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	IbTestnet,
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().into(),
				], vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into(),
				],
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into()
				),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| testnet_genesis(vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().into(),
					ed25519::Pair::from_seed(b"Bob                             ").public().into(),
				], vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into(),
					ed25519::Pair::from_seed(b"Bob                             ").public().0.into(),
					ed25519::Pair::from_seed(b"Charlie                         ").public().0.into(),
					ed25519::Pair::from_seed(b"Dave                            ").public().0.into(),
					ed25519::Pair::from_seed(b"Eve                             ").public().0.into(),
					ed25519::Pair::from_seed(b"Ferdie                          ").public().0.into(),
				],
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into()
				),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::IbTestnet => ChainSpec::from_genesis(
				"IB Chain",
				"ib_chain",
				|| testnet_genesis(vec![
					ed25519::Pair::from_seed(b"Matsuda                         ").public().into(),
				], vec![
					ed25519::Pair::from_seed(b"Matsuda                         ").public().0.into(),
				],
					ed25519::Pair::from_seed(b"Matsuda                         ").public().0.into(),
				),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}
	// $ subkey restore Matsuda
	//Seed 0x4d61747375646120202020202020202020202020202020202020202020202020 is account:
  // Public key (hex): 0x9d361c06d36e411422e49d14d00f5128662b8f84567400ee26aaeb3cfc8f7f77
  // Address (SS58): 5FcqSzTdr3zeiMan8S48Rz7yP8fG3YYkzjpD1iaSBAuihkME
	
	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"ib" => Some(Alternative::IbTestnet),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

fn testnet_genesis(initial_authorities: Vec<Ed25519AuthorityId>, endowed_accounts: Vec<AccountId>, root_key: AccountId) -> GenesisConfig {
	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/node_template_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			period: 5,					// 5 second block time.
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().map(|&k|(k, (1 << 60))).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 0,
			transaction_byte_fee: 0,
		})
	}
}
