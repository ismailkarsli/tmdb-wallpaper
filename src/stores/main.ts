import { defineStore } from "pinia";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api";

interface State {
	settings: Settings;
	request_token: string;
}

export const useMainStore = defineStore("main", {
	state: (): State => ({
		settings: {
			movies: true,
			tv: true,
			list_type: "watchlist",
			fetch_period: "daily",
			filter_photos_with_text: true,
			language_of_photos: "en",
			width: 1280,
			height: 720,
		},
		request_token: "",
	}),

	actions: {
		async getSettings() {
			const res = await invoke("get_settings");
			this.settings = JSON.parse(String(res));
		},
		async saveSettings(): Promise<string> {
			return await invoke("save_settings", { settings: JSON.stringify(this.settings) });
		},
		async createRequestToken(): Promise<string | undefined> {
			this.request_token = await invoke("create_request_token");
			return this.request_token;
		},
		async createSessionId() {
			await invoke("create_session_id", {
				requestToken: this.request_token,
			});
			await this.getSettings();
		},
	},
});

export const periods = computed(() => {
	const { t } = useI18n();

	return [
		{ name: t("every minute"), value: "every minute" },
		{ name: t("every half hour"), value: "every half hour" },
		{ name: t("hourly"), value: "hourly" },
		{ name: t("half a day"), value: "half day" },
		{ name: t("daily"), value: "daily" },
		{ name: t("weekly"), value: "weekly" },
		{ name: t("monthly"), value: "monthly" },
	];
});
