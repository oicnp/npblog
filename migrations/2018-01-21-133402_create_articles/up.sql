-- Your SQL goes here

CREATE TABLE `user` (
    id int(11) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email varchar(255) NOT NULL,
    real_name VARCHAR(128) DEFAULT '',
    password_hash VARCHAR(255) NOT NULL,
    password_reset_token VARCHAR(255) DEFAULT '',
    is_admin TINYINT(1) DEFAULT 0,
    created_at int(11) UNSIGNED DEFAULT 0,
    updated_at int(11) UNSIGNED DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `article` (
    `id` int(11) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `user_id` int(11) UNSIGNED DEFAULT 0,
    `title` varchar(255) NOT NULL,
    `thumb` varchar(255) DEFAULT '',
    `type` varchar(64) DEFAULT '',
    `category` INT(11) DEFAULT 0,
    `excerpt` text,
    `content` text,
    `status` tinyint(1) DEFAULT 0,
    `push_front` tinyint(1) DEFAULT 0,
    `created_at` int(11) UNSIGNED DEFAULT 0,
    `updated_at` int(11) UNSIGNED DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- ----------------------------
-- Table structure for taxonomy
-- ----------------------------
CREATE TABLE `taxonomy` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `vid` varchar(64) COLLATE utf8_unicode_ci NOT NULL COMMENT 'machine name',
  `name` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
  `weight` int(6) DEFAULT 0,
  `description` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

-- ----------------------------
-- Records of taxonomy
-- ----------------------------
BEGIN;
INSERT INTO `taxonomy` VALUES (1, 'content_type', 'Content type', 1, NULL);
INSERT INTO `taxonomy` VALUES (2, 'category', 'Category', 2, NULL);
COMMIT;

-- ----------------------------
-- Table structure for taxonomy_term
-- ----------------------------
CREATE TABLE `taxonomy_term` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `vid` varchar(64) COLLATE utf8_unicode_ci NOT NULL COMMENT 'taxonomy vid',
  `parent` int(11) DEFAULT 0,
  `name` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
  `machine_name` varchar(64) COLLATE utf8_unicode_ci NOT NULL,
  `weight` int(6) DEFAULT 0,
  `num` int(11) DEFAULT 0 COMMENT '引用数量',
  `description` varchar(255) COLLATE utf8_unicode_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

BEGIN;
INSERT INTO `taxonomy_term` VALUES (1, 'content_type', 0, 'Blog', 'blog', 1, 0, '');
COMMIT;

CREATE TABLE `article_taxonomy_term_map` (
  `article_id` int(11) NOT NULL,
  `taxonomy_term_id` int(11) NOT NULL,
  `type` varchar(10) COLLATE utf8_unicode_ci DEFAULT NULL,
  PRIMARY KEY (`article_id`, `taxonomy_term_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;


