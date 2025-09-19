use banking_db::models::person::{EntityReferenceModel, PersonModel, PersonType, RelationshipRole};
use banking_db::repository::{
    BatchRepository, EntityReferenceRepository, PersonRepository, PersonRepos,
};
use heapless::String as HeaplessString;
use uuid::Uuid;

use crate::suites::test_helper::setup_test_context;

pub async fn setup_test_person() -> PersonModel {
    PersonModel {
        id: Uuid::new_v4(),
        person_type: PersonType::Natural,
        display_name: HeaplessString::try_from("Test Person").unwrap(),
        external_identifier: Some(HeaplessString::try_from("EXT_PERSON_001").unwrap()),
        entity_reference_count: 0,
        organization_person_id: None,
        messaging_info1: None,
        messaging_info2: None,
        messaging_info3: None,
        messaging_info4: None,
        messaging_info5: None,
        department: None,
        location_id: None,
        duplicate_of_person_id: None,
    }
}

pub async fn setup_test_entity_reference(person_id: Uuid) -> EntityReferenceModel {
    EntityReferenceModel {
        id: Uuid::new_v4(),
        person_id,
        entity_role: RelationshipRole::Customer,
        reference_external_id: HeaplessString::try_from("EXT_REF_001").unwrap(),
        reference_details_l1: None,
        reference_details_l2: None,
        reference_details_l3: None,
    }
}

#[tokio::test]
async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();
    let entity_ref_repo = ctx.person_repos().entity_references();

    let person = setup_test_person().await;
    person_repo.save(person.clone(), Uuid::new_v4()).await?;

    let mut entity_refs = Vec::new();
    for i in 0..5 {
        let mut entity_ref = setup_test_entity_reference(person.id).await;
        entity_ref.reference_external_id =
            HeaplessString::try_from(format!("EXT_REF_{i:03}").as_str()).unwrap();
        entity_refs.push(entity_ref);
    }

    let audit_log_id = Uuid::new_v4();

    let saved_entity_refs = entity_ref_repo
        .create_batch(entity_refs.clone(), audit_log_id)
        .await?;

    assert_eq!(saved_entity_refs.len(), 5);

    for entity_ref in &saved_entity_refs {
        assert!(entity_ref_repo.exists_by_id(entity_ref.id).await?);
    }

    Ok(())
}

#[tokio::test]
async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();
    let entity_ref_repo = ctx.person_repos().entity_references();

    let person = setup_test_person().await;
    person_repo.save(person.clone(), Uuid::new_v4()).await?;

    let mut entity_refs = Vec::new();
    for i in 0..3 {
        let mut entity_ref = setup_test_entity_reference(person.id).await;
        entity_ref.reference_external_id =
            HeaplessString::try_from(format!("LOAD_EXT_REF_{i:03}").as_str()).unwrap();
        entity_refs.push(entity_ref);
    }
    entity_ref_repo
        .create_batch(entity_refs.clone(), Uuid::new_v4())
        .await?;
    let ids: Vec<Uuid> = entity_refs.iter().map(|e| e.id).collect();
    let loaded = entity_ref_repo.load_batch(&ids).await?;
    assert_eq!(loaded.len(), 3);
    assert!(loaded.iter().all(|item| item.is_some()));
    Ok(())
}

#[tokio::test]
async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();
    let entity_ref_repo = ctx.person_repos().entity_references();

    let person = setup_test_person().await;
    person_repo.save(person.clone(), Uuid::new_v4()).await?;

    let mut entity_refs = Vec::new();
    for i in 0..3 {
        let mut entity_ref = setup_test_entity_reference(person.id).await;
        entity_ref.reference_external_id =
            HeaplessString::try_from(format!("ORIGINAL_{i}").as_str()).unwrap();
        entity_refs.push(entity_ref);
    }
    let saved = entity_ref_repo
        .create_batch(entity_refs.clone(), Uuid::new_v4())
        .await?;
    let mut to_update = saved.clone();
    for (i, item) in to_update.iter_mut().enumerate() {
        item.reference_external_id =
            HeaplessString::try_from(format!("UPDATED_{i}").as_str()).unwrap();
    }
    let updated = entity_ref_repo
        .update_batch(to_update, Uuid::new_v4())
        .await?;
    assert_eq!(updated.len(), 3);
    for item in updated {
        assert!(item.reference_external_id.starts_with("UPDATED"));
    }
    Ok(())
}

#[tokio::test]
async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = setup_test_context().await?;
    let person_repo = ctx.person_repos().persons();
    let entity_ref_repo = ctx.person_repos().entity_references();

    let person = setup_test_person().await;
    person_repo.save(person.clone(), Uuid::new_v4()).await?;

    let mut entity_refs = Vec::new();
    for _ in 0..4 {
        let entity_ref = setup_test_entity_reference(person.id).await;
        entity_refs.push(entity_ref);
    }
    let saved = entity_ref_repo
        .create_batch(entity_refs.clone(), Uuid::new_v4())
        .await?;
    let ids: Vec<Uuid> = saved.iter().map(|e| e.id).collect();
    let deleted_count = entity_ref_repo.delete_batch(&ids).await?;
    assert_eq!(deleted_count, 4);
    let loaded = entity_ref_repo.load_batch(&ids).await?;
    assert_eq!(loaded.len(), 4);
    assert!(loaded.iter().all(|item| item.is_none()));
    Ok(())
}