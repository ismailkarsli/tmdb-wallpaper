<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";

const periods = [
	{ name: "Dakika", value: "every minute" },
	{ name: "Yarım saat", value: "every half hour" },
	{ name: "Saat", value: "hourly" },
	{ name: "12 saat", value: "half day" },
	{ name: "Gün", value: "daily" },
	{ name: "Hafta", value: "weekly" },
	{ name: "Ay", value: "monthly" },
];

const settings = ref<Settings>({
	movies: true,
	tv: true,
	list_type: "watchlist",
	fetch_period: "daily",
	filter_photos_with_text: false,
	language_of_photos: "en",
	width: 1280,
	height: 720,
});

const response = ref("");

const onSubmit = async () => {
	const res = await invoke("save_settings", {
		settings: JSON.stringify(settings.value),
	});
	response.value = String(res);
	//TODO: show a dialog to the user that they need to restart the app
};

onMounted(async () => {
	const res = await invoke("get_settings");
	settings.value = JSON.parse(String(res));
});
</script>

<template>
	<div>
		<h1>Uygulama ayarları</h1>
		<form @submit.prevent="onSubmit">
			<b>TMDb API Key</b>
			<div>
				<label>
					<input type="text" v-model="settings.tmdb_api_key" />
				</label>
			</div>
			<b>Yenileme sıklığı</b>
			<div>
				<select v-model="settings.fetch_period">
					<option v-for="period in periods" :value="period.value">
						{{ period.name }}
					</option>
				</select>
			</div>
			<b>Hangi türden wallpaper çekmek istiyorsunuz?</b>
			<div>
				<label>
					<input type="checkbox" v-model="settings.movies" />
					Filmler
				</label>
				<label>
					<input type="checkbox" v-model="settings.tv" />
					Diziler
				</label>
			</div>
			<b>Fotoğraflar hangi listeden alınsın?</b>
			<div>
				<label>
					<input type="radio" value="watchlist" v-model="settings.list_type" />
					İzleme listesi
				</label>
				<label>
					<input type="radio" value="favorites" v-model="settings.list_type" />
					Favoriler
				</label>
			</div>
			<b>Üzerinde yazı olan fotoğraflar filtrelensin mi?</b>
			<div>
				<label>
					<input type="checkbox" v-model="settings.filter_photos_with_text" />
					Filtrele
				</label>
				<div v-if="!settings.filter_photos_with_text">
					<label>
						Yazı dili
						<input v-model="settings.language_of_photos" />
					</label>
				</div>
			</div>
			<b>İstenen fotoğraf çözünürlüğü</b>
			<div>
				<label>
					Genişlik
					<input type="number" v-model="settings.width" />
				</label>
				<label>
					Yükseklik
					<input type="number" v-model="settings.height" />
				</label>
			</div>
			<button type="submit">Kaydet</button>
		</form>
	</div>
</template>
