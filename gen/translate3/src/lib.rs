// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Translate* crate version *6.0.0+20240301*, where *20240301* is the exact revision of the *translate:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.
//!
//! Everything else about the *Translate* *v3* API can be found at the
//! [official documentation site](https://cloud.google.com/translate/docs/quickstarts).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/translate3).
//! # Features
//!
//! Handle the following *Resources* with ease from the central [hub](Translate) ...
//!
//! * projects
//!  * [*detect language*](api::ProjectDetectLanguageCall), [*get supported languages*](api::ProjectGetSupportedLanguageCall), [*locations adaptive mt datasets adaptive mt files adaptive mt sentences list*](api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileAdaptiveMtSentenceListCall), [*locations adaptive mt datasets adaptive mt files delete*](api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileDeleteCall), [*locations adaptive mt datasets adaptive mt files get*](api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileGetCall), [*locations adaptive mt datasets adaptive mt files list*](api::ProjectLocationAdaptiveMtDatasetAdaptiveMtFileListCall), [*locations adaptive mt datasets adaptive mt sentences list*](api::ProjectLocationAdaptiveMtDatasetAdaptiveMtSentenceListCall), [*locations adaptive mt datasets create*](api::ProjectLocationAdaptiveMtDatasetCreateCall), [*locations adaptive mt datasets delete*](api::ProjectLocationAdaptiveMtDatasetDeleteCall), [*locations adaptive mt datasets get*](api::ProjectLocationAdaptiveMtDatasetGetCall), [*locations adaptive mt datasets import adaptive mt file*](api::ProjectLocationAdaptiveMtDatasetImportAdaptiveMtFileCall), [*locations adaptive mt datasets list*](api::ProjectLocationAdaptiveMtDatasetListCall), [*locations adaptive mt translate*](api::ProjectLocationAdaptiveMtTranslateCall), [*locations batch translate document*](api::ProjectLocationBatchTranslateDocumentCall), [*locations batch translate text*](api::ProjectLocationBatchTranslateTextCall), [*locations datasets create*](api::ProjectLocationDatasetCreateCall), [*locations datasets delete*](api::ProjectLocationDatasetDeleteCall), [*locations datasets examples list*](api::ProjectLocationDatasetExampleListCall), [*locations datasets export data*](api::ProjectLocationDatasetExportDataCall), [*locations datasets get*](api::ProjectLocationDatasetGetCall), [*locations datasets import data*](api::ProjectLocationDatasetImportDataCall), [*locations datasets list*](api::ProjectLocationDatasetListCall), [*locations detect language*](api::ProjectLocationDetectLanguageCall), [*locations get*](api::ProjectLocationGetCall), [*locations get supported languages*](api::ProjectLocationGetSupportedLanguageCall), [*locations glossaries create*](api::ProjectLocationGlossaryCreateCall), [*locations glossaries delete*](api::ProjectLocationGlossaryDeleteCall), [*locations glossaries get*](api::ProjectLocationGlossaryGetCall), [*locations glossaries glossary entries create*](api::ProjectLocationGlossaryGlossaryEntryCreateCall), [*locations glossaries glossary entries delete*](api::ProjectLocationGlossaryGlossaryEntryDeleteCall), [*locations glossaries glossary entries get*](api::ProjectLocationGlossaryGlossaryEntryGetCall), [*locations glossaries glossary entries list*](api::ProjectLocationGlossaryGlossaryEntryListCall), [*locations glossaries glossary entries patch*](api::ProjectLocationGlossaryGlossaryEntryPatchCall), [*locations glossaries list*](api::ProjectLocationGlossaryListCall), [*locations glossaries patch*](api::ProjectLocationGlossaryPatchCall), [*locations list*](api::ProjectLocationListCall), [*locations models create*](api::ProjectLocationModelCreateCall), [*locations models delete*](api::ProjectLocationModelDeleteCall), [*locations models get*](api::ProjectLocationModelGetCall), [*locations models list*](api::ProjectLocationModelListCall), [*locations operations cancel*](api::ProjectLocationOperationCancelCall), [*locations operations delete*](api::ProjectLocationOperationDeleteCall), [*locations operations get*](api::ProjectLocationOperationGetCall), [*locations operations list*](api::ProjectLocationOperationListCall), [*locations operations wait*](api::ProjectLocationOperationWaitCall), [*locations romanize text*](api::ProjectLocationRomanizeTextCall), [*locations translate document*](api::ProjectLocationTranslateDocumentCall), [*locations translate text*](api::ProjectLocationTranslateTextCall), [*romanize text*](api::ProjectRomanizeTextCall) and [*translate text*](api::ProjectTranslateTextCall)
//!
//!
//!
//!
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//!
//! # Structure of this Library
//!
//! The API is structured into the following primary items:
//!
//! * **[Hub](Translate)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](common::MethodsBuilder) which in turn
//!       allow access to individual [*Call Builders*](common::CallBuilder)
//! * **[Resources](common::Resource)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](common::Part)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](common::CallBuilder)**
//!     * operations to apply to *Resources*
//!
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//!
//! Generally speaking, you can invoke *Activities* like this:
//!
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit().await
//! ```
//!
//! Or specifically ...
//!
//! ```ignore
//! let r = hub.projects().locations_datasets_create(...).doit().await
//! let r = hub.projects().locations_datasets_delete(...).doit().await
//! let r = hub.projects().locations_datasets_export_data(...).doit().await
//! let r = hub.projects().locations_datasets_import_data(...).doit().await
//! let r = hub.projects().locations_glossaries_create(...).doit().await
//! let r = hub.projects().locations_glossaries_delete(...).doit().await
//! let r = hub.projects().locations_glossaries_patch(...).doit().await
//! let r = hub.projects().locations_models_create(...).doit().await
//! let r = hub.projects().locations_models_delete(...).doit().await
//! let r = hub.projects().locations_operations_get(...).doit().await
//! let r = hub.projects().locations_operations_wait(...).doit().await
//! let r = hub.projects().locations_batch_translate_document(...).doit().await
//! let r = hub.projects().locations_batch_translate_text(...).doit().await
//! ```
//!
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities`
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//!
//! # Usage
//!
//! ## Setting up your Project
//!
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! google-translate3 = "*"
//! serde = "1"
//! serde_json = "1"
//! ```
//!
//! ## A complete example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate google_translate3 as translate3;
//! use translate3::api::Glossary;
//! use translate3::{Result, Error};
//! # async fn dox() {
//! use translate3::{Translate, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
//!
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and
//! // `client_secret`, among other things.
//! let secret: yup_oauth2::ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you,
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
//!     secret,
//!     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//! ).build().await.unwrap();
//!
//! let client = hyper_util::client::legacy::Client::builder(
//!     hyper_util::rt::TokioExecutor::new()
//! )
//! .build(
//!     hyper_rustls::HttpsConnectorBuilder::new()
//!         .with_native_roots()
//!         .unwrap()
//!         .https_or_http()
//!         .enable_http1()
//!         .build()
//! );
//! let mut hub = Translate::new(client, auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Glossary::default();
//!
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().locations_glossaries_patch(req, "name")
//!              .update_mask(FieldMask::new::<&str>(&[]))
//!              .doit().await;
//!
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::Io(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//!
//! All errors produced by the system are provided either as [Result](common::Result) enumeration as return value of
//! the doit() methods, or handed as possibly intermediate results to either the
//! [Hub Delegate](common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//!
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
//! makes the system potentially resilient to all kinds of errors.
//!
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](common::Result), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](common::ResponseResult), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//!
//! Methods supporting uploads can do so using up to 2 different protocols:
//! *simple* and *resumable*. The distinctiveness of each is represented by customized
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//!
//! ## Customization and Callbacks
//!
//! You may alter the way an `doit()` method is called by providing a [delegate](common::Delegate) to the
//! [Method Builder](common::CallBuilder) before making the final `doit()` call.
//! Respective methods will be called to provide progress information, as well as determine whether the system should
//! retry on failure.
//!
//! The [delegate trait](common::Delegate) is default-implemented, allowing you to customize it with minimal effort.
//!
//! ## Optional Parts in Server-Requests
//!
//! All structures provided by this library are made to be [encodable](common::RequestValue) and
//! [decodable](common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
//! are valid.
//! Most optionals are are considered [Parts](common::Part) which are identifiable by name, which will be sent to
//! the server to indicate either the set parts of the request or the desired parts in the response.
//!
//! ## Builder Arguments
//!
//! Using [method builders](common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//!
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](common::RequestValue) are moved
//!
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//!
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//!
//! ## Cargo Features
//!
//! * `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
//! the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
//! generated `openapi` spec would be invalid.
//!
//!
//!

// Unused attributes happen thanks to defined, but unused structures We don't
// warn about this, as depending on the API, some data structures or facilities
// are never used. Instead of pre-determining this, we just disable the lint.
// It's manually tuned to not have any unused imports in fully featured APIs.
// Same with unused_mut.
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/api/lib.rs.mako'
// DO NOT EDIT !

pub extern crate hyper;
pub extern crate hyper_rustls;
pub extern crate hyper_util;
#[cfg(feature = "yup-oauth2")]
pub extern crate yup_oauth2;

pub extern crate google_apis_common as common;
pub use common::{Delegate, Error, FieldMask, Result};

pub mod api;
pub use api::Translate;
