# Luxon

Luxon **will be** a dotfiles manager with an emphasis on full-system configuration, easy one-liner installation and deployment, and templating and profile features for managing similar configs for different target systems.

You can get started with any of the following commands:

```
sh -c "$(curl -fsLS luxon.sh/i)" -- init $GIT_USERNAME
# OR
sh -c "$(curl -fsLS luxon.sh/i)" -- init $GIT_REMOTE_URL
# OR if you've never used luxon before
sh -c "$(curl -fsLS luxon.sh/i)" -- init
```

where `$GIT_USERNAME` is the username for GitHub, GitLab, or sourcehut (services are searched in that order).
where `$GIT_REMOTE_URL` is the HTTPS or SSH remote for the desired dotfiles repository.

### Features

- Templating
- Scripting via hooks
- Can manage more folders than just `$HOME` (including, but not limited to `/etc`)
- Packaged for the AUR. Open to packaging for more distributions via PR.
- Easy one-liner installation and deployment (install script will prefer package manager install)
- Preserves file permissions
- No external dependencies

