use crate::keys;
use apca::api::v2::order::{Change, ChangeReq, Id, Order};
use apca::Client;
use model::{Account, Trade};
use num_decimal::Num;
use rust_decimal::Decimal;
use std::{error::Error, str::FromStr};
use tokio::runtime::Runtime;
use uuid::Uuid;

pub fn modify(trade: &Trade, account: &Account, price: Decimal) -> Result<Uuid, Box<dyn Error>> {
    assert!(trade.account_id == account.id); // Verify that the trade is for the account

    let api_info = keys::read_api_key(&account.environment, account)?;
    let client = Client::new(api_info);

    // Modify the stop order.
    let target_order_id = trade
        .target
        .broker_order_id
        .ok_or("Target order ID is missing")?;

    let alpaca_order = Runtime::new()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?
        .block_on(submit(&client, target_order_id, price))?;

    // TODO LOG

    Ok(alpaca_order.id.0)
}

async fn submit(client: &Client, order_id: Uuid, price: Decimal) -> Result<Order, Box<dyn Error>> {
    let request = ChangeReq {
        limit_price: Some(
            Num::from_str(&price.to_string())
                .map_err(|e| format!("Failed to parse limit price: {e:?}"))?,
        ),
        ..Default::default()
    };

    let result = client.issue::<Change>(&(Id(order_id), request)).await;
    match result {
        Ok(log) => Ok(log),
        Err(e) => {
            eprintln!("Error modify stop: {e:?}");
            Err(Box::new(e))
        }
    }
}
