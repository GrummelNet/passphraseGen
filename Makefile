SOURCE := passphrase.rs
OUT := passphrase

all:
	rustc -A non-snake-case $(SOURCE)

clean:
	rm -rf passphrase
