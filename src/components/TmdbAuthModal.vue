<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useMainStore } from "../stores/main";

const emit = defineEmits(["close"]);

const { t } = useI18n({
	inheritLocale: true,
	useScope: "global",
});

const mainStore = useMainStore();
const tmdb_api_key = ref(mainStore.settings.tmdb_api_key);
const session_id = ref(mainStore.settings.session_id);

const response = ref<string | undefined>();

const saveApi = async () => {
	mainStore.settings.tmdb_api_key = tmdb_api_key.value;
	await mainStore.saveSettings();
	try {
		await mainStore.createRequestToken();
	} catch (e) {
		mainStore.settings.tmdb_api_key = undefined;
		await mainStore.saveSettings();
		response.value = "Invalid API Key";
	}
};

const createSessionId = async () => {
	try {
		await mainStore.createSessionId();
		emit("close");
	} catch (e) {
		response.value = e as string;
	}
};

onMounted(async () => {
	if (mainStore.settings.tmdb_api_key && mainStore.settings.session_id) {
		emit("close");
		return;
	}

	if (mainStore.settings.tmdb_api_key) {
		await mainStore.createRequestToken();
	}
});
</script>

<template>
	<div
		class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 bottom-0 bg-black/50 z-50 w-screen md:inset-0 md:h-screen">
		<div class="relative p-4 w-full h-full flex items-center justify-center md:h-auto">
			<!-- TMDB Api key -->
			<div v-if="!mainStore.settings.tmdb_api_key" class="relative bg-white rounded-lg shadow dark:bg-gray-700">
				<!-- Modal header -->
				<div class="flex flex-col p-4 pb-2 rounded-t">
					<h3 class="text-xl font-semibold text-gray-900 dark:text-white">{{ t("TMDB_auth") }}</h3>
					<div class="flex mt-2">
						<span class="block mr-1 h-1 bg-sky-700 w-1/2" />
						<span class="block ml-1 h-1 bg-gray-300 w-1/2" />
					</div>
				</div>
				<!-- Modal body -->
				<div class="p-6 space-y-6 w-full">
					<div class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						{{ t("In order to use the application, you must first get TMDb API key from:") }}
						<a href="https://www.themoviedb.org/settings/api" target="_blank" class="text-sky-700">
							https://www.themoviedb.org/settings/api
						</a>
						<div>{{ t("Then enter it below:") }}</div>
					</div>
					<div class="flex flex-col">
						<label class="text-sm font-semibold text-gray-500 dark:text-gray-400">{{ t("TMDB API key") }}</label>
						<input
							v-model="tmdb_api_key"
							type="text"
							class="w-full px-4 py-2 mt-2 text-gray-700 bg-gray-200 border-2 border-transparent rounded-lg dark:bg-gray-800 dark:text-gray-300 focus:border-sky-500 focus:bg-white focus:outline-none" />
					</div>
					<p v-if="response" class="break-all text-base leading-relaxed text-gray-500 dark:text-gray-400">
						<b>{{ t("Response") }}: </b>{{ response }}
					</p>
				</div>
				<!-- Modal footer -->
				<div class="flex items-center p-6 space-x-2 rounded-b border-t border-gray-200 dark:border-gray-600">
					<button
						@click="saveApi"
						type="button"
						:disabled="!tmdb_api_key"
						class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 disabled:bg-gray-400">
						{{ t("Continue") }}
					</button>
				</div>
			</div>
			<!-- Session id -->
			<div v-else class="relative bg-white rounded-lg shadow dark:bg-gray-700">
				<!-- Modal header -->
				<div class="flex flex-col p-4 pb-2 rounded-t">
					<h3 class="text-xl font-semibold text-gray-900 dark:text-white">{{ t("TMDB_auth") }}</h3>
					<div class="flex mt-2">
						<span class="block mr-1 h-1 bg-sky-700 w-1/2" />
						<span class="block ml-1 h-1 bg-sky-700 w-1/2" />
					</div>
				</div>
				<!-- Modal body -->
				<div class="p-6 space-y-6 w-full">
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						{{ t("İnfo_1") }}
					</p>
					<p class="break-all">
						<a
							:href="`https://www.themoviedb.org/authenticate/${mainStore.request_token}`"
							target="_blank"
							class="text-blue-500 hover:underline"
							>{{ `https://www.themoviedb.org/authenticate/${mainStore.request_token}` }}
						</a>
					</p>
					<p v-if="response" class="break-all text-base leading-relaxed text-gray-500 dark:text-gray-400">
						<b>{{ t("Response") }}: </b>{{ response }}
					</p>
				</div>
				<!-- Modal footer -->
				<div class="flex items-center p-6 space-x-2 rounded-b border-t border-gray-200 dark:border-gray-600">
					<button
						@click="createSessionId"
						type="button"
						class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						{{ t("Continue") }}
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
