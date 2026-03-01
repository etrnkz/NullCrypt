use tempfile::TempDir;
use vault_engine::Vault;

#[test]
fn test_full_vault_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("test.vault");
    let password = b"test_password_12345".to_vec();

    // Create vault
    let mut vault = Vault::create(password.clone());

    // Add files
    vault.add_file("file1.txt".to_string(), b"Content 1".to_vec());
    vault.add_file("file2.txt".to_string(), b"Content 2".to_vec());

    // Save vault
    vault.save(&vault_path).unwrap();

    // Verify file exists
    assert!(vault_path.exists());

    // Open vault
    let loaded_vault = Vault::open(&vault_path, password).unwrap();

    // Verify contents
    let file1 = loaded_vault.get_file("file1.txt").unwrap();
    assert_eq!(file1.data, b"Content 1");

    let file2 = loaded_vault.get_file("file2.txt").unwrap();
    assert_eq!(file2.data, b"Content 2");

    // List files
    let files = loaded_vault.list_files();
    assert_eq!(files.len(), 2);
}

#[test]
fn test_wrong_password_fails() {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("test.vault");

    let vault = Vault::create(b"correct_password".to_vec());
    vault.save(&vault_path).unwrap();

    let result = Vault::open(&vault_path, b"wrong_password".to_vec());
    assert!(result.is_err());
}

#[test]
fn test_vault_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let vault_path = temp_dir.path().join("test.vault");
    let password = b"test_password".to_vec();

    // Create and save
    {
        let mut vault = Vault::create(password.clone());
        vault.add_file("test.txt".to_string(), b"Test data".to_vec());
        vault.save(&vault_path).unwrap();
    }

    // Load in new scope
    {
        let vault = Vault::open(&vault_path, password).unwrap();
        let file = vault.get_file("test.txt").unwrap();
        assert_eq!(file.data, b"Test data");
    }
}
