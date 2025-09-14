use crate::domain::person::CountrySubdivision;
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait CountrySubdivisionService: Send + Sync {
    async fn create_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> BankingResult<CountrySubdivision>;
    async fn fix_country_subdivision(
        &self,
        country_subdivision: CountrySubdivision,
    ) -> BankingResult<CountrySubdivision>;
    async fn find_country_subdivision_by_id(
        &self,
        id: Uuid,
    ) -> BankingResult<Option<CountrySubdivision>>;
    async fn find_country_subdivisions_by_country_id(
        &self,
        country_id: Uuid,
    ) -> BankingResult<Vec<CountrySubdivision>>;
    async fn find_country_subdivision_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<10>,
    ) -> BankingResult<Option<CountrySubdivision>>;
}