const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let divOben;

/*
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
*/

async function mygetpos() {
  divOben.innerHTML = await invoke("get_position", {});
}

divOben = document.querySelector("#top-div");
mygetpos();

// Tabelle mit Positionen füllen
const table = document.querySelector('#AufPos tbody');
const data = [
  { id: 1, firstName: 'John', lastName: 'Doe', age: 20 },
  { id: 2, firstName: 'Jane', lastName: 'Doe', age: 19 },
  { id: 3, firstName: 'John', lastName: 'Smith', age: 21 },
  { id: 4, firstName: 'Sarah', lastName: 'Johnson', age: 22 },
];

let html = '';

data.forEach(item => {
  html += `
    <tr>
      <td>${item.id}</td>
      <td>${item.firstName}</td>
      <td>${item.lastName}</td>
      <td>${item.age}</td>
    </tr>
  `;
});

table.innerHTML = html;