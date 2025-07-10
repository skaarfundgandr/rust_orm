-- Your SQL goes here
CREATE TABLE `restock_history` (
  `restock_id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `inventory_id` INT NOT NULL,
  `quantity` INT NOT NULL,
  `datetime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT `FK_INVENTORY_ID_RESTOCK_HISTORY` FOREIGN KEY (`inventory_id`) REFERENCES `inventory`(`inventory_id`),
  CONSTRAINT `CHK_QUANTITY_RESTOCK_HISTORY` CHECK (`quantity` >= 0)
)