export {};

declare global {
  interface Window {
    kernel?: {
      getStatus: () => Promise<string>;
    };
  }
}
