pub const QUERIES: [&str; 5] = [
    r#"
        CREATE TABLE IF NOT EXISTS Files (
            file_id INT AUTO_INCREMENT PRIMARY KEY,
            file_name VARCHAR(255) NOT NULL,
            file_data LONGBLOB NOT NULL,
            file_size INT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB;
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Accounts (
            account_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            pfp INT DEFAULT NULL,
            username VARCHAR(20) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            role VARCHAR(20) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (pfp) REFERENCES Files(file_id)
        ) ENGINE=InnoDB;
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Tokens (
            token_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            user_id INT NOT NULL,
            token VARCHAR(255) NOT NULL UNIQUE,
            role VARCHAR(20) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES Accounts(account_id) ON DELETE CASCADE
        ) ENGINE=InnoDB;
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Subicron (
            subicron_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            image_id INT,
            name VARCHAR(20) NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (image_id) REFERENCES Files(file_id),
            
            INDEX (name)
        ) ENGINE=InnoDB;
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Post (
            post_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
            header VARCHAR(20) NOT NULL,
            body VARCHAR(510) NOT NULL,
            has_embed BOOL NOT NULL,
            embed_id INT,
            poster INT NOT NULL,
            subicron INT NOT NULL,
            upvotes INT DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (embed_id) REFERENCES Files(file_id),
            FOREIGN KEY (poster) REFERENCES Accounts(account_id) ON DELETE CASCADE,
            FOREIGN KEY (subicron) REFERENCES Subicron(subicron_id) ON DELETE CASCADE,

            INDEX (header),
            INDEX (body)
        ) ENGINE=InnoDB;
    "#
];