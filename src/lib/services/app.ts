import { PUBLIC_API_URL } from '$env/static/public';

export const checkOnlineStatus = async () => {
	try {
		const response = await fetch(PUBLIC_API_URL);
		if (response.status === 200) {
			return true;
		} else {
			return false;
		}
	} catch (error) {
		return false;
	}
};
