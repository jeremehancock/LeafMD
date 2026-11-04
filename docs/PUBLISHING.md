# Publishing LeafMD

How releases and [leafmd.app](https://leafmd.app/) are published, the one-time setup they need, and how to try them on your own computer first. For working on the code, see [DEVELOPMENT.md](DEVELOPMENT.md).

## What a release publishes

A release happens when a merge into `main` carries a finished `<release>` in `data/*.metainfo.xml` whose version hasn't been released yet (see [Branches and releases](DEVELOPMENT.md#branches-and-releases)). GitHub's build then:

- signs the build with LeafMD's key
- creates the `v<version>` tag and a [GitHub Release](https://github.com/jeremehancock/LeafMD/releases) with:
  - `LeafMD-<version>.flatpak`, which installs LeafMD and then updates from leafmd.app
  - `leafmd-repository-<version>.tar.gz`, the signed repository exactly as leafmd.app serves it, kept so the site can be republished later without building the app again
- publishes leafmd.app on GitHub Pages, built by `build-aux/build-site.py`:
  - the storefront, from `site/`, with the screenshots from `data/screenshots/` and the version from the newest release in the metainfo file, linked to its release notes on GitHub
  - `LeafMD.flatpakref` and `leafmd.flatpakrepo`, the files that tell Flatpak where to install LeafMD from and which key it's signed with
  - `repo/`, the signed Flatpak repository installed copies get their updates from. It only ever holds the newest version.

## Republishing leafmd.app

Changes to the storefront (`site/`) go live with the next release, or straight away by republishing. Merge them into `main`, then either:

- on GitHub, go to **Actions**, then **Build**, then **Run workflow**, choose `main`, tick **Republish leafmd.app** and click **Run workflow**, or
- run:

  ```
  gh workflow run ci.yml --ref main -f republish=true
  ```

A republish builds nothing. It takes the newest release's `leafmd-repository-<version>.tar.gz` and serves it again unchanged, with the site's text and design from `main`, and the version, app icon and screenshots from that release's tag. So installed copies see no update, and nothing unreleased is ever published, even if `main` already has changes for the next version. It only runs from `main`.

## Publishing setup

Done once, before the first release. You need `gpg` and the GitHub command line (`gh`).

1. **Make LeafMD's signing key.** It signs every release, so Flatpak only accepts updates that really came from you. It has no passphrase (GitHub has to use it without anyone typing) and never expires (an expired key would stop updates for everyone). Make it in a fresh, empty keyring so it stays separate from any personal keys:

   ```
   export GNUPGHOME=$(mktemp -d)
   gpg --batch --passphrase '' --quick-gen-key "LeafMD release signing key" rsa4096 sign never
   gpg --export > build-aux/leafmd-signing-key.gpg
   gpg --armor --export-secret-keys | gh secret set LEAFMD_GPG_PRIVATE_KEY
   gpg --armor --export-secret-keys > ~/leafmd-signing-key-PRIVATE.asc
   rm -rf "$GNUPGHOME"; unset GNUPGHOME
   ```

   - `build-aux/leafmd-signing-key.gpg` is the public half. Commit it.
   - The secret `LEAFMD_GPG_PRIVATE_KEY` is the private half, for GitHub's build.
   - `~/leafmd-signing-key-PRIVATE.asc` is your backup. **Put it in your password manager, then delete the file.** If the key is ever lost, no more updates can be signed, and everyone would have to install LeafMD again.

2. **Turn on GitHub Pages:** the repository's Settings, Pages, then under "Build and deployment" set Source to **GitHub Actions**.

3. **Point leafmd.app at GitHub.** At the domain provider, add:

   | Type | Name | Value |
   |---|---|---|
   | A | `@` | `185.199.108.153`, `185.199.109.153`, `185.199.110.153`, `185.199.111.153` (four records) |
   | AAAA | `@` | `2606:50c0:8000::153`, `2606:50c0:8001::153`, `2606:50c0:8002::153`, `2606:50c0:8003::153` (four records) |
   | CNAME | `www` | `jeremehancock.github.io` |

   Then:
   - In your GitHub account's Settings, Pages, **add `leafmd.app` as a verified domain**. GitHub shows one more TXT record to add. This stops any other GitHub account from using the domain.
   - In the repository's Settings, Pages, set **Custom domain** to `leafmd.app`.
   - Once GitHub says the certificate is ready (minutes, up to a day), tick **Enforce HTTPS**. Every `.app` address needs HTTPS, so the site won't open at all until then.

4. **Keep `leafmd.app` on auto-renew** at the domain provider. Installed copies check it for updates, so if it lapsed, updates would stop.

To check: `dig +short leafmd.app` lists GitHub's four addresses, and `gh api repos/jeremehancock/LeafMD/pages` shows `"build_type": "workflow"`, `"cname": "leafmd.app"` and `"https_enforced": true`.

## Previewing leafmd.app locally

To try leafmd.app itself before publishing (the storefront, the Install button and updates), build a private copy that's served from this computer:

```
./build-aux/flatpak-repo-preview.sh
```

It builds a signed repository with a throwaway key, builds the site for `http://localhost:8642/`, and serves it until you press Ctrl+C. Run it again after a change and the installed preview copy is offered the update. The preview installs as LeafMD's `stable` branch, next to your own build (`master`), and the script prints how to remove it.

For a quick look at the page alone, without building the app, build just the site and serve it:

```
build-aux/build-site.py .flatpak-builder/site-preview --base-url http://localhost:8642/
python3 -m http.server 8642 --bind 127.0.0.1 --directory .flatpak-builder/site-preview
```

The Install buttons don't work in that copy, since it has no repository.
