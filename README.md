# SFM++

SFM++ is an online timetable which (tries to) mimic the "official" timetable
shown at stations of SFM (Serveis Ferroviaris de Mallorca).

The current version is mostly feature-complete (though lacking in robustness).
Right now, you can self-host the server, but no public instance is available.
If you know of any (free) hosting, a comment would be highly appreciated.

![SFM++ screenshot, showing the timetable of Intermodal](./screenshot.png)

## Usage and dependencies

The server part, written in Python, depens on some libraries, which are listed
in `requirements.txt`. In addition, it requires a working installation of
Tesseract 4. A known working version is `4.1.1`.

After installing all the required dependencies, you can run the server by
running `python3 server.py`. As the OCR extractor assumes a certain folder
structure, you will need to create a `out/` folder first (`mkdir out`).

If you plan on hosting an instance of SFM++, keep in mind this project is
licensed under the [GNU Affero General Public License](https://choosealicense.com/licenses/agpl-3.0/),
which means that you must disclose the source of any instances you host. For a
more complete overview, visit the link above or read the [`LICENSE`](./LICENSE).
