// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}

	interface Conversation {
		id: number;
		title: string;
		created_at: string;
		updated_at: string;
	}

	interface Message {
		role: string;
		content: string;
	}	
}

export {};
