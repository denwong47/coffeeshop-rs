## Hello, World Example for Coffeeshop

This example is a minimal example of an application using the Coffeeshop
framework. It demonstrates how to create a simple application that takes API
requests, distribute them to workers via SQS, and return the results using
DynamoDB as a cache.

## Running the Example

To run the example, you will need to have an AWS account and have your
credentials set up. You can set up your credentials by following the
[official guidance](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html).

You will also need to setup the necessary resources for the example, which includes:

- An SQS queue
- A DynamoDB table, and
- An IAM role with the necessary permissions.

### Setting up the resources

An example set of Terraform/OpenTofu configurations are provided in the root folder, which
uses the `infrastructure` module in the main crate.

You can copy

- `backend_config.tfvars.example` to `backend_config.tfvars` and
- `config.tfvars.example` to `config.tfvars`

The backend configuration expects the state files to be stored on an S3 bucket. You can
remove the backend configuration if you want to store the state files locally.

The `config.tfvars` file contains the AWS region and profile name to use, as well as the
name of the app in question, defaulting to `hello-world`. The rest of this guide assumes
this preset value to be used.

You can then run the following commands to set up the resources:

```sh
make init
make apply
```

Upon confirmation, the resources will be created, with an output like below:

```
Apply complete! Resources: 5 added, 0 changed, 0 destroyed.

Outputs:

resources = {
  "dynamodb" = {
    "arn" = "arn:aws:dynamodb:us-west-2:012345678901:table/task-queue-hello-world"
    "attributes" = {
      "identifier" = {
        "type" = "S"
      }
    }
    "name" = "task-queue-hello-world"
    "partition_key" = "identifier"
  }
  "iam_role" = {
    "attachment" = {
      "policy_arn" = "arn:aws:iam::012345678901:policy/CoffeeshopHelloWorldPolicy"
      "role" = "CoffeeshopHelloWorldRole"
    }
    "policy" = {
      "arn" = "arn:aws:iam::012345678901:policy/CoffeeshopHelloWorldPolicy"
      "name" = "CoffeeshopHelloWorldPolicy"
    }
    "role" = {
      "arn" = "arn:aws:iam::012345678901:role/CoffeeshopHelloWorldRole"
      "name" = "CoffeeshopHelloWorldRole"
    }
  }
  "sqs" = {
    "arn" = "arn:aws:sqs:us-west-2:012345678901:task-queue-hello-world"
    "name" = "task-queue-hello-world"
    "url" = "https://sqs.us-west-2.amazonaws.com/012345678901/task-queue-hello-world"
  }
}
```

For the next step, you will need the `iam_role.role.arn`.

### Running the application

We need to assume the role above to run the application. You can do this with the
following command:

```sh
export $( \
    printf "AWS_ACCESS_KEY_ID=%s AWS_SECRET_ACCESS_KEY=%s AWS_SESSION_TOKEN=%s" \
        $(aws sts assume-role \
        --role-arn arn:aws:iam::012345678901:role/CoffeeshopHelloWorldRole \
        --role-session-name task-queue-hello-world-demo \
        --query "Credentials.[AccessKeyId,SecretAccessKey,SessionToken]" --output text)) \
    && cargo run --release --example hello_world -- --baristas 16
```

If you want to run the application without assuming the role, you can simply run:

```sh
cargo run --release --example hello_world -- --baristas 16
```

> [!NOTE]
> Replace the `role-arn` with the ARN of the role you created in the previous step.

> [!TIP]
> You can also set any application configuration after the `--` flag. In the
> above example, we set the number of baristas to 16 by passing `--baristas 16`.
> Since the processing for this example is essentially a no-op, you can increase
> the number of baristas to see the effect of parallel processing.

> [!WARNING]
> The application will run indefinitely until you stop it, however your role session
> will expire after an hour by default. The application will discover the error and
> gracefully exit, but you may need to restart it by re-assuming the role as above.

## Pinging the application

You can ping the application by sending a `GET` request to the `/status` endpoint; it
will return a `200 OK` response with a JSON body containing the status of the application.

```json
{
    "metadata": {
        "hostname": "myhost",
        "timestamp": "2025-01-07T20:30:04.459952506Z",
        "uptime": 25.028522319
    },
    "request_count": 0,
    "ticket_count": 0
}
```

To make actual requests, you can use the `curl` command:

```sh
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name": "Alice", "age": 42}' \
    http://localhost:7007/request?language=es&timeout=5
```

This will return a JSON response with a result similar to:

```json
{
    "ticket": "febb0b75-0f8e-44d7-91f7-6e2d2c626220",
    "metadata": {
        "hostname": "myhost",
        "timestamp": "2025-01-07T20:34:19.797336685Z",
        "uptime": 280.365906447
    },
    "output": {
        "greeting": "Ciao, Alice! 1983 is a good year to be born in.",
        "answer_id": 240
    }
}
```

You can also make asynchronous requests by setting the `async` query parameter to `true`:

```sh
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name": "Alice", "age": 42}' \
    http://localhost:7007/request?language=es&timeout=5&async=true
```

This will return a `202 Accepted` response with a JSON body containing the ticket:

```json
{
    "metadata": {
        "hostname": "myhost",
        "timestamp": "2025-01-07T20:35:59.316473493Z",
        "uptime": 379.885043245
    },
    "ticket": "0a22ae44-6dc6-496e-82c7-52ea6797e676"
}
```

With this ticket number, you can then request the result of the operation in the
future:

```sh
curl -X GET \
    http://localhost:7007/retrieve\?ticket=0a22ae44-6dc6-496e-82c7-52ea6797e676
```

Which will return the same JSON response as the synchronous request.

## Using the sample client script

A sample client script in Python is provided in the `client` folder; the only
dependency is the `httpx` library, which you can install with:

```sh
pip install httpx
```

After which you can run the script with:

```sh
python client/main.py
```

Which will send 200 synchronous requests and 200 asynchronous requests to the
application, and print the results.
