use iroha_client::client::Client;
use iroha_data_model::prelude::*;
use eyre::Result;

pub async fn issue_digital_id(client: &Client) -> Result<()> {
    println!("🪪 Initializing Identity System...");

    // 1. Register a University Domain
    let domain_id: DomainId = "university".parse()?;
    client.submit(Register::domain(Domain::new(domain_id.clone()))).await?;

    // 2. Onboard a Student (Create Account)
    let student_id: AccountId = "student_01@university".parse()?;
    let student_key = KeyPair::generate()?; // In production, student provides PubKey
    
    let create_account = Register::account(Account::new(student_id.clone(), [student_key.public_key().clone()]));
    client.submit(create_account).await?;

    // 3. Define the Diploma (Credential)
    let diploma_def: AssetDefinitionId = "diploma#university".parse()?;
    client.submit(Register::asset_definition(AssetDefinition::store(diploma_def.clone()))).await?;

    // 4. Issue Diploma and attach IPFS Hash of the PDF
    let diploma_id = AssetId::new(diploma_def, student_id);
    let attach_hash = SetKeyValue::asset(diploma_id, "ipfs_hash".parse()?, "QmHashOfPDFDocument...".into());
    client.submit(attach_hash).await?;

    println!("✅ Diploma issued to {}", student_id);

    Ok(())
}