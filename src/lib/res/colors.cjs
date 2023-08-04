// Main Colors
const brandColors = {
	'brand-01': '#1D1530',
	'brand-accent-01': '#5C3973',
	'brand-accent-02': '#5D576B'
};

const darkColors = {
	bg_dark: '#1b1b1b',
	border_dark: '#a6a6a6',
	accent_dark: '#fafafa',

	'text-low-contrast_dark': '#dbdbdb',
	'text-medium-contrast_dark': '#f0f0f0',
	'text-high-contrast_dark': '#fafafa',

	success_dark: '#5c6a1f',
	warning_dark: '#a08124',
	error_dark: '#F20202',
	info_dark: '#36566c'
};

const lightColors = {
	bg_light: '#fafafa',
	border_light: '#555555',
	accent_light: '#1b1b1b',

	'text-low-contrast_light': '#727272',
	'text-medium-contrast_light': '#111111',
	'text-high-contrast_light': '#1b1b1b',

	success_light: '#a7b41f',
	warning_light: '#ffbf1a',
	error_light: '#d10000',
	info_light: '#4795cc'
};

// Other Colors
const otherColors = {
	white: '#ffffff',
	black: '#000000',
	transparent: 'transparent',
	'bg-code_dark': '#22272e',
	'bg-code_light': '#ffffff'
};

// All Colors
const colors = {
	...brandColors,
	...darkColors,
	...lightColors,
	...otherColors
};

module.exports = { colors, brandColors, darkColors, lightColors, otherColors };
