const { colors } = require('../res/colors.cjs');

const typography = {
	'.heading': {
		fontFamily: 'Segma-Regular',
		fontWeight: '600',
		fontSize: '16px'
	}
};

const shadows = {
	'.glow': {
		'-webkit-box-shadow': '0px 0px 50px 5px rgba(252,105,254,0.48)',
		'-moz-box-shadow': ' 0px 0px 50px 5px rgba(252,105,254,0.48)',
		'box-shadow': '0px 0px 50px 5px rgba(252,105,254,0.48)'
	},
	'.glow_dark': {
		'-webkit-box-shadow': '0px 0px 50px 5px rgb(91 92 120)',
		'-moz-box-shadow': ' 0px 0px 50px 5px rgb(91 92 120)',
		'box-shadow': '0px 0px 50px 5px rgb(91 92 120)'
	},
	'.svg-glow': {
		filter: 'drop-shadow(0px 0px 15px rgb(252,105,254))'
	},
	'.svg-glow_dark': {
		filter: 'drop-shadow(0px 0px 15px rgb(91 92 120))'
	}
};

const styles = { ...typography, ...shadows };

module.exports = { styles };
