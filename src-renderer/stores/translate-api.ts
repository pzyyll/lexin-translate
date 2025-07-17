import { v7 as uuid } from 'uuid';

export const useTranslateApiStore = defineStore('translate-api', () => {
	const channels = ref<Translate.Channel[]>([]);

	async function GetChannelById(channelId: string): Promise<Translate.Channel | undefined> {
		if (!channels.value) return undefined;
		return channels.value.find((channel) => channel.id === channelId);
	}

	async function AddChannel(channel: Translate.Channel) {
		if (!channels.value) {
			channels.value = [];
		}

		if (!channel.id) {
			channel.id = uuid();
		}

		console.log(`Adding channel with ID: ${channel}`);

		if (channels.value.some((c) => c.id === channel.id)) {
			return console.error(`Channel with ID ${channel.id} already exists`);
		}

		channels.value.push(channel);
	}

	async function UpdateChannel(
		channel: Partial<Translate.Channel> & Pick<Translate.Channel, 'id'>
	) {
		if (!channels.value) return;

		const index = channels.value.findIndex((c) => c.id === channel.id);
		if (index === -1) {
			return console.error(`Channel with ID ${channel.id} not found`);
		}

		channels.value[index] = { ...channels.value[index], ...channel };
	}

	async function DeleteChannel(channelId: string) {
		if (!channels.value) return;
		channels.value = channels.value.filter((channel) => channel.id !== channelId);
	}

	return {
		channels,
		GetChannelById,
		AddChannel,
		UpdateChannel,
		DeleteChannel
	};
});
