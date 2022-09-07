<script lang="ts" setup>
import { storeToRefs } from "pinia";
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useMainStore } from "../stores/main";

const emit = defineEmits(["close"]);

const { t } = useI18n({
	inheritLocale: true,
	useScope: "global",
});

const mainStore = useMainStore();
const { settings, request_token } = storeToRefs(mainStore);

const response = ref<string | undefined>();

const createSessionId = async () => {
	try {
		await mainStore.createSessionId();
		emit("close");
	} catch (e) {
		response.value = e as string;
	}
};

onMounted(async () => {
	if (settings.value?.session_id) {
		emit("close");
		return;
	}
	await mainStore.createRequestToken();
});
</script>

<template>
	<div
		class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 bottom-0 bg-black/50 z-50 w-screen md:inset-0 md:h-screen">
		<div class="relative p-4 w-full h-full flex items-center justify-center md:h-auto">
			<!-- Modal content -->
			<div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
				<!-- Modal header -->
				<div class="flex justify-between items-start p-4 rounded-t border-b dark:border-gray-600">
					<h3 class="text-xl font-semibold text-gray-900 dark:text-white">{{ t("TMDB_auth") }}</h3>
				</div>
				<!-- Modal body -->
				<div class="p-6 space-y-6 w-full">
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						{{ t("İnfo_1") }}
					</p>
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						{{ t("İnfo_2") }}
					</p>
					<p class="break-all">
						<a
							:href="`https://www.themoviedb.org/authenticate/${request_token}`"
							target="_blank"
							class="text-blue-500 hover:underline"
							>{{ `https://www.themoviedb.org/authenticate/${request_token}` }}
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
						{{ t("Go") }}
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
