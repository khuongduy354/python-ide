const { invoke } = window.__TAURI__.tauri;

// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document.querySelector("#greet-form").addEventListener("submit", (e) => {
//     e.preventDefault();
//     greet();
//   });
// });
const button = document.getElementById("runButton");

const inp = document.getElementById("input");
async function runCode() {
  const out = document.getElementById("output");
  const res = await invoke("run_code", { source: inp.value });

  console.log(res);
  if (res.exit_code == 0) {
    out.textContent = res.stdout;
  } else {
    out.textContent = res.stderr;
  }
  out.textContent = out.textContent + `\nExit code: ` + res.exit_code;
}

button.addEventListener("click", runCode);

// allow tab in indent area
inp.addEventListener("keydown", function (e) {
  if (e.key == "Tab") {
    // console.log("tabbed");
    e.preventDefault();
    var start = this.selectionStart;
    var end = this.selectionEnd;

    // set textarea value to: text before caret + tab + text after caret
    this.value =
      this.value.substring(0, start) + "\t" + this.value.substring(end);

    // put caret at right position again
    this.selectionStart = this.selectionEnd = start + 1;
  }
});
