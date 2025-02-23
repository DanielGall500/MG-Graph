/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_NEO_PW: string; 
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}