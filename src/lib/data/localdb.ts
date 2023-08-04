import Database from 'tauri-plugin-sql-api';

const openDB = async () => {
	// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
	const db = await Database.load('sqlite:caspr.db');
	// create a table

	// Each conversation will have many messages and each message will have only one conversation
	await db.execute(
		'CREATE TABLE IF NOT EXISTS conversations (id INTEGER PRIMARY KEY, title TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)'
	);

	await db.execute(
		'CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY, conversation_id INTEGER NOT NULL, role TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY(conversation_id) REFERENCES conversations(id) ON DELETE CASCADE)'
	);

	return db;
};

const closeDB = async (db: Database) => {
	await db.close();
};

export const getConversations = async (callback: (() => void) | undefined = undefined) => {
	const db = await openDB();
	const conversations = await db.select('SELECT * FROM conversations ORDER BY updated_at DESC');
	await closeDB(db);

	if (callback) callback();

	return conversations as Conversation[];
};

export const getConversation = async (id: number) => {
	const db = await openDB();
	const messages = await db.select('SELECT * FROM messages WHERE conversation_id = ?', [id]);
	await closeDB(db);

	return messages as Message[];
};

export const addConversation = async (messages: { role: string; content: string }[]) => {
	const db = await openDB();
	const now = new Date().toISOString();
	const result = await db.execute(
		'INSERT INTO conversations (title, created_at, updated_at) VALUES (?, ?, ?)',
		[messages[0].content.substring(0, 50), now, now]
	);
	for (const message of messages) {
		await db.execute(
			'INSERT INTO messages (conversation_id, role, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)',
			[result.lastInsertId, message.role, message.content, now, now]
		);
	}

	await closeDB(db);

	return result;
};

export const deleteConversation = async (id: number) => {
	const db = await openDB();
	await db.execute('DELETE FROM conversations WHERE id = ? ', [id]);
	await closeDB(db);
};

export const deleteAllConversations = async () => {
	const db = await openDB();
	await db.execute('DELETE FROM conversations');
	await closeDB(db);
};

export const updateConversation = async (
	id: number,
	messages: { role: string; content: string }[]
) => {
	const db = await openDB();
	const now = new Date().toISOString();
	const results = [];

	for (const message of messages) {
		results.push(
			await db.execute(
				'INSERT INTO messages (conversation_id, role, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)',
				[id, message.role, message.content, now, now]
			)
		);
	}

	await closeDB(db);
	return results;
};
