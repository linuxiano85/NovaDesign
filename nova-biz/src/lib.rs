use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Company profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyProfile {
    pub id: Uuid,
    pub name: String,
    pub vat_id: String,
    pub address: String,
    pub logo_path: Option<String>,
    pub iban: Option<String>,
    pub email: String,
    pub phone: String,
}

impl CompanyProfile {
    pub fn new(
        name: String,
        vat_id: String,
        address: String,
        email: String,
        phone: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            vat_id,
            address,
            logo_path: None,
            iban: None,
            email,
            phone,
        }
    }
}

/// Customer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub vat_id_or_cf: String, // P.IVA or Codice Fiscale
    pub address: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Customer {
    pub fn new(name: String, vat_id_or_cf: String, address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            vat_id_or_cf,
            address,
            email: None,
            phone: None,
        }
    }
}

/// Tax rate definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub id: Uuid,
    pub name: String,
    pub percent: f64,
}

impl TaxRate {
    pub fn new(name: String, percent: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            percent,
        }
    }

    /// Standard Italian VAT rates
    pub fn iva_22() -> Self {
        Self::new("IVA 22%".to_string(), 22.0)
    }

    pub fn iva_10() -> Self {
        Self::new("IVA 10%".to_string(), 10.0)
    }

    pub fn iva_4() -> Self {
        Self::new("IVA 4%".to_string(), 4.0)
    }

    pub fn esente() -> Self {
        Self::new("Esente IVA".to_string(), 0.0)
    }
}

/// Price list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceItem {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub unit: String, // pz, m, m², h, etc.
    pub base_price: f64,
    pub tax_rate: TaxRate,
}

impl PriceItem {
    pub fn new(
        code: String,
        description: String,
        unit: String,
        base_price: f64,
        tax_rate: TaxRate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            code,
            description,
            unit,
            base_price,
            tax_rate,
        }
    }

    pub fn price_with_tax(&self) -> f64 {
        self.base_price * (1.0 + self.tax_rate.percent / 100.0)
    }

    pub fn tax_amount(&self) -> f64 {
        self.base_price * (self.tax_rate.percent / 100.0)
    }
}

/// Price list containing multiple items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub items: Vec<PriceItem>,
}

impl PriceList {
    pub fn new(name: String, currency: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            currency,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: PriceItem) {
        self.items.push(item);
    }

    pub fn find_item_by_code(&self, code: &str) -> Option<&PriceItem> {
        self.items.iter().find(|item| item.code == code)
    }
}

/// Quote line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteLineItem {
    pub price_item_id: Uuid,
    pub code: String,
    pub description: String,
    pub unit: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub tax_rate: f64,
}

impl QuoteLineItem {
    pub fn new(price_item: &PriceItem, quantity: f64) -> Self {
        Self {
            price_item_id: price_item.id,
            code: price_item.code.clone(),
            description: price_item.description.clone(),
            unit: price_item.unit.clone(),
            quantity,
            unit_price: price_item.base_price,
            tax_rate: price_item.tax_rate.percent,
        }
    }

    pub fn subtotal(&self) -> f64 {
        self.quantity * self.unit_price
    }

    pub fn tax_amount(&self) -> f64 {
        self.subtotal() * (self.tax_rate / 100.0)
    }

    pub fn total(&self) -> f64 {
        self.subtotal() + self.tax_amount()
    }
}

/// Quote/Estimate (Preventivo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: Uuid,
    pub number: String,
    pub date: DateTime<Utc>,
    pub customer: Customer,
    pub company: CompanyProfile,
    pub line_items: Vec<QuoteLineItem>,
    pub notes: String,
}

impl Quote {
    pub fn new(number: String, customer: Customer, company: CompanyProfile) -> Self {
        Self {
            id: Uuid::new_v4(),
            number,
            date: Utc::now(),
            customer,
            company,
            line_items: Vec::new(),
            notes: String::new(),
        }
    }

    pub fn add_line_item(&mut self, item: QuoteLineItem) {
        self.line_items.push(item);
    }

    pub fn subtotal(&self) -> f64 {
        self.line_items.iter().map(|item| item.subtotal()).sum()
    }

    pub fn total_tax(&self) -> f64 {
        self.line_items.iter().map(|item| item.tax_amount()).sum()
    }

    pub fn total(&self) -> f64 {
        self.subtotal() + self.total_tax()
    }

    /// Export quote to CSV format
    pub fn export_csv(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record([
            "Codice",
            "Descrizione",
            "Unità",
            "Quantità",
            "Prezzo Unit.",
            "IVA %",
            "Subtotale",
            "IVA",
            "Totale",
        ])?;

        // Write line items
        for item in &self.line_items {
            wtr.write_record([
                &item.code,
                &item.description,
                &item.unit,
                &item.quantity.to_string(),
                &item.unit_price.to_string(),
                &item.tax_rate.to_string(),
                &item.subtotal().to_string(),
                &item.tax_amount().to_string(),
                &item.total().to_string(),
            ])?;
        }

        // Write totals
        wtr.write_record([
            "",
            "",
            "",
            "",
            "",
            "TOTALE:",
            &self.subtotal().to_string(),
            &self.total_tax().to_string(),
            &self.total().to_string(),
        ])?;

        let data = String::from_utf8(wtr.into_inner().map_err(|e| e.into_error())?).unwrap();
        Ok(data)
    }
}

