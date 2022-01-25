// hey.js
export const get = async () => {
	const res = await fetch('http://127.0.0.1:8080/api/hey');
	const text = await res.text();
	return {
		body: text
	};
};
