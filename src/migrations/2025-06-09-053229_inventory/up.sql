CREATE TABLE `inventory` (
    `inventory_id` INT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    `product_id` INT NOT NULL,
    `quantity` INT NOT NULL,
    CONSTRAINT `FK_PRODUCT_ID_INVENTORY` FOREIGN KEY (`product_id`) REFERENCES `products`(`product_id`),
    CONSTRAINT `CHK_QUANTITY` CHECK (`quantity` >= 0)
)