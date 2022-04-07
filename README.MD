<div id="top"></div>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
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
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">Fitness Data Exporter</h3>

  <p align="center">
    For GSoC 22 and CCExtractor
    <br />
    <a href="https://github.com/sh-soltanpour/fitness-data-exporter"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/sh-soltanpour/fitness-data-exporter">View Demo</a>
    ·
    <a href="https://github.com/sh-soltanpour/fitness-data-exporter/issues">Report Bug</a>
    ·
    <a href="https://github.com/sh-soltanpour/fitness-data-exporter/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
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
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->

## About The Project

This project is done as my qualification task
for [CCExtractor program](https://ccextractor.org/public/gsoc/takehome/#export-data-from-myfitnesspal-to-grafana) for
Google Summer of Code 2022.

In this project, I have a CSV file data source that contains Fitbit application data (link
to [the dataset](https://ccextractor.org/public/gsoc/takehome/#export-data-from-myfitnesspal-to-grafana)). The rust code
responsibility is to parse this csv file, and write each record in a Postgres database. Then we can run grafana using
the given dockerfile, import the given dashboard into it and see the visualized data of Postgres.


<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

* [Rust](https://www.rust-lang.org/)
* [Postgres](https://www.postgresql.org/)
* [Grafana](https://grafana.com/)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally. To get a local copy up and
running follow these simple example steps.

### Prerequisites

You should have rust, cargo, diesel-cli and docker installed on your machine.

### Installation

1. Build and run Postgres Dockerfile:

```sh
   cd postgres
   docker build -t fitness-postgres .
   docker run -p 5432:5432 --name postgres fitness-postgres
   ```

2. Build Rust code:

    ```sh
    cargo build
    ```

3. Run migrations on postgres

    ```sh
    diesel migration run
    ```

4. Running the rust code:

    ```sh
    diesel migration run
    ```

5. Run rust code
    ```sh
    cargo run
    ```
6. Build Grafana Dockerfile
    ```sh
   cd grafana
   docker build -t fitness-grafana .
   docker run -d -p 3000:3000 --name grafana fitness-grafana
    ```
7. Add data source and import dashboard to grafana.

   The dashboard json file exists in grafana/dashboards directory. All you need to do is to add PostgresSQL data source
   to grafana and import the dashboard json file.

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- ROADMAP -->

## Roadmap

See the [open issues](https://github.com/sh-soltanpour/fitness-data-exporter/issues) for a full list of proposed
features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- CONTRIBUTING -->

## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also
simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->

## Contact

Shahryar Soltanpour - shahryar.soltanpour@gmail.com

Project
Link: [https://github.com/sh-soltanpour/fitness-data-exporter](https://github.com/sh-soltanpour/fitness-data-exporter)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/sh-soltanpour/fitness-data-exporter.svg?style=for-the-badge

[contributors-url]: https://github.com/sh-soltanpour/fitness-data-exporter/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/sh-soltanpour/fitness-data-exporter.svg?style=for-the-badge

[forks-url]: https://github.com/sh-soltanpour/fitness-data-exporter/network/members

[stars-shield]: https://img.shields.io/github/stars/sh-soltanpour/fitness-data-exporter.svg?style=for-the-badge

[stars-url]: https://github.com/sh-soltanpour/fitness-data-exporter/stargazers

[issues-shield]: https://img.shields.io/github/issues/sh-soltanpour/fitness-data-exporter.svg?style=for-the-badge

[issues-url]: https://github.com/sh-soltanpour/fitness-data-exporter/issues

[license-shield]: https://img.shields.io/github/license/sh-soltanpour/fitness-data-exporter.svg?style=for-the-badge

[license-url]: https://github.com/sh-soltanpour/fitness-data-exporter/blob/master/LICENSE.txt

[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555

[linkedin-url]: https://linkedin.com/in/soltanpour

[product-screenshot]: images/screenshot.png