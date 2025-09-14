use crate::domain::person::Locality;
use crate::BankingResult;
use async_trait::async_trait;
use heapless::String as HeaplessString;
use uuid::Uuid;

#[async_trait]
pub trait LocalityService: Send + Sync {
    async fn create_locality(&self, locality: Locality) -> BankingResult<Locality>;
    async fn fix_locality(&self, locality: Locality) -> BankingResult<Locality>;
    async fn find_locality_by_id(&self, id: Uuid) -> BankingResult<Option<Locality>>;
    async fn find_localities_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> BankingResult<Vec<Locality>>;
    async fn find_locality_by_code(
        &self,
        country_id: Uuid,
        code: HeaplessString<50>,
    ) -> BankingResult<Option<Locality>>;
}