# SFM++

SFM++ is an online timetable which (tries to) mimic the "official" timetable shown at stations of SFM.

## Usage and dependencies

The server part, written in Python, depens on some libraries, which are listed
in `requirements.txt`. In addition, it requires a working installation of
Tesseract 4. A known working version is `4.1.1`.

After installing all the required dependencies, you can run the server by
running `python3 server.py`. As the OCR extractor assumes a certain folder
structure, you will need to create a `out/` folder first (`mkdir out`).
