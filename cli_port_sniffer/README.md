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
  <a href="https://github.com/Maofryy/RustProjects/tree/main/cli_port_sniffer">
    <img src="images/rustacean-flat-happy.png" alt="Logo" width="110" height="80">
  </a>

  <h3 align="center">TCP port sniffer</h3>

  <p align="center">
    Implementation of a tcp port sniffer binary, interacting with cli.
    <br />
    <a href="https://github.com/Maofryy/RustProjects/tree/main/cli_port_sniffer"><strong>Explore the docs »</strong></a>
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
* [csv](https://crates.io/crates/csv)


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
      cd repo/cli_port_sniffer
      cargo run
    ```

<!-- USAGE EXAMPLES -->
## Usage

This is a TaskManager: able to store actions or things to do, mark them as done, un check them and delete them.
- System usage : 
    ```sh
        Usage:
          ./cli_port_sniffer -h                           : Print this help message
          ./cli_port_sniffer <ip_addr>                    : Sniff all ports of target <ip_addr>
          ./cli_port_sniffer -j <thread_number> <ip_addr> : Sniff all ports of <ip_addr> using <thread_number> threads.
    ``` 
-  ```./cli_port_sniffer -j 1000 <ip_addr>``` command :
    ```sh
      Welcome to your CLI Port sniffer.
      ........!!!!!!!!!!!!!!!!!
      Opened ports on <id_addr>:
      PORT     STATE  SERVICE
      2000/tcp closed Remotely Anywhere / VIA NET.WORKS PostOffice Plus
      2001/tcp closed Cisco mgmt / Remotely Anywhere
      2002/tcp closed globe
      2003/tcp closed GNU finger
      2004/tcp closed mailbox
      2005/tcp closed encrypted symmetric telnet/login
      2006/tcp closed invokator
      2007/tcp closed dectalk
      2008/tcp closed conf
      2009/tcp closed news
      2010/tcp open   search
      2011/tcp closed raid
      2012/tcp closed ttyinfo
      2013/tcp open   raid-am
      2014/tcp open   troff
      2015/tcp open   cypress
      2016/tcp closed bootserver
      2017/tcp open   cypress-stat
      2018/tcp open   terminaldb
      2019/tcp open   whosockami
      2020/tcp open   xinupageserver
      2021/tcp closed servexec
    ```
    > Running with large number of threads is recommended: If numerous ports are timing out it may take a while to complete.
    "." indicates open port found.
    "!" indicates port from which the connection was refused
    "" *indicates* unreachable or timing out ports

    There is then a table sorted by port number displaying status and description of the ports scanned.
<!-- _For more examples, please refer to the [Documentation](https://example.com)_



<!-- ROADMAP -->
## Roadmap
### Features to implement
  - ### Better sorting implementation : Would want sorted by status open in first then by port number
  - ### Finish scan() tests

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
