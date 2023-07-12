hmark - for creating hashed bookmarks (still working on it)

  ex. usage:

    $ hmark set -k gitlab_home -u https://gitlab.com
    $ hmark set -k hmark       -u https://github.com/rayhan-wijaya/hmark

    $ hmark view gitlab_home
    https://gitlab.com

    $ hmark list
    hmark
    gitlab_home

    bookmarks you create will be stored in the ~/.hmark/bookmarks folder, where
    tilde, is of course, your home directory.

  ex. bookmarks folder:

    * ~/.hmark/bookmarks/
    |  ffmpeg_docs  -> https://ffmpeg.org/ffmpeg.html/
    |  diesel_docs  -> https://docs.rs/diesel/latest/diesel/
    |  a_cool_tweet -> https://twitter.com/Neovim/status/1671419524854210560/
