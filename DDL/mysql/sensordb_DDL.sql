-- Execute: mysql -u root -p < sensordb_DDL.sql

SET GLOBAL max_allowed_packet=1073741824;
SET @@global.sql_mode= '';

DROP DATABASE IF EXISTS `sensors`;

DROP USER 'data_api'@'%';

FLUSH PRIVILEGES;

CREATE USER IF NOT EXISTS 'data_api'@'%' IDENTIFIED BY 'data_api';

GRANT ALL PRIVILEGES ON *.* TO 'data_api'@'%';

CREATE DATABASE IF NOT EXISTS `sensors`
CHARACTER SET utf8
COLLATE utf8_general_ci;

USE `sensors`;

CREATE TABLE IF NOT EXISTS `sensors`.`sensors_users` (
  `user_id` INT(11) unsigned NOT NULL AUTO_INCREMENT,
  `username` VARCHAR(150) DEFAULT NULL,
  `password` VARCHAR(150) DEFAULT NULL,
  `name` VARCHAR(150) COLLATE utf8_general_ci DEFAULT NULL,
  `surname` VARCHAR(150) COLLATE utf8_general_ci DEFAULT NULL,
  `description` VARCHAR(150) COLLATE utf8_general_ci DEFAULT NULL,
  `creation_ts_user` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `ts_last_update` DATETIME DEFAULT NULL,
  `enabled` BOOLEAN NOT NULL DEFAULT 1,
  `is_admin` BOOLEAN NOT NULL DEFAULT 0,
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

CREATE TABLE IF NOT EXISTS `sensors`.`sensors_tokens` (
  `token_user_id` INT(11) unsigned NOT NULL,
  `token` VARCHAR(150) COLLATE utf8_general_ci DEFAULT NULL,
  `creation_ts_token` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `expired` BOOLEAN NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci;

TRUNCATE TABLE `sensors`.`sensors_users`;
TRUNCATE TABLE `sensors`.`sensors_tokens`;

-- User: admin / Passord: admin1234
INSERT INTO `sensors_users` (`user_id`, `username`, `password`, `name`, `surname`, `description`, `creation_ts_user`, `ts_last_update`, `enabled`, `is_admin`)
VALUES (1, 'admin', '8d58e07ebe6faa4f35568dc01fe63152176a06e7e277759f4d9db51bbe0c4cc0', 'Sergio', 'Martinez Losa', 'Admin API user', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 1);

INSERT INTO `sensors_tokens` (`token_user_id`, `token`, `creation_ts_token`, `expired`)
VALUES (1, 'aca6038665c811e8a96100089be8caec', CURRENT_TIMESTAMP, 0);

-- User: api_user / Passord: api_user1234
INSERT INTO `sensors_users` (`user_id`, `username`, `password`, `name`, `surname`, `description`, `creation_ts_user`, `ts_last_update`, `enabled`, `is_admin`)
VALUES (2, 'api_user', '06e242e1ee293f4d2f622376f03dd732ec8a725bb35bf73e553444664c3d64d5', 'Sergio', 'Martinez Losa', 'API user', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 0);

INSERT INTO `sensors_tokens` (`token_user_id`, `token`, `creation_ts_token`, `expired`)
VALUES (2, '7b774d0765d011e8a96100089be8caec', CURRENT_TIMESTAMP, 0);

-- Insert a new token when user inserts new data
DROP TRIGGER IF EXISTS `sensors`.`trigger_insert_sensor_tokensnew_user`;
DROP PROCEDURE IF EXISTS `sensors`.`insert_new_row_sensors_tokens`;

DELIMITER //
CREATE TRIGGER `sensors`.`trigger_insert_sensor_tokens`
AFTER INSERT ON `sensors`.`sensors_users` FOR EACH ROW
    BEGIN
      CALL `sensors`.`insert_new_row_sensors_tokens`(NEW.`user_id`);
    END; //
DELIMITER ;

DELIMITER //
CREATE PROCEDURE `sensors`.`insert_new_row_sensors_tokens`(IN new_user_id INT(11))
MODIFIES SQL DATA
    BEGIN
		 SET @token_id := new_user_id;
		 SET @token := (SELECT REPLACE(LOWER(LEFT(UUID(), 110)), '-', ''));
     SET @date_now := NOW();

     INSERT INTO `sensors`.`sensors_tokens` (`token_user_id`, `token`, `creation_ts_token`)
     VALUES (@token_id, @token, @date_now);

    END; //
DELIMITER ;

-- Delete tokens after user deletes its data
DROP TRIGGER IF EXISTS `sensors`.`trigger_delete_sensor_tokens`;

DELIMITER //
CREATE TRIGGER `sensors`.`trigger_delete_sensor_tokens`
AFTER DELETE ON `sensors`.`sensors_users` FOR EACH ROW
    BEGIN

      SET @token_id := OLD.`user_id`;

     	DELETE FROM `sensors`.`sensors_tokens`
      WHERE `sensors_tokens`.`token_user_id` = @token_id;

    END; //
DELIMITER ;

-- Create procudure to manage user login
DROP PROCEDURE IF EXISTS `sensors`.`login_user_actions`;

DELIMITER //
CREATE PROCEDURE `sensors`.`login_user_actions`(IN username VARCHAR(150), IN password VARCHAR(150))
MODIFIES SQL DATA
    BEGIN

     SET @temp_username := username;
		 SET @temp_password := password;

     SET @temp_user_id := (SELECT user_id FROM `sensors`.`sensors_users`
                           WHERE `sensors_users`.`username` = @temp_username AND `sensors_users`.`password` = @temp_password LIMIT 1);

     SET @is_token_expired := (SELECT s2.expired FROM `sensors`.`sensors_users` AS s1 INNER JOIN `sensors`.`sensors_tokens` AS s2
                               ON s1.`user_id` = s2.`token_user_id`
                               WHERE s1.`username` = @temp_username AND s1.`password` = @temp_password LIMIT 1);


     IF @is_token_expired = 1 THEN

        DELETE FROM `sensors`.`sensors_tokens`
        WHERE `sensors_tokens`.`token_user_id` = @temp_user_id;

        CALL `sensors`.`insert_new_row_sensors_tokens`(@temp_user_id);

     END IF;

     SET @token_value := (SELECT token FROM `sensors`.`sensors_tokens` WHERE `sensors_tokens`.`token_user_id` = @temp_user_id);
     SET @user_id_value := @temp_user_id;

     SELECT @token_value AS token, @user_id_value AS user_id;
    END; //
DELIMITER ;

-- Create an event that checks tokens' timestamps, if timestamp >= 24 hours token expires
DROP EVENT IF EXISTS `sensors`.`event_expire_tokens`;
DROP PROCEDURE IF EXISTS `sensors`.`procedure_expire_timestamps`;

SET GLOBAL event_scheduler = ON;

CREATE EVENT `sensors`.`event_expire_tokens`
ON SCHEDULE EVERY 1 HOUR
COMMENT 'Ssets expired = 0 on sensors.sensors_tokens >= 24 hours'
    DO CALL `sensors`.`procedure_expire_timestamps`();

DELIMITER //

CREATE PROCEDURE `sensors`.`procedure_expire_timestamps`()
MODIFIES SQL DATA
BEGIN
    DECLARE done INTEGER DEFAULT 0;
  	DECLARE v_creation_ts TIMESTAMP;
  	DECLARE v_token_id INT(11);
  	DECLARE v_token VARCHAR(150);
  	DECLARE cursor_tokens CURSOR FOR SELECT token_user_id, token, creation_ts_token FROM `sensors`.`sensors_tokens` AS s2 INNER JOIN `sensors`.`sensors_users` AS s1 ON s2.`token_user_id` = s1.`user_id` WHERE s1.`is_admin` = 0 AND  s2.`expired` = 0;
  	DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = 1;

	  OPEN cursor_tokens;

  	read_loop: LOOP
      FETCH cursor_tokens INTO v_token_id, v_token, v_creation_ts;
      	IF done = TRUE THEN
        		LEAVE read_loop;
      	END IF;

      	# Do substraction for creation_ts
      	SET @diff_days := (SELECT CAST(DATEDIFF(CURDATE(), v_creation_ts) AS UNSIGNED));

        # Date diff bigger or equal than one day
      	IF @diff_days >= 1 THEN

      		DELETE FROM `sensors`.`sensors_tokens` WHERE `sensors_tokens`.`token_user_id` = v_token_id;

      		INSERT INTO `sensors`.`sensors_tokens` (`token_user_id`, `token`, `creation_ts_token`, `expired`)
      		VALUES (v_token_id, v_token, CURRENT_TIMESTAMP, 1);

    	END IF;

  	END LOOP;

  	CLOSE cursor_tokens;
END; //
DELIMITER ;

COMMIT;
