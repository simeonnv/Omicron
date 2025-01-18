pub const QUERIES: [&str; 2] = [
    r#"
        CREATE TABLE IF NOT EXISTS Accounts (
            account_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            username VARCHAR(20) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            role VARCHAR(20) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Tokens (
            token_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            user_id INT NOT NULL,
            token VARCHAR(255) NOT NULL UNIQUE,
            role VARCHAR(20) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
];