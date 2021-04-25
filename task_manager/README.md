<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
-->
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/Maofryy/RustProjects/tree/main/task_manager">
    <img src="images/rustacean-flat-happy.png" alt="Logo" width="110" height="80">
  </a>

  <h3 align="center">Rust Task Manager</h3>

  <p align="center">
    Task Manager implementation using Rust, focused on learning the lang.
    <br />
    <a href="https://github.com/Maofryy/RustProjects/tree/main/task_manager"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    ·
    <a href="https://github.com/maofryy/RustProjects/issues">Report Bug</a>
    ·
    <a href="https://github.com/maofryy/RustProjects/issues">Request Feature</a>
    .
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- 
Here's a blank template to get started:
**To avoid retyping too much info. Do a search and replace with your text editor for the following:**
`maofryy`, `todos-group-1/tree/branche_maori`, `twitter_handle`, `maori.benhassine@gmail.com`, `Rust TaskManager`, `Task Manager implementation using Rust, focused on learning the lang.`
-->

### Built With

* [Rust Programming Language](https://www.rust-lang.org/)
* [Cargo](https://doc.rust-lang.org/cargo/guide/)
* [Rusqlite](https://github.com/rusqlite/rusqlite)


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.
- ### Prerequisites
   #### Install Rust
  Follow [Rust install](https://www.rust-lang.org/tools/install) documentation
- ### Installation
  - ### Clone repo
    ```sh
    git clone https://github.com/Maofryy/RustProjects.git
    ```

  - ### Build and run project
    ```sh
      cd repo/task_manager
      cargo run
    ```

<!-- USAGE EXAMPLES -->
## Usage

This is a TaskManager: able to store actions or things to do, mark them as done, un check them and delete them.
- System usage : 
    ```sh
        Usage :
        + <label>   : Create new task with <label>
        - <id>      : Remove task from list by <id>
        x <id>      : Check task by <id>
        c <id>      : Uncheck task by <id>
        clear       : Erase all tasks
        quit        : Exit task manager
    ``` 
- Add a task with the ```+ <label>``` command :
    ```sh
    >>> + task
    0 [ ] task
    ```
    The system will provide an id and store the task
- Delete a task by id ```- <id> ```
    ```sh
    >>> - 0
    Deleted task 0
    ```
- Check a task : ```x <id>```
    ```bash
    0 [ ] task
    >>> x 0
    Checked task 0
    0 [x] task
    ```
- Un-check a task : ```c <id>```
    ```sh
    0 [x] task
    >>> c 0
    Unchecked task 0
    0 [ ] task
    ```
- Cleari the tasks list : ```clear```
    ```sh
    >>> clear
    Tasks cleared.
    ```
- Exit from the manager: ```quit```
    ```sh
    >>> quit
    Terminating program.
    ```
<!-- _For more examples, please refer to the [Documentation](https://example.com)_



<!-- ROADMAP -->
## Roadmap
### Features to implement
  - ### Persistence : Loading and Writing tasks in database
  - ### More attributes to each tasks
  - ### Sorting by last task updated

See the [open issues](https://github.com/maofryy/RustProject/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Maori Benhassine - maori.benhassine@gmail.com \
Project Link: [https://github.com/Maofryy/RustProjects](https://github.com/Maofryy/RustProjects)


<!--
 ACKNOWLEDGEMENTS 
## Acknowledgements

* []()
* []()
* []()





<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/maofryy/repo.svg?style=for-the-badge
[contributors-url]: https://github.com/maofryy/repo/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/maofryy/repo.svg?style=for-the-badge
[forks-url]: https://github.com/maofryy/repo/network/members
[stars-shield]: https://img.shields.io/github/stars/maofryy/repo.svg?style=for-the-badge
[stars-url]: https://github.com/maofryy/repo/stargazers
[issues-shield]: https://img.shields.io/github/issues/maofryy/repo.svg?style=for-the-badge
[issues-url]: https://github.com/maofryy/repo/issues
[license-shield]: https://img.shields.io/github/license/maofryy/repo.svg?style=for-the-badge
[license-url]: https://github.com/maofryy/repo/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/maori-benhassine/
