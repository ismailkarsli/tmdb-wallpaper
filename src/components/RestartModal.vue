<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";

const emit = defineEmits(["close"]);

const { t } = useI18n({
	inheritLocale: true,
	useScope: "global",
});

const restart = () => {
	invoke("restart_app");
};

const close = () => {
	emit("close");
};
</script>

<template>
	<div
		class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 bottom-0 bg-black/50 z-50 w-screen md:inset-0 md:h-screen">
		<div class="relative p-4 w-full h-full flex items-center justify-center md:h-auto">
			<!-- Modal content -->
			<div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
				<!-- Modal header -->
				<div class="flex justify-between items-start p-4 rounded-t border-b dark:border-gray-600">
					<h3 class="text-xl font-semibold text-gray-900 dark:text-white">{{ t("Restart App") }}</h3>
				</div>
				<!-- Modal body -->
				<div class="p-6 space-y-6 w-full">
					<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
						{{ t("To apply the new settings, you have to restart the app.") }}
					</p>
				</div>
				<!-- Modal footer -->
				<div class="flex items-center p-6 space-x-2 rounded-b border-t border-gray-200 dark:border-gray-600">
					<button
						@click="restart"
						type="button"
						class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						{{ t("Continue") }}
					</button>
					<button
						@click="close"
						type="button"
						class="text-black hover:bg-gray-200 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						{{ t("Cancel") }}
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
