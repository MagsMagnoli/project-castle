export function POST() {
	console.log('POST /server');
	return {
		status: 200,
		body: 'Hello, world!'
	};
}
