const { invoke } = window.__TAURI__.tauri;

/*
let greetInputEl;
let greetMsgEl;
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

  const tbody = document.getElementsByTagName("tbody")[0];
  const rows = tbody.getElementsByTagName("tr");
  for (let i = 0; i < rows.length; i++) {
    rows[i].addEventListener("click", function() {
      // Remove "selected" class from all rows
      for (let j = 0; j < rows.length; j++) {
        rows[j].classList.remove("selected");
      }
      // Add "selected" class to clicked row
      this.classList.add("selected");
    });
  }
}
mygetpos();

/*
const selectedRow = document.querySelector("#AufPos tbody tr.selected");
if (selectedRow) {
  // Do something with the selected row
  console.log(selectedRow);
} else {
  console.log("No row is currently selected.");
}
*/