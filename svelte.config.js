/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// hydrate the <div id="svelte"> element in client/app.html
		target: 'body',
		files: {
			hooks: 'client/hooks',
			lib: 'client/lib',
			routes: 'client/routes',
			serviceWorker: 'client/service-worker',
			template: 'client/app.html'
		},
	}
};

export default config;
