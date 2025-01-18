pub const QUERIES: [&str; 2] = [
    r#"
        CREATE TABLE IF NOT EXISTS Accounts (
            AccountID INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            Username VARCHAR(20) NOT NULL,
            Password VARCHAR(255) NOT NULL,
            Role VARCHAR(20) NOT NULL,
            CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Tokens (
            TokenID INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            UserID INT NOT NULL,
            Token VARCHAR(255) NOT NULL,
            Role VARCHAR(20) NOT NULL,
            CreatedAt DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (UserID) REFERENCES Accounts(AccountID) ON DELETE CASCADE
        );
    "#,
];