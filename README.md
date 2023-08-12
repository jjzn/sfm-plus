# SFM++

SFM++ is an online timetable which (tries to) mimic the "official" timetable
shown at stations of SFM (Serveis Ferroviaris de Mallorca).

The current version is mostly feature-complete (though lacking in robustness).
Right now, you can self-host the server, but no public instance is available.
If you know of any (free) hosting, a comment would be highly appreciated.

![SFM++ screenshot, showing the timetable of Intermodal](./screenshot.png)

## Usage and dependencies

### TL;DR

Run these commands:

    # only if you haven't cloned already
    git clone https://github.com/jjzn/sfm-plus.git
    cd sfm-plus

    cargo run

### The long story

The server part, written in Rust, depends on some libraries, which are listed
in `Cargo.toml` and will be automatically installed by Cargo. In addition, it
requires a working installation of Tesseract 4 (known working version `4.1.1`)
and of OpenCV.

After having installed all the system dependencies, you can build and start the
server by running `cargo run`. Alternatively, you can directly invoke the build
executable, which will be located somewhere in `target/`.

If you plan on hosting an instance of SFM++, keep in mind this project is
licensed under the [GNU Affero General Public License](https://choosealicense.com/licenses/agpl-3.0/),
which means that you must disclose the source of any instances you host. For a
more complete overview, visit the link above or read the [`LICENSE`](./LICENSE).

## Testing

Test data, which includes images and transcriptions in JSON format, are
included in [`test/`](./test). The tests compare the output of the extractor
with a manual transcription of the corresponding image. They can be run with
the `test_runner.sh` Bash script.

## Roadmap and To-do

- [ ] Only re-render the table after data has been fetched from the server
- [ ] Reduce time spent on data extraction in the server
