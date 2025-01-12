
CREATE TABLE `users` (
    `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    `username` VARCHAR(191) NOT NULL UNIQUE,
    `hashed_password` VARCHAR(191) NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `events` (
    `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    `status` TINYINT NOT NULL,
    `owner_id` INT UNSIGNED NOT NULL,
    `title` VARCHAR(191) NOT NULL,
    `location` VARCHAR(191) NOT NULL ,
    `start_at` TIMESTAMP NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE INDEX start_at ON events(start_at);

