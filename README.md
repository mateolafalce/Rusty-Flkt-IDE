<div align="center">

  ![rusty](src/rusty.ico)

  ---

<h2>⚠️Still in development⚠️</h2>

![ide](src/ide.png)

  <h1>🪂Rusty IDE🪂</h1>
  <p>A lightweight IDE based on rust</p>
</div>

<br>

🪂Rusty is a lightweight and fast IDE developed in Rust, focused on Rust and web3 technology development. With its simple and elegant graphical interface‍, you can enhance productivity in your projects and enjoy a smooth coding experience. Designed with performance in mind, Rusty provides a fast and efficient code editing experience, allowing developers to focus on creating innovative solutions in Rust and web3 technology. If you're looking for an intuitive and user-friendly tool for your Rust programming projects, give Rusty a try today!

---

<details>
<summary>Add a project into the IDE 💼</summary>

<br>

<div align="center">

![add_project_button](readme/add_project_button.gif)

</div>

<br>

The Rusty fltk IDE is an integrated development environment specifically designed for the Rust programming language. It utilizes the FLTK library for the graphical interface. It provides a range of features and functionalities to facilitate project development and repository management.

The provided code shows a function called [btn_add_folder](https://github.com/mateolafalce/Rusty-Fltk-IDE/blob/main/src/functions/folders_functions/btn_add_folder.rs) btn_add_folder that creates a button in the IDE's interface. Clicking on this button will open a native dialog box to select a folder representing a project or repository.

Once a folder is selected, a series of actions are performed. The selected folder path is obtained and checked for validity. If a valid path is provided, a label is displayed in the options window indicating that it is loading.

Next, the [set_folders_roots](https://github.com/mateolafalce/Rusty-Fltk-IDE/blob/main/src/functions/root/set_folders_roots.rs) function is called to set the root folder paths for the project. If successful, the [render_folder](https://github.com/mateolafalce/Rusty-Fltk-IDE/blob/main/src/functions/folders_functions/render_folder.rs) function is invoked to render the project's folder structure as a visual tree within the IDE.

In case any errors occur during the process, an alert with the corresponding error message is displayed.

Additionally, the button has an event handling mechanism that changes the cursor when the mouse enters or exits the button's area.

</details>

---

<details>
<summary>Add a project by drag🤏 and drop 🙌</summary>

<br>

<div align="center">

![drag_and_drop](readme/drag_and_drop.gif)

</div>

<br>

</details>

---

<details>
<summary>Apache2.0 📜</summary>

<br>

This project is licensed under the Apache License, Version 2.0 (the "License"). You may not use this file except in compliance with the License. You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

<div align="center">

![license](readme/license.png)

</div>

</details>
