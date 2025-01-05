# This is a simple Makefile to deploy the necessary infrastructure to run the
# hello_world example.
#
# You do not need to use this; this is just a way to document the infrastructure
# and permissions needed to run the example.

tf_args := -var-file=config.tfvars
tf_action := plan
tf_command := tofu # Alternatively, set this to `terraform`

version:
	$(tf_command) version

init: tf_action := init
init: tf_args += -backend-config=backend_config.tfvars -migrate-state
init:
	$(tf_command) $(tf_action) $(tf_args)

plan: tf_action := plan
plan:
	$(tf_command) $(tf_action) $(tf_args)

apply: tf_action := apply
apply:
	$(tf_command) $(tf_action) $(tf_args)
