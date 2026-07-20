import {build, createLogger, defineConfig} from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { extname, join, basename, resolve } from 'path';
import { readdirSync, statSync, } from 'fs';

let logger = createLogger('info', { prefix: '[tidos]' })

function getEntries(dir, extension = '.svelte') {
    const entries = {};

    function walk(currentDir) {
        const files = readdirSync(currentDir);

        files.forEach((file) => {
            const fullPath = join(currentDir, file);
            const stat = statSync(fullPath);

            if (stat.isDirectory()) {
                // Recurse into subdirectories
                walk(fullPath);
            } else if (extname(file) === extension) {
                // Collect files with the specified extension
                const name = basename(file, extension);
                entries[name] = resolve(__dirname, fullPath);
            }
        });
    }

    walk(dir);
    return entries;
}

const entries = getEntries('src');

function tidosSvelteHMR() {

    let shouldDebounce = false
    const hmrBuild = async () => {
        shouldDebounce = true
        await build({ logLevel: "silent" })
    };

    return {
        name: 'tidos-svelte-hmr',
        enforce: "pre",
        // HMR
        handleHotUpdate: ({ file, server }) => {
            if (!shouldDebounce) {
                logger.info(`Changes detected, building new version...`, { timestamp: true })
                hmrBuild()
                    .then(() => {
                        shouldDebounce = false
                        logger.info(`Build completed.`, { timestamp: true })
                    })
            }
            return []
        }
    }
}

export default defineConfig({
    plugins: [
        svelte({
            compilerOptions: {
                customElement: true,
            },
        }),
        tidosSvelteHMR(),
    ],
    build: {
        rollupOptions: {
            input: entries,
            output: {
                entryFileNames: '[name].js',
                chunkFileNames: '[name].js',
                dir: 'dist', // Output directory for the compiled files
                assetFileNames: '[name][extname]',
            },
        },
    },
});
