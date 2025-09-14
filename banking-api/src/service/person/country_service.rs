use crate::domain::person::Country;
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait CountryService: Send + Sync {
    async fn create_country(&self, country: Country) -> BankingResult<Country>;
    async fn fix_country(&self, country: Country) -> BankingResult<Country>;
    async fn find_country_by_id(&self, id: Uuid) -> BankingResult<Option<Country>>;
    async fn find_country_by_iso2(&self, iso2: HeaplessString<2>) -> BankingResult<Option<Country>>;
    async fn get_all_countries(&self) -> BankingResult<Vec<Country>>;
}