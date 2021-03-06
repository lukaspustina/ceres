# NAME

ceres.conf - configures profiles and others settings for ceres. This configuration file uses TOML syntax. It may define multiple profiles with different names.

# DESCRIPTION

*default_profile* = "\<profile name\>"

[logging]

default = "warn"

ceres = "info"

[github]

token = "\<your github token\>"

[pivotal]

token = "\<your pivotal token\>"

[status_pages."\status page name\>"]
id = "\<your status page id\>"

[profiles."\<profile name\>"]

ssh_user = "\<a user name\>" (*optional*)

local_base_dir = "\<path to your infrastructure as code sub-directory of your CenterDevice infrastructure repo\>" (*optional*)

[profiles."\<profile name\>".issue_tracker]

github_org = "\<your github org\>"

github_repo = "\<your github repo\>"

project_number = \<number of corresponding github project\>

[profiles."\<profile name\>".story_tracker]

project_id = \<number of corresponding pivotal project\>

[profiles."\<profile name\>".provider] (*optional*)

type = "aws"

access_key_id = "\<access key id\>"

secret_access_key = "\<secret access key\>"

token = "\<session token\>"

region = "\<AWS region string\>"

role_arn = "\<role arn\>"

[profiles."\<profile name\>".consul] (*optional*)

urls = ["\<URL to your consul server or agent\>", ...]

[profiles."\<profile name\>".health]

base_domain = "\<base domain name of your CenterDevice instance\>"

root_ca = \"PEM file containing additional Root CA cert\" (*optional*)

[profiles."\<profile name\>".centerdevice] (*optional*)

client_id = \"CenterDevice client id for ceres\"

client_secret = \"CenterDevice client secret for ceres\"

redirect_uri = \"CenterDevice client redirect URL for ceres\"

base_domain = \"Base domain for distributor associated with this profile\"

access_token = \"Access token\" (*optional*)

refresh_token = \"Refresh token\" (*optional*)

root_ca = \"PEM file containing additional Root CA cert\" (*optional*)

# SEE ALSO
  ceres(1)

# COPYRIGHT AND LICENSE

Copyright (c) CenterDevice. Licensed under the MIT License. See *https://github.com/centerdevice/ceres/blob/master/LICENSE* for details.

