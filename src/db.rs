use rusqlite::{Connection, Result, OptionalExtension}; // Ajout de OptionalExtension ici

#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: i32,
    pub content_type: String,
    pub content: Option<String>,
    pub media_path: Option<String>,
    pub source_app: Option<String>,
    pub is_pinned: bool,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("clipboard_history.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content_type TEXT NOT NULL,
            content TEXT,
            media_path TEXT,
            source_app TEXT,
            is_pinned BOOLEAN DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(conn)
}

pub fn insert_item(conn: &Connection, item: &ClipboardItem) -> Result<()> {
    // Check si le contenu existe déjà dans la base de données pour éviter les doublons
    if item.content_type == "text" {
        if let Some(ref text_content) = item.content {
            let mut stmt = conn.prepare("SELECT id FROM clipboard_history WHERE content = ?1 AND content_type = 'text'")?;
            
            let existing_id: Option<i32> = stmt.query_row([text_content], |row| row.get(0)).optional()?;

            if let Some(id) = existing_id {
                // Si le texte existe déjà, on met à jour la date de création pour le rafraîchir
                conn.execute(
                    "UPDATE clipboard_history SET created_at = CURRENT_TIMESTAMP WHERE id = ?1",
                    [id],
                )?;
                return Ok(());
            }
        }
    }

    // Nouveau texte ou un autre type de contenu (image, etc.), on l'insère dans la base de données
    conn.execute(
        "INSERT INTO clipboard_history (content_type, content, media_path, source_app, is_pinned)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &item.content_type,
            &item.content,
            &item.media_path,
            &item.source_app,
            &item.is_pinned,
        ),
    )?;
    Ok(())
}

pub fn get_recent_texts(conn: &Connection, limit: u32) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT content FROM clipboard_history 
         WHERE content_type = 'text' AND content IS NOT NULL 
         ORDER BY created_at DESC 
         LIMIT ?1"
    )?;
    
    let rows = stmt.query_map([limit], |row| row.get(0))?;
    
    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }
    
    Ok(items)
}