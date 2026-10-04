export type User = {
	id: number,
	username: string,
	about: string,
	tags: Tag[],
	achievements: AchievementUnlockedPayload[],
}

export type Tag = {
	id: number,
	name: string,
	category: string,
	description?: string,
}

export type AchievementUnlockedPayload = {
	id: number,
	name: string,
	goals: GoalUnlockedPayload[],
}

export type GoalUnlockedPayload = {
	id: number,
	description: string,
	sequence: number,
	time: Date,
}