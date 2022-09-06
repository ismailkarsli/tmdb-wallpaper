<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";

const emit = defineEmits(["close"]);

const settings = ref<Settings>();
const request_token = ref("");
const response = ref("");

const createSessionId = async () => {
	try {
		await invoke("create_session_id", {
			requestToken: request_token.value,
		});
		emit("close");
	} catch (e) {
		response.value = e as string;
	}
};

onMounted(async () => {
	const res = await invoke("get_settings");
	settings.value = JSON.parse(String(res));

	console.log(settings.value);

	if (settings.value?.session_id) {
		emit("close");
		return;
	}

	request_token.value = await invoke("create_request_token");
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
					<h3 class="text-xl font-semibold text-gray-900 dark:text-white">TMDb Yetkilendirme</h3>
				</div>
				<!-- Modal body -->
				<div class="p-6 space-y-6 w-full">
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						Uygulamayı kullanabilmek için öncelikle API key ile yetkilendirme yapmanız gerekmektedir.
					</p>
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						Yetkilendirmek için aşağıdaki linke tıklayarak TMDb üzerinden doğrulama talebini doğrulayın. Sonrasında
						devam et butonuna basın.
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
						<b>Yanıt: </b>{{ response }}
					</p>
				</div>
				<!-- Modal footer -->
				<div class="flex items-center p-6 space-x-2 rounded-b border-t border-gray-200 dark:border-gray-600">
					<button
						@click="createSessionId"
						type="button"
						class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						Devam et
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
