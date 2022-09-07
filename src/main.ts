import "./styles.css";
import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";

import en from "./locales/en.json";
import tr from "./locales/tr.json";

const i18n = createI18n({
	locale: "en",
	legacy: false,
	globalInjection: true,
	messages: {
		en,
		tr,
	},
});

createApp(App).use(i18n).mount("#app");
