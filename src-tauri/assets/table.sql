/*
 Navicat Premium Data Transfer

 Source Server         : toolbox
 Source Server Type    : SQLite
 Source Server Version : 3030001
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3030001
 File Encoding         : 65001

 Date: 26/11/2021 17:07:08
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for category
-- ----------------------------
DROP TABLE IF EXISTS "category";
CREATE TABLE "category" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "parent_id" INTEGER,
  "name" TEXT(50)
);

-- ----------------------------
-- Table structure for command
-- ----------------------------
DROP TABLE IF EXISTS "command";
CREATE TABLE "command" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "project_id" INTEGER,
  "profile" TEXT,
  "remote_dir" TEXT,
  "before" TEXT,
  "after" TEXT,
  "target_name" TEXT,
  "need_upload" INTEGER
);

-- ----------------------------
-- Table structure for datasource
-- ----------------------------
DROP TABLE IF EXISTS "datasource";
CREATE TABLE "datasource" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT(50),
  "db_type" TEXT(20),
  "host" TEXT(50),
  "port" integer,
  "db_name" TEXT(50),
  "prefix" TEXT(20),
  "user" TEXT(20),
  "password" TEXT(20)
);

-- ----------------------------
-- Table structure for gen_project
-- ----------------------------
DROP TABLE IF EXISTS "gen_project";
CREATE TABLE "gen_project" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "project_id" INTEGER(11),
  "datasource" INTEGER(11),
  "output" TEXT,
  "template" TEXT,
  "type_mapping" TEXT
);

-- ----------------------------
-- Table structure for project
-- ----------------------------
DROP TABLE IF EXISTS "project";
CREATE TABLE "project" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT(50),
  "path" TEXT(50)
);

-- ----------------------------
-- Table structure for server
-- ----------------------------
DROP TABLE IF EXISTS "server";
CREATE TABLE "server" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT(120),
  "label" TEXT(120),
  "host" TEXT(120),
  "port" INTEGER(32),
  "user" TEXT(120),
  "password" TEXT(120),
  "private_key" TEXT(500),
  "auth_type" INTEGER(32),
  "command" TEXT(500)
);

-- ----------------------------
-- Table structure for template
-- ----------------------------
DROP TABLE IF EXISTS "template";
CREATE TABLE "template" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "category_id" INTEGER,
  "language" TEXT(50),
  "name" TEXT(50),
  "content" TEXT
);

-- ----------------------------
-- Auto increment value for command
-- ----------------------------

-- ----------------------------
-- Auto increment value for deploy_project
-- ----------------------------
DROP TABLE IF EXISTS "env";
CREATE TABLE "env" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT,
  "value" TEXT
);

DROP TABLE IF EXISTS "quick_deploy";
CREATE TABLE "quick_deploy" (
    "id"   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT,
    "project" INTEGER,
    "profile" INTEGER,
    "server" TEXT
);

PRAGMA foreign_keys = true;
