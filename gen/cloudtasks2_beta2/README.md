<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-cloudtasks2_beta2` library allows access to all features of the *Google Cloud Tasks* service.

This documentation was generated from *Cloud Tasks* crate version *6.0.0+20240614*, where *20240614* is the exact revision of the *cloudtasks:v2beta2* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *Cloud Tasks* *v2_beta2* API can be found at the
[official documentation site](https://cloud.google.com/tasks/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/CloudTasks) ...

* api
 * [*queue update*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ApiQueueUpdateCall)
* projects
 * [*locations get*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationGetCall), [*locations get cmek config*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationGetCmekConfigCall), [*locations list*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationListCall), [*locations queues create*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueCreateCall), [*locations queues delete*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueDeleteCall), [*locations queues get*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueGetCall), [*locations queues get iam policy*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueGetIamPolicyCall), [*locations queues list*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueListCall), [*locations queues patch*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueuePatchCall), [*locations queues pause*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueuePauseCall), [*locations queues purge*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueuePurgeCall), [*locations queues resume*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueResumeCall), [*locations queues set iam policy*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueSetIamPolicyCall), [*locations queues tasks acknowledge*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskAcknowledgeCall), [*locations queues tasks buffer*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskBufferCall), [*locations queues tasks cancel lease*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskCancelLeaseCall), [*locations queues tasks create*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskCreateCall), [*locations queues tasks delete*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskDeleteCall), [*locations queues tasks get*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskGetCall), [*locations queues tasks lease*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskLeaseCall), [*locations queues tasks list*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskListCall), [*locations queues tasks renew lease*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskRenewLeaseCall), [*locations queues tasks run*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTaskRunCall), [*locations queues test iam permissions*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationQueueTestIamPermissionCall) and [*locations update cmek config*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/api::ProjectLocationUpdateCmekConfigCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/CloudTasks)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::CallBuilder)
* **[Resources](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_queues_create(...).doit().await
let r = hub.projects().locations_queues_get(...).doit().await
let r = hub.projects().locations_queues_patch(...).doit().await
let r = hub.projects().locations_queues_pause(...).doit().await
let r = hub.projects().locations_queues_purge(...).doit().await
let r = hub.projects().locations_queues_resume(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities`
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-cloudtasks2_beta2 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_cloudtasks2_beta2 as cloudtasks2_beta2;
use cloudtasks2_beta2::api::Queue;
use cloudtasks2_beta2::{Result, Error};
use cloudtasks2_beta2::{CloudTasks, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build()
);
let mut hub = CloudTasks::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Queue::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_queues_patch(req, "name")
             .update_mask(FieldMask::new::<&str>(&[]))
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Delegate) to the
[Method Builder](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::RequestValue) and
[decodable](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-cloudtasks2_beta2/6.0.0+20240614/google_cloudtasks2_beta2/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **cloudtasks2_beta2** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

