---
sidebar_position: 1
description: A beginner's guide to using Git and GitHub for version control.
---

# Git & GitHub

Here, we will cover the essential steps to install Git, set up your GitHub account, and manage your first project. Git is a *Version Control System* for tracking changes in your code, and GitHub is a cloud platform for hosting and collaborating on your Git projects.

import Tabs from '@theme/Tabs'; 
import TabItem from '@theme/TabItem';

## Resources

1. [Pro Git book](https://git-scm.com/book/en/v2)
2. [Git documentation](https://git-scm.com/docs)
3. [Github documentation](https://docs.github.com/en)

## Prerequisites

You'll need two things to get started: the Git software on your computer and a GitHub account.

### Install Git

Follow the steps for your operating system to install the Git command-line tools.

<Tabs> 
    <TabItem value="linux" label="Linux (Debian/Ubuntu)" default> 
        You can install Git using your distribution's package manager.
        ```shell
        sudo apt-get update
        sudo apt-get install git
        ```
    </TabItem>

    <TabItem value="windows" label="Windows"> 
        Download and run the official installer from [official website](https://git-scm.com/install/windows)

        :::info
        The default options are fine for most users. This installation includes **Git Bash**, a terminal application that you should use for running all the `git` commands in this tutorial.
        :::
    </TabItem>

    <TabItem value="macos" label="macOS"> 
        The easiest way to install Git is to install the Xcode Command Line Tools.
        ```shell
            xcode-select --install
        ```
        Alternatively, if you use [Homebrew](https://brew.sh/):
        ```shell
        brew install git
        ```
    </TabItem>

</Tabs>

After installing, open your terminal (or Git Bash on Windows) and run `git --version`. If it was installed correctly, you should see an output similar to this:

```shell
git version 2.34.1
```

### Create a GitHub Account

If you don't already have one, go to [GitHub official website](https://github.com) and sign up for a free account. This will be where you'll host your remote repositories.

## First-Time Configuration

Before you make your first commit, you need to tell Git who you are. This information will be attached to every commit you create.

Run these two commands in your terminal, replacing the dummy text with your own name and email.

```shell
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

:::info 
Use the same email address you used to sign up for GitHub. This links your commits to your GitHub profile. 
:::

## Setting Up an SSH Key

When you clone, push, or pull, you need to authenticate with GitHub. You can use HTTPS (which prompts for your username and password/token) or SSH.

*SSH keys are a more secure and convenient* way to authenticate. You generate a "key pair" (a private key that stays on your computer and a public key you give to GitHub). GitHub can then verify your identity without you ever typing a password.

### Check for Existing Keys

First, check if you already have an SSH key pair.

```shell
ls -al ~/.ssh
```

:::info 
If you see files named `id_rsa` and `id_rsa.pub`, or `id_ed25519` and `id_ed25519.pub`, you can skip this part.
:::

### Generate a New SSH Key

If you don't have a key, generate a new one. *Ed25519* is the recommended algorithm.

```shell
ssh-keygen -t ed25519 -C "your_email@example.com"
```

:::info
- When prompted to "Enter a file in which to save the key," press Enter to accept the default location.

- You'll be asked to "Enter passphrase." This is an optional password for your key itself. It adds another layer of security. Pressing Enter skips it.
:::

### Add Your SSH Key to GitHub

Now you need to give your public key (`.pub` file) to GitHub.

1. *Copy the key* to your clipboard.

2. *Go to GitHub*:

    - Click your profile picture in the top-right corner and go to *Settings*.
    - In the "Access" menu on the left, click *SSH and GPG keys*.
    - Click the *New SSH key* button.
    - Give it a *Title* (e.g., "My laptop").
    - *Paste* your copied key into the "Key" field.
    - Click *Add SSH key*.

### Test Your Connection

Run the following command in your terminal:

```shell
ssh -T git@github.com
```

You may see a warning about the authenticity of the host. Type `yes`. If everything worked, you'll see a message like: `Hi YourUsername! You've successfully authenticated...`

From now on, when you clone a repository, make sure to use the SSH URL, not the HTTPS one. `git@github.com:YourUsername/existing-project.git`

## The Basic Git Workflow

This is the core loop you'll use most of the time. We'll cover two main scenarios: creating a new project and working on an existing one.

### Creating a New Project (Local-First)

Use this when you're starting a brand-new project on your computer.

#### Initialize a Repository

First, create a folder for your project, `cd` into it, and then run `git init`.

```shell
# Create a new directory and move into it
mkdir my-first-project
cd my-first-project

# Tell Git to start tracking this folder
git init
```

This creates a hidden `.git` folder in your directory. This is where Git stores all its tracking information.

#### The Add & Commit Loop

This is how you create "save points."

1. **Create a file**. Let's make a `README.md` file.

```shell
echo "Hello, Git!" > README.md
```

2. **Check the status**. Run `git status` to see what Git knows about.
```shell
On branch main
No commits yet
Untracked files:
  (use "git add <file>..." to include in what will be committed)
        README.md
```

Git sees the new file but isn't tracking it yet (it's "untracked").

3. **Stage your changes**. Before you commit, you must "stage" your changes. This is like putting files in a box to be saved. Use `git add` to stage them.

```shell
# Stage the README.md file
git add README.md
```

:::note
*What is the "Staging Area"?*

1. *Working Directory*: Your actual project folder where you edit files.
2. *Staging Area (or "Index")*: An intermediate area. When you `git add` a file, you are not saving it yet. You are simply adding a "snapshot" of that file (as it is right now) to the staging area.
3. *Repository (`.git` folder)*: The permanent history of commits.
:::

4. **Commit your changes**. Now, you "commit" (save) everything currently in the staging area.

```shell
git commit -m "Initial commit: Add README file"
```

:::info
A commit requires a message (using the `-m` flag) to describe the change.
:::

You've just made your first commit! You can see your project's history by running `git log`.

### Working on an Existing Project (Remote-First)

This is the most common scenario. You're joining a project that already exists on GitHub.

#### Clone the Repository

You "clone" the remote repository, which downloads a full copy of the project and its Git history to your computer.

1. Go to the project's page on GitHub.
2. Click the green `< > Code` button.
3. Copy the **SSH** URL (if you set up an SSH key) or the **HTTPS** URL.
4. Run `git clone [URL]` in your terminal.

```shell
# Use the SSH URL (recommended)
git clone git@github.com:YourUsername/existing-project.git

# Or, use the HTTPS URL
git clone https://github.com/YourUsername/existing-project.git

# This creates a folder named 'existing-project', so move into it
cd existing-project
```
:::info
This automatically sets up the connection to the remote repository (which Git calls `origin`).
:::

## Working with GitHub

Now let's connect your local commits to the GitHub.

### Create a Remote Repository on GitHub

If you started from a local repository, your project exists only locally. You need to create a matching empty repository on GitHub.4

1. On GitHub, click the **+** icon in the top-right and select **New repository**.
2. Give it the **same name** as your local folder (e.g., `my-first-project`).
3. **Do NOT** check "Add a README file," "Add .gitignore," or "Choose a license." We want an empty project, since we already have our own files.
4. Click **Create repository**.

### Connect and Push Your Commits

GitHub will now show you a page with some commands. We want the "push an existing repository from the command line" section.

1. **Add the remote**. This tells your local repo where `origin` (GitHub) is. Copy the command from GitHub (use the SSH version if you set it up).

```shell
# For SSH
git remote add origin git@github.com:YourUsername/my-first-project.git
# For HTTPS
# git remote add origin https://github.com/YourUsername/my-first-project.git
```

2. **Rename your branch**. By default, Git might call your main branch `master`. GitHub's standard is `main`. Let's rename it.

```shell
git branch -M main
```

3. **Push your code**. "Push" means "upload" your local commits to the `origin` remote's `main` branch.

```shell
git push -u origin main
```

:::info
The `-u` flag sets the "upstream" tracking. You only need to do this once per branch.
:::

:::warning
If you used HTTPS, you'll be prompted for your GitHub username and password/token!
:::

Now, refresh your GitHub page. You'll see your `README.md` file is there!

### Pulling and Pushing Changes

Now that your local and remote repos are connected, your new workflow looks like this:

1. **Pull Changes (Fetch & Merge)**: Before you start working, always get the latest changes from the remote.

```shell
# Fetches changes from 'origin' and merges them into your current branch
git pull
```

:::info
If you're working with a team, do this often!
:::

2. **Add & Commit**: Make your local changes as normal.

```shell
echo "Another line" >> README.md
git add README.md
git commit -m "Update README"
```

3. **Push Changes**: Upload your new commits to GitHub.

```shell
# Since we already set the upstream with -u, we can just run:
git push
```

## Branching, Merging, and Rebasing

This is the real power of Git, allowing for collaboration and safe development.

The `main` branch should **always** contain stable, working code. You should never work directly on `main`. Instead, you create a branch to work on your new feature.

### Branching: Working in Parallel

A branch is a lightweight movable pointer to one of your commits. When you create a branch, you're creating a new, independent timeline of commits.

1. **Create and switch to a new branch**: The most common workflow is to create a new branch and immediately switch to it. The `-b` flag does both.

```shell
# Make sure you are on main and it's up to date
git checkout main
git pull

# Create a new branch named 'my-new-feature' and switch to it
git checkout -b my-new-feature
```

:::info
This is a shortcut for `git branch my-new-feature` followed by `git checkout my-new-feature`.
:::

2. **Do your work**: Now, you are on the `my-new-feature` branch. You can `add` and `commit` changes here as much as you want. Your `main` branch will remain untouched and stable.

```shell
# ... edit some files ...
git add .
git commit -m "Implement amazing new feature"
```

3. **Push your branch to GitHub**: The first time you push a new branch, you need to set its upstream.

```shell
git push -u origin my-new-feature
```

### Merging: Combining Your Work

Once your feature is complete and tested, you'll want to combine it back into the `main` branch. This is called **merging**.

A merge takes the changes from your feature branch and integrates them into `main`, creating a special **merge commit** that ties the two histories together.

```shell
# 1. Switch back to the main branch
git checkout main

# 2. Make sure main is up-to-date with the remote
git pull origin main

# 3. Merge your feature branch into main
git merge my-new-feature
```

Your `main` branch now contains all the work from `my-new-feature`.

:::note  
If you and another person both changed the exact same lines in the exact same file (one on `main`, one on your branch), Git won't know which change to keep. This is a **merge conflict**.

Git will stop the merge and mark the file(s) as "conflicted." You must manually open the file, delete the lines you don't want, and save the correct version. Then, you `git add` the fixed file and run `git commit` to finalize the merge. 
:::

### Rebasing: An Alternative to Merging

**Rebasing** is the other primary method for integrating changes from one branch to another. While a merge creates a new "merge commit" that ties two commit histories together, rebase achieves integration by **rewriting your branch's history**.

**How to rebase**: Let's say `main` has new commits since you started your branch. You want to "catch up" to `main` before you merge.

```shell
# 1. Switch to your feature branch
git checkout my-new-feature

# 2. "Rebase" your branch on top of main
git rebase main
```

Git will "rewind" your branch, pull in the latest changes from `main`, and then "replay" your commits one by one on top. You may have to solve conflicts, but you do it for each commit, one at a time.

Your feature branch now looks like it was started from the very latest version of `main`. Now, merging it is simple:

```shell
git checkout main
git merge my-new-feature  # This will be a "fast-forward" merge, no merge commit
```

:::warning
Never rebase a branch that other people are using (like `main` or any shared branch).

Rebasing rewrites history (it changes the commit IDs). If you rebase a shared branch, you are changing the "truth" of the project, and it will cause massive problems for everyone else who tries to pull or push.

**Only rebase your own local branches that nobody else is working on**.
:::

### Pull Requests: The GitHub Workflow

In a team, you almost never merge directly on your own computer. Instead, you use **Pull Requests (PRs)** on GitHub.

A Pull Request is a formal request to merge your branch into another (e.g., merge `my-new-feature` into `main`).

1. **Push** your feature branch to GitHub.

```shell
git push -u origin my-new-feature
```

2. **Go to GitHub**. You will see a green button to "Compare & pull request." Click it.
3. **Fill out the PR**: Give it a title and a description of your changes. This is where you can tag teammates for a **code review**.
4. **Discuss and Review**: Your team can now look at your changes, leave comments, and request fixes.
5. **Merge**: Once approved, a team member (or you) can click the green "Merge" button on the GitHub website. This performs the merge on the remote and keeps a record of the discussion.

:::info
This workflow (Branch -> Push -> Pull Request -> Review -> Merge) is the foundation of modern software collaboration.
:::
