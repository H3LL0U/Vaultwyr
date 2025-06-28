# Vaultwyr

## What's Vaultwyr?

Vaultwyr is a simple tool that lets you encrypt and decrypt your files using a password of your choice. It's handy for locking down documents, notes, or anything else you'd prefer to keep private.

## Built with Tauri

Vaultwyr is made with Tauri, which helps keep it lightweight and fast. The frontend is built using React and TypeScript, while the backend uses Rust for security and performance.

## What are the features

You can use Vaultwyr to encrypt single files or entire folders. It uses password-based encryption with AES behind the scenes, so your data is safe as long as your password is.

You can also adjust the maximum file deletion size (after encryption), 
encryption chunk size (the bigger the faster, but it will cause more memory usage overhead) 
and if you want the decrypted files be decrypted back into their original location

## How It Works

Open the app, pick a file or folder, select the "Encrypt" option, type in your password, and you're good to go. That’s it. To unlock it later, just do the same thing again but choose "Decrypt" instead. It’s designed to be straightforward and beginner-friendly.

## Getting Started

You can download the latest build or clone the project from GitHub and build it yourself.

## Showcases

### Encryption
![Image](https://github.com/user-attachments/assets/bb305f99-2229-4e35-bc3e-072a82e4c36f)

### Decryption
![Image](https://github.com/user-attachments/assets/0d6ee08a-114d-4226-bb0d-9e2a36570a32)

