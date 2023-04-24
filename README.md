# Mayhem

![Lines of code](https://img.shields.io/tokei/lines/github/RedstoneWizard08/mayhem?label=lines%20of%20code&style=for-the-badge)
![License](https://img.shields.io/github/license/RedstoneWizard08/mayhem?style=for-the-badge)
![Client Version](https://img.shields.io/github/package-json/v/RedstoneWizard08/mayhem?label=Client&style=for-the-badge)
[![Server Version](https://img.shields.io/crates/v/mayhem?style=for-the-badge)](https://crates.io/crates/mayhem)
![Dependencies](https://img.shields.io/librariesio/github/RedstoneWizard08/mayhem?style=for-the-badge)

An open-source Discord clone. Optimized for security, stability, safety, and customization.

## The Stack

The app is built with:

-   Backend

    -   Rust
    -   Axum
    -   Tokio
    -   SeaORM
    -   Serde

-   Frontend:
    -   TypeScript
    -   Vite
    -   SvelteKit
    -   SASS

## Features

-   [x] Secure token-based auth.
-   [ ] 2FA support.
-   [ ] Session-based token expiry.
-   [ ] Multiplatform
-   [ ] Themeable
-   [ ] Customizable (plugins)
-   [ ] Bots
-   [ ] Public and private servers

## For Schools

-   [ ] District-level restriction (joining servers, adding friends, content filter, etc.)
-   [ ] School-level restriction (see above for some rules)
-   [ ] Person-level restriction

## The Goal

The Goal of Mayhem is to be a viable and fast Discord alternative, that is secure from the ground up and completely open-source, but also to be a good educational tool, as a lot of school districts (like my own) lack the ability to chat effectively with team members during group projects. The goal of this is to allow people to do whatever they want at home, but at school, restrictions can be created by schools to allow this to be a good system for them.

## Restrictions for Schools

-   [ ] Which servers people can join
-   [ ] Who can be invited to private servers
-   [ ] Content filter settings
-   [ ] Voice and video call limitations
-   [ ] Time limitations
-   [ ] And a few more.

Note that these restrictions will only apply when using a district account.

You will not be able to log in with personal accounts or register for a new account when on a chromebook and have a matching IP with a district-specified subnet.
