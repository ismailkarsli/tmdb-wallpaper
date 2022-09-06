interface Settings {
	tmdb_api_key?: string;
	movies: boolean;
	tv: boolean;
	list_type: "favorite" | "watchlist";
	fetch_period: "every minute" | "every half hour" | "hourly" | "half day" | "daily" | "weekly" | "monthly";
	filter_photos_with_text: boolean;
	language_of_photos: string;
	width?: number;
	height?: number;
	session_id?: string;
}
