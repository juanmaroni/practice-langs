const urlSet00 = 'https://steamcdn-a.akamaihd.net/apps/583950/resource/card_set_0.79824067E797DFFD5925FE05AB27CF5ACC88723F.json';
const urlSet01 = 'https://steamcdn-a.akamaihd.net/apps/583950/resource/card_set_1.134088E143B4C697AA334924195E1DFB24EC95C5.json';

let lang = navigator.language;

let mapLangCodes = new Map([
	['en', 'english'], ['de', 'german'], ['fr', 'french'], ['it', 'italian'],
	['ko', 'koreana'], ['es', 'spanish'], ['zh-CN', 'schinese'], ['zh-TW', 'tchinese'], 
	['ru', 'russian'], ['th', 'thai'], ['ja', 'japanese'], ['pt', 'portuguese'],
	['pl', 'polish'], ['da', 'danish'], ['da', 'danish'], ['nl', 'dutch'],
	['fi', 'finnish'], 	['no', 'norwegian'], ['sv', 'swedish'], ['hu', 'hungarian'],
	['cs', 'czech'], ['ro', 'romanian'], ['tr', 'turkish'], ['pt-BR', 'brazilian'],
	['bg', 'bulgarian'], ['el', 'greek'],	['uk', 'ukrainian'], ['es-419', 'latam'],
	['vn', 'vietnamese'], ['default', 'english']
]);

const app = new Vue ({
	el: '#app',

	data: {
		search: '',

		nameSet00: '', nameSet01: '',

		fullCollection: []

	},

	mounted() {
		this.getCollection(urlSet00, this.nameSet00);
		this.getCollection(urlSet01, this.nameSet01);
	},

	methods: {
		getUserLang() {
			let key = '';

			if (!mapLangCodes.has(lang) && !mapLangCodes.has(lang.substring(0,2))) {
				key = 'default';
			}
			else if (lang.substring(0,3) === 'es-' && lang.substring(3,5) !== 'ES') {
				key = 'es-419';
			}
			else {
				if (lang === "zh-CN" || lang === "zh-TW" || lang === "pt-BR") {
					key = lang;
				}
				else {
					key = lang.substring(0,2);
				}
			}

			return mapLangCodes.get(key);
		},

		getCollection(url, nameSet) {
			let responseCards = [];
			axios.get(url).then(response => {
				nameSet = response.data.card_set.set_info.name[this.getUserLang()];
	
				responseCards = response.data.card_set.card_list;
				for (let i = 0; i < responseCards.length; i++) {
					// Getting rid of Ancients and Creeps from Base Set
					if (response.data.card_set.set_info.name.english === 'Base Set') {
						if (i < 9) { continue; }
					}
					
					// Getting rid of Abilities
					if (responseCards[i].card_type.includes('Ability')) { continue; }

					let color = '';
	
					if (responseCards[i].is_red) {
						color = 'Red';
					}
					else if (responseCards[i].is_blue) {
						color = 'Blue';
					}
					else if (responseCards[i].is_black) {
						color = 'Black';
					}
					else if (responseCards[i].is_green) {
						color = 'Green';
					}
					else {
						color = 'Orange';
					}
	
					this.fullCollection.push({
						set: nameSet,
						id: responseCards[i].card_id,
						type: responseCards[i].card_type,
						name: responseCards[i].card_name[this.getUserLang()],
						text: responseCards[i].card_text[this.getUserLang()],
						ingame_img: responseCards[i].ingame_image.default,
						rarity: responseCards[i].rarity,
						mini_img: responseCards[i].mini_image.default,
						large_img: responseCards[i].large_image[this.getUserLang()],
						color: color,
						attack: responseCards[i].attack,
						armor: responseCards[i].armor,
						hp: responseCards[i].hit_points,
						mana_cost: responseCards[i].mana_cost,
						subtype: responseCards[i].sub_type,
						gold: responseCards[i].gold_cost,
					});
				}
			})
		},
		
		onSearch() {

		}
	}
})


const footer = new Vue ({
	el: 'footer',

	methods: {
		currentYear: function() {
			return new Date().getFullYear();
		}
	}
});


/*let langCodes = {
	'en': 'english', 'de': 'german', 'fr': 'french', 'it': 'italian',
	'ko': 'koreana', 'es': 'spanish', 'zh-CN': 'schinese', 'zh-TW': 'tchinese',
	'ru': 'russian', 'th': 'thai', 'ja': 'japanese', 'pt': 'portuguese',
	'pl': 'polish', 'da': 'danish', 'nl': 'dutch', 'fi': 'finnish', 'no': 'norwegian',
	'sv': 'swedish', 'hu': 'hungarian', 'cs': 'czech', 'ro': 'romanian', 'tr': 'turkish',
	'pt-BR': 'brazilian', 'bg': 'bulgarian', 'el': 'greek', 'uk': 'ukrainian',
	'es-419': 'latam', 'vn': 'vietnamese', 'default': 'english'
};*/