use keyring::Entry;

#[tauri::command]
pub fn save_user(username: &str, password: &str) {
    let entry = Entry::new("fishtest_manager", "username").unwrap();
    entry.set_password(&username).unwrap();

    let entry = Entry::new("fishtest_manager", "password").unwrap();
    entry.set_password(&password).unwrap();
}

#[tauri::command]
pub fn get_user() -> (String, String) {
    (
        Entry::new("fishtest_manager", "username")
            .unwrap()
            .get_password()
            .unwrap(),
        Entry::new("fishtest_manager", "password")
            .unwrap()
            .get_password()
            .unwrap(),
    )
}
