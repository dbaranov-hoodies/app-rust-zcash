/*****************************************************************************
 *   Ledger App Boilerplate Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/
use crate::{AppSW, app_ui::load_glyph};

use alloc::{format, string::String};

use include_gif::include_gif;
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview};

fn format_zec_amount(amount: u64) -> String {
    // ZEC has 8 decimal places
    let whole = amount / 100_000_000;
    let fractional = amount % 100_000_000;
    format!("ZEC {}.{:08}", whole, fractional)
}

/// Displays transaction output and returns true if user approved it.
pub fn ui_display_tx_output(
    output_number: usize,
    amount: u64,
    address: &str,
    is_change: bool,
) -> Result<bool, AppSW> {
    let value_str = format_zec_amount(amount);
    // Make it 1-based for display
    let output_number = output_number + 1;

    // Define transaction review fields
    let my_fields = [
        Field {
            name: "Amount",
            value: value_str.as_str(),
        },
        Field {
            name: "Address",
            value: address,
        },
    ];

    // Create transaction review
    let title = if is_change {
        format!("Review change output #{output_number}")
    } else {
        format!("Review output #{output_number}")
    };

    // Create NBGL review. Maximum number of fields and string buffer length can be customised
    // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
    let review: NbglReview = NbglReview::new()
        .titles(&title, "", "Accept")
        .glyph(load_glyph());

    Ok(review.show(&my_fields))
}

/// Displays transaction fees and returns true if user approved it.
pub fn ui_display_tx_fees(
    fees: u64,
) -> Result<bool, AppSW> {
    let fees_str = format_zec_amount(fees);

    // Define transaction review fields
    let my_fields = [
        Field {
            name: "Fees",
            value: fees_str.as_str(),
        },
    ];

    // Create transaction review
    let title = format!("Confirm transaction");

    // Create NBGL review. Maximum number of fields and string buffer length can be customised
    // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
    let review: NbglReview = NbglReview::new()
        .titles(&title, "", "Accept and send")
        .glyph(load_glyph());

    Ok(review.show(&my_fields))
}
