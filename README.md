# Yew Site

## Hosted on Netlify

<https://1k2s.netlify.app>

[![Netlify Status](https://api.netlify.com/api/v1/badges/09fe8d27-526e-4c93-a05e-918da6455e77/deploy-status)](https://app.netlify.com/sites/1k2s/deploys)

<!-- markdownlint-disable MD033 -->
<p>
  <img height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/Rust.svg"
    alt="Rust"
  />
  <img
    height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/Yew-Light.svg"
    alt="Yew"
  />
  <img
    height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/Sass.svg"
    alt="Syntactically Awesome Style Sheets (Sass)"
  />
  <img
    height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/Python-Dark.svg"
    alt="Python"
  />
  <img height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/Bash-Dark.svg"
    alt="Shell"
  />
  <img
    height="50px"
    src="https://codeberg.org/1Kill2Steal/skill-icons/raw/branch/main/icons/HTML.svg"
    alt="HTML"
  />
</p>
<!-- markdownlint-enable MD033 -->

## Short Description & Instructions

It uses `wasm-bindgen` with the Yew framework and the yew router to serve the
website. The website itself is straight-forward as it features the following
content:

- Sticky navigation bar
- Fixed at the bottom Footer
- 3 pages: Homepage, About Me, Gallery
- 2 page categories: Projects and Blogs

The Gallery images are locally saved and the instructions to set the gallery images are the following:

- You upload the images in `/hutao/pics` and `/hutao/pics_uncompressed/` (You can change that directory name but you need to bother with changing up the paths in some files - namely the json utility script and the `/index.html` file)
- In the image folders you need to strictly have your image names either start with the number followed by underscore or end in underscore followed by the number followed by the extension (as that's how the regex is implemented in `/utils/x1_file_and_json_utils/`).

Tips: If you use Thunar as your file manager there's a very intuitive way to
bulk rename your files. If not then, I'm sure there's other utilities to get
that job done.

The site is fully responsive and the gallery has a similar structure to the way
Pinterest images are displayed (just a lot more simple).

Additional info:

**Artist credits**

There's a python script (under `/py-utils/`) for calling the API of a site
called [SauceNAO](https://saucenao.com/) which is used to look up image sources
on the internet. It's used to configure the `/hutao/json/artist_credits.json`
file which serves as a way to find the Artists information.

> [!IMPORTANT]
> If you're an artist of any of the artworks on this site and you don't want
> them listed in it then by all means list it as an issue or contact me via any
> of my social medias (discord at: `1Kill2Steal#5316` for example).

### Setting up

[Rust w/ rustup](https://www.rust-lang.org/tools/install) (rust-lang.org)
From here on you have 2 routes (using the build.sh script if you're on Linux or
running commands manually):

Running the script:

```sh
chmod +x ./build.sh # <- That's one time only.
./build.sh
```

Manual way:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

cargo make test_1
cargo make 1
```

> [!NOTE]
> There may be more cargo make tests, in the case that there's more and this
> wasn't updated, feel free to PR it adding another command for the test or
> just open an issue.

Then you can build your application.

For development:

```bash
trunk serve
```

For production:

```bash
trunk build --release
trunk serve --release
```
