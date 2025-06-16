use std::sync::Mutex;
use tauri::State;

mod vault;
use vault::Vault;

type VaultState = Mutex<Option<Vault>>;

#[tauri::command]
fn open_vault(path: String, key: String, state: State<'_, VaultState>) -> Result<String, String> {
    let vault = Vault::new(path, key);

    let content = vault.read()?;

    // Store the vault as the active vault
    let mut vault_state = state.lock().unwrap();
    *vault_state = Some(vault);

    Ok(content)
}

#[tauri::command]
fn save_vault(data: String, state: State<'_, VaultState>) -> Result<(), String> {
    let mut vault_state = state.lock().unwrap();
    if let Some(vault) = vault_state.as_mut() {
        vault.write(data)?;
        Ok(())
    } else {
        Err("No active vault found".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_vault, save_vault])
        .manage(Mutex::new(Option::<Vault>::None))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
