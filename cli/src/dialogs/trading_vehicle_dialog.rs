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

use std::error::Error;

use core::TrustFacade;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use model::{TradingVehicle, TradingVehicleCategory};

use crate::views::TradingVehicleView;

pub struct TradingVehicleDialogBuilder {
    symbol: Option<String>,
    isin: Option<String>,
    category: Option<TradingVehicleCategory>,
    broker: Option<String>,
    result: Option<Result<TradingVehicle, Box<dyn Error>>>,
}

impl TradingVehicleDialogBuilder {
    pub fn new() -> Self {
        TradingVehicleDialogBuilder {
            symbol: None,
            isin: None,
            category: None,
            broker: None,
            result: None,
        }
    }

    pub fn build(mut self, trust: &mut TrustFacade) -> TradingVehicleDialogBuilder {
        let isin = self.isin.clone().expect("Select isin first");
        let symbol = self.symbol.clone().expect("Select symbol first");
        let category = self.category.expect("Select category first");
        let broker = self.broker.clone().expect("Select broker first");

        self.result = Some(trust.create_trading_vehicle(&symbol, &isin, &category, &broker));
        self
    }

    pub fn display(self) {
        match self
            .result
            .expect("No result found, did you forget to call build?")
        {
            Ok(tv) => TradingVehicleView::display(tv),
            Err(error) => println!("Error creating trading vehicle: {error:?}"),
        }
    }

    pub fn category(mut self) -> Self {
        let available_categories = TradingVehicleCategory::all();

        let selected_category = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Category:")
            .items(&available_categories[..])
            .interact()
            .map(|index| available_categories.get(index).unwrap())
            .unwrap();

        self.category = Some(*selected_category);
        self
    }

    pub fn symbol(mut self) -> Self {
        self.symbol = Some(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Symbol: ")
                .validate_with({
                    |input: &String| -> Result<(), &str> {
                        match input.parse::<String>() {
                            Ok(_) => Ok(()),
                            Err(_) => Err("Please enter a valid symbol."),
                        }
                    }
                })
                .interact_text()
                .unwrap(),
        );
        self
    }

    pub fn isin(mut self) -> Self {
        self.isin = Some(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("ISIN: ")
                .validate_with({
                    |input: &String| -> Result<(), &str> {
                        match input.parse::<String>() {
                            Ok(_) => Ok(()),
                            Err(_) => Err("Please enter a valid ISIN."),
                        }
                    }
                })
                .interact_text()
                .unwrap(),
        );
        self
    }

    pub fn broker(mut self) -> Self {
        self.broker = Some(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Broker: ")
                .validate_with({
                    |input: &String| -> Result<(), &str> {
                        match input.parse::<String>() {
                            Ok(_) => Ok(()),
                            Err(_) => Err("Please enter a valid broker."),
                        }
                    }
                })
                .interact_text()
                .unwrap(),
        );
        self
    }
}

pub struct TradingVehicleSearchDialogBuilder {
    result: Option<Result<TradingVehicle, Box<dyn Error>>>,
}

impl TradingVehicleSearchDialogBuilder {
    pub fn new() -> Self {
        TradingVehicleSearchDialogBuilder { result: None }
    }

    pub fn build(self) -> Result<TradingVehicle, Box<dyn Error>> {
        self.result
            .expect("No result found, did you forget to call search?")
    }

    pub fn display(self) {
        match self
            .result
            .expect("No result found, did you forget to call search?")
        {
            Ok(tv) => {
                TradingVehicleView::display(tv);
            }
            Err(error) => println!("Error searching Trading Vehicles: {error:?}"),
        }
    }

    pub fn search(mut self, trust: &mut TrustFacade) -> Self {
        let trading_vehicles = trust.search_trading_vehicles();
        match trading_vehicles {
            Ok(tvs) => {
                if tvs.is_empty() {
                    panic!("No trading vehicles found, did you forget to add one?")
                }
                let selected_tv = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Trading Vehicle: ")
                    .items(&tvs[..])
                    .interact()
                    .map(|index| tvs[index].clone())
                    .unwrap();

                self.result = Some(Ok(selected_tv));
            }
            Err(error) => self.result = Some(Err(error)),
        }

        self
    }
}
