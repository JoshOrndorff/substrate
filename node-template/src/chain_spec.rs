use primitives::{ed25519, sr25519, Pair};
use node_template_runtime::{
	AccountId, GenesisConfig, AuraConfig, BalancesConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY, AuraId};
use node_template_runtime::TemplateModuleConfig;
use node_template_runtime::SecondCopyConfig;
use node_template_runtime::CouncilConfig;
use node_template_runtime::TechnicalCommitteeConfig;

use substrate_service;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
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
}

fn authority_key(s: &str) -> AuraId {
	ed25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

fn account_key(s: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					authority_key("Alice")
				], vec![
					account_key("Alice")
				],
					account_key("Alice")
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
					authority_key("Alice"),
					authority_key("Bob"),
				], vec![
					account_key("Alice"),
					account_key("Bob"),
					account_key("Charlie"),
					account_key("Dave"),
					account_key("Eve"),
					account_key("Ferdie"),
				],
					account_key("Alice"),
				),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

fn testnet_genesis(initial_authorities: Vec<AuraId>, endowed_accounts: Vec<AccountId>, root_key: AccountId) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.clone(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		template_Instance2: Some(SecondCopyConfig {
			something: 7,
		//	phantom: Default::default(),
		}),
		template_Instance1: Some(TemplateModuleConfig {
			something: 6,
		//	phantom: Default::default(),
		}),
        collective_Instance1: Some(CouncilConfig {
			members: vec![account_key("Alice")],
			phantom: Default::default(),
		}),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: vec![account_key("Bob")],
			phantom: Default::default(),
		}),
	}
}


#[test]
fn test_genesis() {
	use node_template_runtime::BuildStorage;
	let c = testnet_genesis(Default::default(), Default::default(), Default::default());
	let mut s = c.build_storage().unwrap();
	sr_io::with_storage(&mut s.0, || {
		assert_eq!(7, node_template_runtime::SecondCopy::something().unwrap());
		assert_eq!(6, node_template_runtime::TemplateModule::something().unwrap());
	});
}
