const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
    invoke("on_start");
});
