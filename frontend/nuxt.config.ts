// https://v3.nuxtjs.org/api/configuration/nuxt.config

if (!process.env.API_URL) {
    throw new Error("API URL not specified");
}

export default defineNuxtConfig({
    alias: {
        "&": "./ts-modules"
    },
    runtimeConfig: {
        public: {
            apiURL: process.env.API_URL
        }
    }
});
