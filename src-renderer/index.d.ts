import translate from '~/pages/translate.vue';

declare global {
	declare namespace Translate {
		declare interface TranslateTypeInfo {
			name: string;
			icon: any;
			sort: number;
			api_type: string;
		}

		declare type TranslateType = {
			[key: string]: TranslateTypeInfo;
		};

		declare interface Channel {
			id: string;
			name: string;
			api_type: string;
			api_config: any;
			enable: boolean;
		}

		declare interface ChannelProtocol {
			TestConnect: (channel: Channel) => Promise<boolean>;
			Translate: (
				channel: Channel,
				source: string,
				sourceLanguage: string,
				targetLanguage: string,
				extra?: Record<string, any>
			) => Promise<string>;
		}
	}
}

declare module 'nuxt/schema' {
	interface AppConfigInput {
		/** Theme configuration */
		title?: string;
		snapEdgeSize: number;
		default_language: string;
	}
}

export {};
