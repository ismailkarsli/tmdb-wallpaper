import { nextTick } from "vue";
import { createI18n } from "vue-i18n";

export class I18n {
	private static _i18n: any;

	public static supportLocales = ["en", "fr"];
	public static fallback = "en";

	public static setupI18n(options: any = { locale: "en" }) {
		this._i18n = createI18n(options);
		this.setI18nLanguage(options.locale);
		this.loadLocaleMessages(options.locale);
		return this._i18n;
	}

	public static get i18n() {
		return this._i18n;
	}

	public static setI18nLanguage(locale: string) {
		if (this._i18n.mode === "legacy") {
			this._i18n.global.locale = locale;
		} else {
			this._i18n.global.locale.value = locale;
		}
		/**
		 * NOTE:
		 * If you need to specify the language setting for headers, such as the `fetch` API, set it here.
		 * The following is an example for axios.
		 *
		 * axios.defaults.headers.common['Accept-Language'] = locale
		 */
		document.querySelector("html")!.setAttribute("lang", locale);
	}

	public static async loadLocaleMessages(locale: string) {
		// load locale messages with dynamic import
		const messages = await import(/* webpackChunkName: "locale-[request]" */ `../locales/${locale}.json`);

		// set locale and locale message
		this._i18n.global.setLocaleMessage(locale, messages.default);

		return nextTick();
	}
}
