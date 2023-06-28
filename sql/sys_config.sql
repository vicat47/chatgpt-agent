/*
 Navicat Premium Data Transfer

 Source Server Type    : PostgreSQL
 Source Server Version : 140007
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 140007
 File Encoding         : 65001

 Date: 29/06/2023 01:26:55
*/


-- ----------------------------
-- Table structure for sys_config
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_config";
CREATE TABLE "public"."sys_config" (
  "id" int4 NOT NULL DEFAULT nextval('sys_config_id_seq'::regclass),
  "key" varchar(255) COLLATE "pg_catalog"."default",
  "value" varchar(255) COLLATE "pg_catalog"."default",
  "timestamp" timestamptz(6) DEFAULT clock_timestamp(),
  "group" varchar(255) COLLATE "pg_catalog"."default"
)
;
COMMENT ON COLUMN "public"."sys_config"."id" IS 'id';
COMMENT ON COLUMN "public"."sys_config"."key" IS 'key';
COMMENT ON COLUMN "public"."sys_config"."value" IS 'value';
COMMENT ON COLUMN "public"."sys_config"."group" IS 'group';

-- ----------------------------
-- Records of sys_config
-- ----------------------------
INSERT INTO "public"."sys_config" VALUES (2, 'gpt-3.5-turbo-0301_output', '0.002', '2023-06-28 15:57:23.708878+00', 'model_price');
INSERT INTO "public"."sys_config" VALUES (1, 'gpt-3.5-turbo-0301_input', '0.0015', '2023-06-28 15:56:27.080423+00', 'model_price');

-- ----------------------------
-- Primary Key structure for table sys_config
-- ----------------------------
ALTER TABLE "public"."sys_config" ADD CONSTRAINT "sys_config_pkey" PRIMARY KEY ("id");
