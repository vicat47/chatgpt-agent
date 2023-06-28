/*
 Navicat Premium Data Transfer

 Source Server Type    : PostgreSQL
 Source Server Version : 140007
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 140007
 File Encoding         : 65001

 Date: 29/06/2023 01:27:05
*/


-- ----------------------------
-- Table structure for sys_model_request
-- ----------------------------
DROP TABLE IF EXISTS "public"."sys_model_request";
CREATE TABLE "public"."sys_model_request" (
  "id" int4 NOT NULL DEFAULT nextval('sys_model_request_id_seq'::regclass),
  "remote_id" varchar(255) COLLATE "pg_catalog"."default",
  "model" varchar(100) COLLATE "pg_catalog"."default",
  "define_price" varchar(100) COLLATE "pg_catalog"."default",
  "completion_tokens" int4,
  "prompt_tokens" int4,
  "total_tokens" int4,
  "timestamp" timestamptz(6) DEFAULT clock_timestamp(),
  "price" numeric(10,8)
)
;
COMMENT ON COLUMN "public"."sys_model_request"."remote_id" IS '远端id';
COMMENT ON COLUMN "public"."sys_model_request"."model" IS '模型';
COMMENT ON COLUMN "public"."sys_model_request"."define_price" IS '调用时价格';
COMMENT ON COLUMN "public"."sys_model_request"."completion_tokens" IS '完成 token';
COMMENT ON COLUMN "public"."sys_model_request"."prompt_tokens" IS '提示 token';
COMMENT ON COLUMN "public"."sys_model_request"."total_tokens" IS '总 token 数';
COMMENT ON COLUMN "public"."sys_model_request"."timestamp" IS '调用时间';
COMMENT ON COLUMN "public"."sys_model_request"."price" IS '该条请求的实际价格';

-- ----------------------------
-- Primary Key structure for table sys_model_request
-- ----------------------------
ALTER TABLE "public"."sys_model_request" ADD CONSTRAINT "sys_model_request_pkey" PRIMARY KEY ("id");
