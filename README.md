<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![AGPL License][license-shield]][license-url]



<br />
<p align="center">
  <a href="https://github.com/rmsthebest/iban_beaver">
    <img src="images/ibanbeaver.jpg" alt="Logo" width="368" height="368">
  </a>

  <h3 align="center">IBAN Beaver</h3>

  <p align="center">
  IBAN verification for cheapskates
    <br />
<!-- PROJECT LOGO 
    <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a>
    ·
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Report Bug</a>
    ·
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Request Feature</a>
-->
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
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
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

IBAN Beaver is a webapp that verifies an IBAN upon request. The response is a json object with information about the associated bank (if any). It is free to use, but keep in mind that this is just a project to learn about webby things and databases. Don't sue me if you use this for billing and something goes wrong.

### Supported Countries
* Germany
* Austria
* Netherlands

### Built With

* [Diesel](https://diesel.rs)
* [Warp](https://github.com/seanmonstar/warp)

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites
Only tested on GNU Linux.

* rust / cargo
* base-devel / build-essential / your distros dev meta-package
* diesel_cli, needed for diesel migrations. See their doc.
  ```sh
   cargo install diesel_cli # generic
   sudo pacman -S diesel-cli # arch
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/rmsthebest/iban_beaver.git
   ```
2. Run!
   ```sh
   diesel setup # creates db
   diesel migration run # create the tables. can use redo to drop table first
   cargo run --release
   ```



<!-- USAGE EXAMPLES -->
## Usage

# Server Side
See step 2 in [Installation](#Installation)
By default a resources directory is expected to exist from where you are running iban_beaver.
If you install it somewhere else like /usr/local/bin you may want to run it like this:
```sh
mkdir -p ~/.local/share/iban_beaver/resources
mv /path/to/db.sqlite ~/.local/share/iban_beaver/resources
env IBAN_BEAVER_RESOURCES=~/.local/share/iban_beaver/resources iban_beaver
```

# Client/User
Verify IBAN
```sh
curl 127.0.0.1:3030/iban/<iban>
```
Update database
```sh
curl 127.0.0.1:3030/db/update/<country>
```
Fill database without downloading data (can be used to reset things like blacklists)
```sh
curl 127.0.0.1:3030/db/fill/<country>
```
Blacklist IBAN
```sh
curl 127.0.0.1:3030/db/blacklist/<iban>/<add or remove>
```

<!-- ROADMAP -->
## Roadmap

I'm not that ambitious. If you assume current status is all you're going to get you are most likely right.
If someone [opens an issue](https://github.com/rmsthebest/iban_beaver/issues) I might work on that.



<!-- CONTRIBUTING -->
## Contributing

This is a best effort project, I might move on with my life and never see your contribution.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

# Steps to add support for a country

1. Create `src/country/<countrycode>.rs`
2. Satisfy the country trait (copy a similar country and fix what needs to be fixed)
3. Add country to match statement in `src/country/mod.rs`
4. Test the update/fill/iban commands. Valid ibans for testing can be found [here](https://wise.com/gb/iban/example)

<!-- Acknowledgements -->
## Acknowledgements
* [Readme Template](https://github.com/othneildrew/Best-README-Template)

<!-- LICENSE -->
## License

Distributed under the AGPL-v3 License. See `LICENSE` for more information.


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/rmsthebest/iban_beaver.svg
[contributors-url]: https://github.com/rmsthebest/iban_beaver/graphs/contributors
[issues-shield]: https://img.shields.io/github/issues/rmsthebest/iban_beaver.svg
[issues-url]: https://github.com/rmsthebest/iban_beaver/issues
[license-shield]: https://img.shields.io/github/license/rmsthebest/iban_beaver.svg
[license-url]: https://github.com/rmsthebest/iban_beaver/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
