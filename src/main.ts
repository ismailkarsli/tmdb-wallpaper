import "./styles.css";
import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import { createPinia } from "pinia";
import App from "./App.vue";

import en from "./locales/en.json";
import tr from "./locales/tr.json";

const locale = navigator.language.split("-")[0] || "en";

const i18n = createI18n({
	locale: locale,
	legacy: false,
	globalInjection: true,
	messages: {
		en,
		tr,
	},
});
const pinia = createPinia();
const app = createApp(App);

app.use(i18n);
app.use(pinia);
app.mount("#app");
