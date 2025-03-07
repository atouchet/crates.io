#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `api_tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    api_tokens (id) {
        /// The `id` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `token` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Bytea`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Bytea,
        /// The `name` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `created_at` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `last_used_at` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        last_used_at -> Nullable<Timestamp>,
        /// The `revoked` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        revoked -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `background_jobs` table.
    ///
    /// (Automatically generated by Diesel.)
    background_jobs (id) {
        /// The `id` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `job_type` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        job_type -> Text,
        /// The `data` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        data -> Jsonb,
        /// The `retries` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        retries -> Int4,
        /// The `last_retry` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        last_retry -> Timestamp,
        /// The `created_at` column of the `background_jobs` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `badges` table.
    ///
    /// (Automatically generated by Diesel.)
    badges (crate_id, badge_type) {
        /// The `crate_id` column of the `badges` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `badge_type` column of the `badges` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        badge_type -> Varchar,
        /// The `attributes` column of the `badges` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        attributes -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `categories` table.
    ///
    /// (Automatically generated by Diesel.)
    categories (id) {
        /// The `id` column of the `categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `category` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Varchar,
        /// The `slug` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        slug -> Varchar,
        /// The `description` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `crates_cnt` column of the `categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crates_cnt -> Int4,
        /// The `created_at` column of the `categories` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `crate_owner_invitations` table.
    ///
    /// (Automatically generated by Diesel.)
    crate_owner_invitations (invited_user_id, crate_id) {
        /// The `invited_user_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invited_user_id -> Int4,
        /// The `invited_by_user_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invited_by_user_id -> Int4,
        /// The `crate_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `created_at` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `token` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Text,
        /// The `token_generated_at` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        token_generated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `crate_owners` table.
    ///
    /// (Automatically generated by Diesel.)
    crate_owners (crate_id, owner_id, owner_kind) {
        /// The `crate_id` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `owner_id` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        owner_id -> Int4,
        /// The `created_at` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `created_by` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        created_by -> Nullable<Int4>,
        /// The `deleted` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        deleted -> Bool,
        /// The `updated_at` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `owner_kind` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        owner_kind -> Int4,
        /// The `email_notifications` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        email_notifications -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `crates` table.
    ///
    /// (Automatically generated by Diesel.)
    crates (id) {
        /// The `id` column of the `crates` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `crates` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `updated_at` column of the `crates` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `created_at` column of the `crates` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `downloads` column of the `crates` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `description` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Varchar>,
        /// The `homepage` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        homepage -> Nullable<Varchar>,
        /// The `documentation` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        documentation -> Nullable<Varchar>,
        /// The `readme` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        readme -> Nullable<Varchar>,
        /// The `textsearchable_index_col` column of the `crates` table.
        ///
        /// Its SQL type is `Tsvector`.
        ///
        /// (Automatically generated by Diesel.)
        textsearchable_index_col -> Tsvector,
        /// The `repository` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Nullable<Varchar>,
        /// The `max_upload_size` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        max_upload_size -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `crates_categories` table.
    ///
    /// (Automatically generated by Diesel.)
    crates_categories (crate_id, category_id) {
        /// The `crate_id` column of the `crates_categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `category_id` column of the `crates_categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        category_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `crates_keywords` table.
    ///
    /// (Automatically generated by Diesel.)
    crates_keywords (crate_id, keyword_id) {
        /// The `crate_id` column of the `crates_keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `keyword_id` column of the `crates_keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        keyword_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `dependencies` table.
    ///
    /// (Automatically generated by Diesel.)
    dependencies (id) {
        /// The `id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `version_id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `crate_id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `req` column of the `dependencies` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        req -> Varchar,
        /// The `optional` column of the `dependencies` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        optional -> Bool,
        /// The `default_features` column of the `dependencies` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        default_features -> Bool,
        /// The `features` column of the `dependencies` table.
        ///
        /// Its SQL type is `Array<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        features -> Array<Text>,
        /// The `target` column of the `dependencies` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        target -> Nullable<Varchar>,
        /// The `kind` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        kind -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `emails` table.
    ///
    /// (Automatically generated by Diesel.)
    emails (id) {
        /// The `id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `email` column of the `emails` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `verified` column of the `emails` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        verified -> Bool,
        /// The `token` column of the `emails` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Text,
        /// The `token_generated_at` column of the `emails` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        token_generated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `follows` table.
    ///
    /// (Automatically generated by Diesel.)
    follows (user_id, crate_id) {
        /// The `user_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `crate_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `keywords` table.
    ///
    /// (Automatically generated by Diesel.)
    keywords (id) {
        /// The `id` column of the `keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `keyword` column of the `keywords` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        keyword -> Text,
        /// The `crates_cnt` column of the `keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crates_cnt -> Int4,
        /// The `created_at` column of the `keywords` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `metadata` table.
    ///
    /// (Automatically generated by Diesel.)
    metadata (total_downloads) {
        /// The `total_downloads` column of the `metadata` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        total_downloads -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `publish_limit_buckets` table.
    ///
    /// (Automatically generated by Diesel.)
    publish_limit_buckets (user_id) {
        /// The `user_id` column of the `publish_limit_buckets` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `tokens` column of the `publish_limit_buckets` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tokens -> Int4,
        /// The `last_refill` column of the `publish_limit_buckets` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        last_refill -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `publish_rate_overrides` table.
    ///
    /// (Automatically generated by Diesel.)
    publish_rate_overrides (user_id) {
        /// The `user_id` column of the `publish_rate_overrides` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `burst` column of the `publish_rate_overrides` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        burst -> Int4,
        /// The `expires_at` column of the `publish_rate_overrides` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `readme_renderings` table.
    ///
    /// (Automatically generated by Diesel.)
    readme_renderings (version_id) {
        /// The `version_id` column of the `readme_renderings` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `rendered_at` column of the `readme_renderings` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        rendered_at -> Timestamp,
    }
}

table! {
    /// Representation of the `recent_crate_downloads` view.
    ///
    /// This data represents the downloads in the last 90 days.
    /// This view does not contain realtime data.
    /// It is refreshed by the `update-downloads` script.
    recent_crate_downloads (crate_id) {
        /// The `crate_id` column of the `recent_crate_downloads` view.
        ///
        /// Its SQL type is `Integer`.
        crate_id -> Integer,
        /// The `downloads` column of the `recent_crate_downloads` table.
        ///
        /// Its SQL type is `BigInt`.
        downloads -> BigInt,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `reserved_crate_names` table.
    ///
    /// (Automatically generated by Diesel.)
    reserved_crate_names (name) {
        /// The `name` column of the `reserved_crate_names` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `teams` table.
    ///
    /// (Automatically generated by Diesel.)
    teams (id) {
        /// The `id` column of the `teams` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `login` column of the `teams` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        login -> Varchar,
        /// The `github_id` column of the `teams` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        github_id -> Int4,
        /// The `name` column of the `teams` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Nullable<Varchar>,
        /// The `avatar` column of the `teams` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        avatar -> Nullable<Varchar>,
        /// The `org_id` column of the `teams` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        org_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `gh_access_token` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        gh_access_token -> Varchar,
        /// The `gh_login` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        gh_login -> Varchar,
        /// The `name` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Nullable<Varchar>,
        /// The `gh_avatar` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        gh_avatar -> Nullable<Varchar>,
        /// The `gh_id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        gh_id -> Int4,
        /// The `account_lock_reason` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        account_lock_reason -> Nullable<Varchar>,
        /// The `account_lock_until` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        account_lock_until -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `version_downloads` table.
    ///
    /// (Automatically generated by Diesel.)
    version_downloads (version_id, date) {
        /// The `version_id` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `downloads` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `counted` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        counted -> Int4,
        /// The `date` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Date`.
        ///
        /// (Automatically generated by Diesel.)
        date -> Date,
        /// The `processed` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        processed -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `version_owner_actions` table.
    ///
    /// (Automatically generated by Diesel.)
    version_owner_actions (id) {
        /// The `id` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `version_id` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `user_id` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `api_token_id` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        api_token_id -> Nullable<Int4>,
        /// The `action` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        action -> Int4,
        /// The `time` column of the `version_owner_actions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `versions` table.
    ///
    /// (Automatically generated by Diesel.)
    versions (id) {
        /// The `id` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `crate_id` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `num` column of the `versions` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        num -> Varchar,
        /// The `updated_at` column of the `versions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `created_at` column of the `versions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `downloads` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `features` column of the `versions` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        features -> Jsonb,
        /// The `yanked` column of the `versions` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        yanked -> Bool,
        /// The `license` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        license -> Nullable<Varchar>,
        /// The `crate_size` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        crate_size -> Nullable<Int4>,
        /// The `published_by` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        published_by -> Nullable<Int4>,
        /// The `checksum` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Bpchar>`.
        ///
        /// (Automatically generated by Diesel.)
        checksum -> Nullable<Bpchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    /// Representation of the `versions_published_by` table.
    ///
    /// (Automatically generated by Diesel.)
    versions_published_by (version_id) {
        /// The `version_id` column of the `versions_published_by` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `email` column of the `versions_published_by` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
    }
}

joinable!(api_tokens -> users (user_id));
joinable!(badges -> crates (crate_id));
joinable!(crate_owner_invitations -> crates (crate_id));
joinable!(crate_owners -> crates (crate_id));
joinable!(crate_owners -> teams (owner_id));
joinable!(crate_owners -> users (owner_id));
joinable!(crates_categories -> categories (category_id));
joinable!(crates_categories -> crates (crate_id));
joinable!(crates_keywords -> crates (crate_id));
joinable!(crates_keywords -> keywords (keyword_id));
joinable!(dependencies -> crates (crate_id));
joinable!(dependencies -> versions (version_id));
joinable!(emails -> users (user_id));
joinable!(follows -> crates (crate_id));
joinable!(follows -> users (user_id));
joinable!(publish_limit_buckets -> users (user_id));
joinable!(publish_rate_overrides -> users (user_id));
joinable!(readme_renderings -> versions (version_id));
joinable!(recent_crate_downloads -> crates (crate_id));
joinable!(version_downloads -> versions (version_id));
joinable!(version_owner_actions -> api_tokens (api_token_id));
joinable!(version_owner_actions -> users (user_id));
joinable!(version_owner_actions -> versions (version_id));
joinable!(versions -> crates (crate_id));
joinable!(versions -> users (published_by));
joinable!(versions_published_by -> versions (version_id));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    background_jobs,
    badges,
    categories,
    crate_owner_invitations,
    crate_owners,
    crates,
    crates_categories,
    crates_keywords,
    dependencies,
    emails,
    follows,
    keywords,
    metadata,
    publish_limit_buckets,
    publish_rate_overrides,
    readme_renderings,
    recent_crate_downloads,
    reserved_crate_names,
    teams,
    users,
    version_downloads,
    version_owner_actions,
    versions,
    versions_published_by,
);
