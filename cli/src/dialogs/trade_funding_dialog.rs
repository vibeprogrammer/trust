//! UI Dialog Module - User Interaction Code
//!
//! TEMPORARY SAFETY ALLOWANCE: This dialog module contains user interaction code
//! that uses .unwrap() and .expect() for UI input handling. While not ideal,
//! these are less critical than business logic safety violations.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use crate::views::{AccountBalanceView, TradeBalanceView, TradeView};
use crate::{dialogs::AccountSearchDialog, views::TransactionView};
use core::TrustFacade;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use model::{Account, AccountBalance, Status, Trade, TradeBalance, Transaction};
use std::error::Error;

type TradeDialogApproverBuilderResult =
    Option<Result<(Trade, Transaction, AccountBalance, TradeBalance), Box<dyn Error>>>;

pub struct FundingDialogBuilder {
    account: Option<Account>,
    trade: Option<Trade>,
    result: TradeDialogApproverBuilderResult,
}

impl FundingDialogBuilder {
    pub fn new() -> Self {
        FundingDialogBuilder {
            account: None,
            trade: None,
            result: None,
        }
    }

    pub fn build(mut self, trust: &mut TrustFacade) -> FundingDialogBuilder {
        let trade: Trade = self.trade.clone().unwrap();
        self.result = Some(trust.fund_trade(&trade));
        self
    }

    pub fn display(self) {
        match self
            .result
            .expect("No result found, did you forget to call search?")
        {
            Ok((trade, tx, account_balance, trade_balance)) => {
                let account = self.account.clone().unwrap().name;

                println!("Trade approved:");
                TradeView::display(&trade, &self.account.unwrap().name);

                TradeBalanceView::display(&trade_balance);

                println!("Transaction moving funds to trade:");
                TransactionView::display(&tx, account.as_str());

                println!("Account balance after funding trade:");
                AccountBalanceView::display(account_balance, account.as_str());
            }
            Err(error) => println!("Error approving trade: {error:?}"),
        }
    }

    pub fn account(mut self, trust: &mut TrustFacade) -> Self {
        let account = AccountSearchDialog::new().search(trust).build();
        match account {
            Ok(account) => self.account = Some(account),
            Err(error) => println!("Error searching account: {error:?}"),
        }
        self
    }

    pub fn search(mut self, trust: &mut TrustFacade) -> Self {
        let trades = trust.search_trades(self.account.clone().unwrap().id, Status::New);
        match trades {
            Ok(trades) => {
                if trades.is_empty() {
                    panic!("No trade found, did you forget to create one?")
                }
                let trade = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Trade:")
                    .items(&trades[..])
                    .default(0)
                    .interact_opt()
                    .unwrap()
                    .map(|index| trades.get(index).unwrap())
                    .unwrap();

                self.trade = Some(trade.to_owned());
            }
            Err(error) => self.result = Some(Err(error)),
        }

        self
    }
}
