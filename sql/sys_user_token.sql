/*
 Navicat Premium Data Transfer

 Source Server Type    : PostgreSQL
 Source Server Version : 140007
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 140007
 File Encoding         : 65001

 Date: 29/06/2023 01:27:14
*/


-- ----------------------------
-- Table structure for sys_user_token
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_user_token";
CREATE TABLE "public"."sys_user_token" (
  "id" int4 NOT NULL DEFAULT nextval('sys_user_token_id_seq'::regclass),
  "name" varchar(20) COLLATE "pg_catalog"."default",
  "local_token" varchar(100) COLLATE "pg_catalog"."default",
  "gpt_token" varchar(100) COLLATE "pg_catalog"."default",
  "comment" varchar(255) COLLATE "pg_catalog"."default",
  "create_time" timestamptz(6) DEFAULT clock_timestamp()
)
;

-- ----------------------------
-- Primary Key structure for table sys_user_token
-- ----------------------------
ALTER TABLE "public"."sys_user_token" ADD CONSTRAINT "sys_user_token_pkey" PRIMARY KEY ("id");
