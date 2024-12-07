-- Add down migration script here
DROP INDEX users_permission_levels_user_id;

DROP TABLE user_management.permissions_permission_levels;

DROP TABLE user_management.permission_levels;

DROP TABLE user_management.permissions;

DROP INDEX user_management_users_email;

DROP TABLE user_management.items;

DROP SCHEMA user_management;
