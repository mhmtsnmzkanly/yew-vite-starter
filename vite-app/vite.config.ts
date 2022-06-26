import path from 'path';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

const dirLoc = {
  buildOutDir: path.resolve(__dirname, "../build/outdir"),
  cache: path.resolve(__dirname, "../build/cache"),
  pub: path.resolve(__dirname, "./src/public"),
  root: path.resolve(__dirname, "./src/root"),
};

export default defineConfig({
  base: "",
  root: dirLoc.root,
  cacheDir: dirLoc.cache,
  publicDir: dirLoc.pub,

  resolve: {
    alias: {
      "@": dirLoc.root,
    },
  },
  
  build: {
    target: 'esnext',
    minify: true,
	outDir: dirLoc.buildOutDir,
    assetsInlineLimit: 0,
    emptyOutDir: true,
  },
  
  plugins: [wasmPack(['../yew-app'])]
});