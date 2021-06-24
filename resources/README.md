# The Resources Folder
This is the default resources folder that will be used to store downloded files and your database when you run with ```cargo run --release```.

You can override this with the environment variable `IBAN_BEAVER_RESOURCES`.
I strongly recommend using this environment variable when executing the binary directly. If you do not, it will look in your current working directory for a resources folder. If there is none, the program won't work.
