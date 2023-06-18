const { invoke } = window.__TAURI__.tauri;

const btnMenuAuftrag = document.querySelector('#btnMenuAuftrag');
btnMenuAuftrag.addEventListener('click', () => {
  window.location.href = 'auftrag.html';
});

const btnMenuArtikel = document.querySelector('#btnMenuArtikel');
btnMenuArtikel.addEventListener('click', () => {
  window.location.href = 'artikel.html';
});

const btnMenuDashboard = document.querySelector('#btnMenuDashoard');
btnMenuDashboard.addEventListener('click', () => {
  window.location.href = 'index.html';
});