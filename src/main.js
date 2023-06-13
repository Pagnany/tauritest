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
  const positionJson = await invoke("get_position", {});
  const positionObj = JSON.parse(positionJson);

  const table = document.querySelector('#AufPos tbody');
  let html = '';

  positionObj.forEach(item => {
    html += `
      <tr>
        <td>${item.position}</td>
        <td>${item.artikel}</td>
        <td>${item.menge}</td>
        <td>${item.preis.netto}</td>
      </tr>
    `;
  });

  table.innerHTML = html;
}

mygetpos();