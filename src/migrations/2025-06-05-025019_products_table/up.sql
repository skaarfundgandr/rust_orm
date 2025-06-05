-- Your SQL goes here
CREATE TABLE `products` (
    `product_id` INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    `product_name` VARCHAR(50) NOT NULL,
    `product_desc` TEXT,
    `product_category` TEXT NOT NULL,
    `product_price` FLOAT NOT NULL,
    CONSTRAINT `CHK_PRODNAME` UNIQUE (`product_name`),
    CONSTRAINT `CHK_PRODPRICE` CHECK (`product_price` >= 0)
)