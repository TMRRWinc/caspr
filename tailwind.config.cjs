const plugin = require('tailwindcss/plugin');
const { colors } = require('./src/lib/res/colors.cjs');
const { styles } = require('./src/lib/styles/commonStyles.cjs');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,ts,svelte}'],
	theme: {
		colors: colors,
		extend: {
			fontFamily: {
				mabry: ["'Mabry Pro'"],
				segma: ["'Segma-Regular'"],
				favorit: ["'Favorit Pro'"]
			}
		}
	},
	plugins: [
		plugin(function ({ addComponents }) {
			addComponents(styles);
		})
	],
	darkMode: 'class'
};
