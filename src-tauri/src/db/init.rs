use tauri_plugin_sql::{Builder, Migration, MigrationKind};

/// Database migration system for the Tauri app
/// 
/// This module contains all database migrations
/// Each migration version should have its own function for better maintainability.

/// Migration v1: Initial database setup
/// Creates the foundational tables for the application
fn migration_v1() -> Migration {
    Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "
            CREATE TABLE IF NOT EXISTS detectable (
                id TEXT PRIMARY KEY,
                name TEXT,
                aliases TEXT NULL,              -- JSON array as TEXT
                executables TEXT NOT NULL,      -- JSON array as TEXT
                hook BOOLEAN,
                overlay BOOLEAN,
                overlay_compatibility_hook BOOLEAN,
                overlay_methods TEXT,
                overlay_warn BOOLEAN,
                themes TEXT NULL,               -- JSON array as TEXT
                aliases_search TEXT,            -- Lowercase, space-separated for fuzzy search
                themes_search TEXT              -- Lowercase, space-separated for fuzzy search
            );

            -- Index for fast lookup by name
            CREATE INDEX IF NOT EXISTS idx_detectable_name ON detectable(name);

            -- Index for fuzzy search on aliases
            CREATE INDEX IF NOT EXISTS idx_detectable_aliases_search ON detectable(aliases_search);

            -- Index for fuzzy search on themes
            CREATE INDEX IF NOT EXISTS idx_detectable_themes_search ON detectable(themes_search);
        
            CREATE TABLE IF NOT EXISTS app_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT UNIQUE NOT NULL,
                value TEXT,
                description TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Insert default app settings
            INSERT OR IGNORE INTO app_settings (key, value, description) VALUES 
                ('app_version', '25.06.12', 'Current application version'),
                ('db_version', '1', 'Current database schema version'),
                ('default_game_path', '', 'Default directory for game executables');
        ",
        kind: MigrationKind::Up,
    }
}

/// Future migration template (v3)
/// This is a template for future migrations - uncomment and modify as needed
/*
fn migration_v3() -> Migration {
    Migration {
        version: 3,
        description: "add_new_feature",
        sql: "
            -- Add your SQL here
            ALTER TABLE games ADD COLUMN new_column TEXT;
        ",
        kind: MigrationKind::Up,
    }
}
*/

/// Get all database migrations in order
/// 
/// Returns a vector of all migrations that should be applied to the database.
/// Migrations are returned in order of their version numbers.
pub fn get_migrations() -> Vec<Migration> {
    vec![
        migration_v1(),
        migration_v2(),
        // migration_v3(), // Uncomment when ready to use
        // Add future migrations here in order
    ]
}

/// Initialize the database with all migrations
/// 
/// This function should be called during app initialization to ensure
/// the database schema is up to date.
pub fn init_database() -> Builder {
    Builder::default()
        .add_migrations(&get_migrations())
}

/// Helper function to get the current migration version
pub fn get_current_version() -> u32 {
    get_migrations().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_are_ordered() {
        let migrations = get_migrations();
        for (i, migration) in migrations.iter().enumerate() {
            assert_eq!(migration.version, (i + 1) as i64, 
                "Migration version should match its position in the vector");
        }
    }

    #[test]
    fn test_migration_descriptions_exist() {
        let migrations = get_migrations();
        for migration in migrations {
            assert!(!migration.description.is_empty(), 
                "Every migration should have a description");
        }
    }
}