extern crate core;

use ethereum::AccessListItem;
use ethereum::EIP2930Transaction;
use ethereum::EnvelopedDecodable;
use ethereum::EnvelopedEncodable;
use ethereum::TransactionAction;
use ethereum::TransactionV1;
use ethereum::*;

use ethereum_types::{Address, H160, H256, U256};
use frame_support::*;
use hex::*;
use hex_literal::hex;
use ziggy;

fn main() {
	ziggy::fuzz!(|data: &[u8]| {
		if &data.len() < &501 {
			return;
		}
		println!("{:?}", &data);

		let eip = EIP2930Transaction {
			chain_id: 5,
			nonce: 7.into(),
			gas_price: U256::from(&data[0..4]),
			gas_limit: U256::from(&data[4..8]),
			action: TransactionAction::Call(H160(data[8..28].try_into().unwrap())),
			value: U256::from(data[28]) * 1_000_000_000 * 1_000_000_000,
			input: data[32..288].to_vec(),
			access_list: vec![
				AccessListItem {
					address: H256(data[288..320].try_into().unwrap()).into(),
					storage_keys: vec![
						H256(data[320..352].try_into().unwrap()),
						H256(data[352..384].try_into().unwrap()),
					],
				},
				AccessListItem {
					address: H160(data[384..404].try_into().unwrap()),
					storage_keys: vec![
						H256(data[404..436].try_into().unwrap());
						data[436] as usize
					],
				},
			],
			odd_y_parity: false,
			r: H256(data[437..469].try_into().unwrap()),
			s: H256(data[469..501].try_into().unwrap()),
		};

		println!("{:#?}", &eip);

		let tx = TransactionV2::EIP2930(eip.clone());
		let tx2 = TransactionV1::EIP2930(eip.clone());

		assert_eq!(
			tx,
			<TransactionV2 as EnvelopedDecodable>::decode(&tx.encode()).unwrap()
		);

		assert_eq!(
			tx2,
			<TransactionV1 as EnvelopedDecodable>::decode(&tx2.encode()).unwrap()
		);

		let _ = <TransactionV0 as EnvelopedDecodable>::decode(&data);

		let sign = TransactionSignature::new(
			U256::from(&data[4..8]).as_u32().into(),
			H256(data[437..469].try_into().unwrap()),
			H256(data[469..501].try_into().unwrap()),
		);

		// assert!(sign.is_none());

		let txxx = TransactionV0 {
			nonce: 12.into(),
			gas_price: U256::from(&data[0..4]),
			gas_limit: U256::from(&data[4..8]),
			action: TransactionAction::Call(H160(data[8..28].try_into().unwrap())),
			value: U256::from(&data[4..8]) * 1_000_000_000 * 1_000_000_000,
			input: data[32..288].to_vec(),
			signature: TransactionSignature::new(
				38,
				hex!("be67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717").into(),
				hex!("2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718").into(),
			)
			.unwrap(),
		};

		// assert_err!(sign);

		assert_eq!(
			txxx,
			<TransactionV0 as EnvelopedDecodable>::decode(&txxx.encode()).unwrap()
		);
	});
}
