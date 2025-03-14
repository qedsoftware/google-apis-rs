<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-connectors1` library allows access to all features of the *Google Connectors* service.

This documentation was generated from *Connectors* crate version *6.0.0+20240619*, where *20240619* is the exact revision of the *connectors:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *Connectors* *v1* API can be found at the
[official documentation site](https://cloud.google.com/apigee/docs/api-platform/connectors/about-connectors).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/Connectors) ...

* projects
 * [*locations connections connection schema metadata get action*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionConnectionSchemaMetadataGetActionCall), [*locations connections connection schema metadata get entity type*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionConnectionSchemaMetadataGetEntityTypeCall), [*locations connections connection schema metadata list actions*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionConnectionSchemaMetadataListActionCall), [*locations connections connection schema metadata list entity types*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionConnectionSchemaMetadataListEntityTypeCall), [*locations connections connection schema metadata refresh*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionConnectionSchemaMetadataRefreshCall), [*locations connections create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionCreateCall), [*locations connections delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionDeleteCall), [*locations connections event subscriptions create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionCreateCall), [*locations connections event subscriptions delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionDeleteCall), [*locations connections event subscriptions get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionGetCall), [*locations connections event subscriptions list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionListCall), [*locations connections event subscriptions patch*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionPatchCall), [*locations connections event subscriptions retry*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionEventSubscriptionRetryCall), [*locations connections get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionGetCall), [*locations connections get connection schema metadata*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionGetConnectionSchemaMetadataCall), [*locations connections get iam policy*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionGetIamPolicyCall), [*locations connections list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionListCall), [*locations connections listen event*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionListenEventCall), [*locations connections patch*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionPatchCall), [*locations connections repair eventing*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionRepairEventingCall), [*locations connections runtime action schemas list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionRuntimeActionSchemaListCall), [*locations connections runtime entity schemas list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionRuntimeEntitySchemaListCall), [*locations connections search*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionSearchCall), [*locations connections set iam policy*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionSetIamPolicyCall), [*locations connections test iam permissions*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationConnectionTestIamPermissionCall), [*locations custom connectors custom connector versions delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationCustomConnectorCustomConnectorVersionDeleteCall), [*locations custom connectors custom connector versions deprecate*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationCustomConnectorCustomConnectorVersionDeprecateCall), [*locations custom connectors validate custom connector spec*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationCustomConnectorValidateCustomConnectorSpecCall), [*locations endpoint attachments create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationEndpointAttachmentCreateCall), [*locations endpoint attachments delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationEndpointAttachmentDeleteCall), [*locations endpoint attachments get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationEndpointAttachmentGetCall), [*locations endpoint attachments list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationEndpointAttachmentListCall), [*locations endpoint attachments patch*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationEndpointAttachmentPatchCall), [*locations get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGetCall), [*locations get regional settings*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGetRegionalSettingCall), [*locations get runtime config*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGetRuntimeConfigCall), [*locations global custom connectors create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorCreateCall), [*locations global custom connectors custom connector versions create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorCustomConnectorVersionCreateCall), [*locations global custom connectors custom connector versions get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorCustomConnectorVersionGetCall), [*locations global custom connectors custom connector versions list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorCustomConnectorVersionListCall), [*locations global custom connectors delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorDeleteCall), [*locations global custom connectors get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorGetCall), [*locations global custom connectors list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorListCall), [*locations global custom connectors patch*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalCustomConnectorPatchCall), [*locations global get settings*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalGetSettingCall), [*locations global managed zones create*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalManagedZoneCreateCall), [*locations global managed zones delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalManagedZoneDeleteCall), [*locations global managed zones get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalManagedZoneGetCall), [*locations global managed zones list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalManagedZoneListCall), [*locations global managed zones patch*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalManagedZonePatchCall), [*locations global update settings*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationGlobalUpdateSettingCall), [*locations list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationListCall), [*locations operations cancel*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationOperationCancelCall), [*locations operations delete*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationOperationDeleteCall), [*locations operations get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationOperationGetCall), [*locations operations list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationOperationListCall), [*locations providers connectors get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorGetCall), [*locations providers connectors list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorListCall), [*locations providers connectors versions eventtypes get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorVersionEventtypeGetCall), [*locations providers connectors versions eventtypes list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorVersionEventtypeListCall), [*locations providers connectors versions get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorVersionGetCall), [*locations providers connectors versions list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderConnectorVersionListCall), [*locations providers get*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderGetCall), [*locations providers get iam policy*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderGetIamPolicyCall), [*locations providers list*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderListCall), [*locations providers set iam policy*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderSetIamPolicyCall), [*locations providers test iam permissions*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationProviderTestIamPermissionCall) and [*locations update regional settings*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/api::ProjectLocationUpdateRegionalSettingCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/Connectors)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::CallBuilder)
* **[Resources](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().locations_connections_connection_schema_metadata_get_action(...).doit().await
let r = hub.projects().locations_connections_connection_schema_metadata_get_entity_type(...).doit().await
let r = hub.projects().locations_connections_connection_schema_metadata_refresh(...).doit().await
let r = hub.projects().locations_connections_event_subscriptions_create(...).doit().await
let r = hub.projects().locations_connections_event_subscriptions_delete(...).doit().await
let r = hub.projects().locations_connections_event_subscriptions_patch(...).doit().await
let r = hub.projects().locations_connections_event_subscriptions_retry(...).doit().await
let r = hub.projects().locations_connections_create(...).doit().await
let r = hub.projects().locations_connections_delete(...).doit().await
let r = hub.projects().locations_connections_patch(...).doit().await
let r = hub.projects().locations_connections_repair_eventing(...).doit().await
let r = hub.projects().locations_custom_connectors_custom_connector_versions_delete(...).doit().await
let r = hub.projects().locations_custom_connectors_custom_connector_versions_deprecate(...).doit().await
let r = hub.projects().locations_endpoint_attachments_create(...).doit().await
let r = hub.projects().locations_endpoint_attachments_delete(...).doit().await
let r = hub.projects().locations_endpoint_attachments_patch(...).doit().await
let r = hub.projects().locations_global_custom_connectors_custom_connector_versions_create(...).doit().await
let r = hub.projects().locations_global_custom_connectors_create(...).doit().await
let r = hub.projects().locations_global_custom_connectors_delete(...).doit().await
let r = hub.projects().locations_global_custom_connectors_patch(...).doit().await
let r = hub.projects().locations_global_managed_zones_create(...).doit().await
let r = hub.projects().locations_global_managed_zones_delete(...).doit().await
let r = hub.projects().locations_global_managed_zones_patch(...).doit().await
let r = hub.projects().locations_global_update_settings(...).doit().await
let r = hub.projects().locations_operations_get(...).doit().await
let r = hub.projects().locations_update_regional_settings(...).doit().await
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
google-connectors1 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_connectors1 as connectors1;
use connectors1::api::EventSubscription;
use connectors1::{Result, Error};
use connectors1::{Connectors, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

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
let mut hub = Connectors::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = EventSubscription::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().locations_connections_event_subscriptions_create(req, "parent")
             .event_subscription_id("magna")
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

All errors produced by the system are provided either as [Result](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Delegate) to the
[Method Builder](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::RequestValue) and
[decodable](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-connectors1/6.0.0+20240619/google_connectors1/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **connectors1** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