/// Invoice (Fattura) - extends Quote functionality
pub type Invoice = Quote;

/// DDT (Documento di Trasporto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDT {
    pub id: Uuid,
    pub number: String,
    pub date: DateTime<Utc>,
    pub customer: Customer,
    pub company: CompanyProfile,
    pub line_items: Vec<QuoteLineItem>, // Same structure as quotes but for transport
    pub transport_reason: String,
    pub transport_method: String,
    pub packages: u32,
}

impl DDT {
    pub fn new(number: String, customer: Customer, company: CompanyProfile) -> Self {
        Self {
            id: Uuid::new_v4(),
            number,
            date: Utc::now(),
            customer,
            company,
            line_items: Vec::new(),
            transport_reason: "Vendita".to_string(),
            transport_method: "Mittente".to_string(),
            packages: 1,
        }
    }

    pub fn add_line_item(&mut self, item: QuoteLineItem) {
        self.line_items.push(item);
    }

    /// Export DDT to CSV format
    pub fn export_csv(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record([
            "Codice",
            "Descrizione",
            "Unità",
            "Quantità",
            "Causale",
            "Trasporto",
            "Colli",
        ])?;

        // Write line items
        for (i, item) in self.line_items.iter().enumerate() {
            let causale = if i == 0 { &self.transport_reason } else { "" };
            let trasporto = if i == 0 { &self.transport_method } else { "" };
            let colli = if i == 0 {
                &self.packages.to_string()
            } else {
                ""
            };

            wtr.write_record([
                &item.code,
                &item.description,
                &item.unit,
                &item.quantity.to_string(),
                causale,
                trasporto,
                colli,
            ])?;
        }

        let data = String::from_utf8(wtr.into_inner().map_err(|e| e.into_error())?).unwrap();
        Ok(data)
    }
}

/// Numbering sequence for documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberingSequence {
    pub id: Uuid,
    pub name: String,
    pub prefix: String,
    pub current_number: u32,
    pub year: i32,
}

impl NumberingSequence {
    pub fn new(name: String, prefix: String, year: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            prefix,
            current_number: 0,
            year,
        }
    }

    pub fn next_number(&mut self) -> String {
        self.current_number += 1;
        format!("{}{}/{}", self.prefix, self.current_number, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_calculations() {
        let tax_rate = TaxRate::iva_22();
        let price_item = PriceItem::new(
            "TEST001".to_string(),
            "Test Item".to_string(),
            "pz".to_string(),
            100.0,
            tax_rate,
        );

        assert_eq!(price_item.price_with_tax(), 122.0);
        assert_eq!(price_item.tax_amount(), 22.0);
    }

    #[test]
    fn test_quote_totals() {
        let customer = Customer::new(
            "Test Customer".to_string(),
            "12345678901".to_string(),
            "Via Test 123".to_string(),
        );
        let company = CompanyProfile::new(
            "Test Company".to_string(),
            "IT12345678901".to_string(),
            "Via Company 456".to_string(),
            "test@company.com".to_string(),
            "+39123456789".to_string(),
        );

        let mut quote = Quote::new("Q001/2024".to_string(), customer, company);

        let price_item = PriceItem::new(
            "TEST001".to_string(),
            "Test Item".to_string(),
            "pz".to_string(),
            100.0,
            TaxRate::iva_22(),
        );

        let line_item = QuoteLineItem::new(&price_item, 2.0);
        quote.add_line_item(line_item);

        assert_eq!(quote.subtotal(), 200.0);
        assert_eq!(quote.total_tax(), 44.0);
        assert_eq!(quote.total(), 244.0);
    }

    #[test]
    fn test_numbering_sequence() {
        let mut seq = NumberingSequence::new("Preventivi".to_string(), "PREV".to_string(), 2024);

        assert_eq!(seq.next_number(), "PREV1/2024");
        assert_eq!(seq.next_number(), "PREV2/2024");
    }

    #[test]
    fn test_csv_export() {
        let customer = Customer::new(
            "Test Customer".to_string(),
            "12345678901".to_string(),
            "Via Test 123".to_string(),
        );
        let company = CompanyProfile::new(
            "Test Company".to_string(),
            "IT12345678901".to_string(),
            "Via Company 456".to_string(),
            "test@company.com".to_string(),
            "+39123456789".to_string(),
        );

        let mut quote = Quote::new("Q001/2024".to_string(), customer, company);

        let price_item = PriceItem::new(
            "TEST001".to_string(),
            "Test Item".to_string(),
            "pz".to_string(),
            100.0,
            TaxRate::iva_22(),
        );

        let line_item = QuoteLineItem::new(&price_item, 1.0);
        quote.add_line_item(line_item);

        let csv = quote.export_csv().unwrap();
        assert!(csv.contains("Codice,Descrizione"));
        assert!(csv.contains("TEST001,Test Item"));
    }
}
