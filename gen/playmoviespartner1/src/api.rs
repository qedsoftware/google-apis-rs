#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// View the digital assets you publish on Google Play Movies and TV
    PlaymovyPartnerReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PlaymovyPartnerReadonly => {
                "https://www.googleapis.com/auth/playmovies_partner.readonly"
            }
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::PlaymovyPartnerReadonly
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all PlayMovies related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// use playmoviespartner1::{Result, Error};
/// # async fn dox() {
/// use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct PlayMovies<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for PlayMovies<C> {}

impl<'a, C> PlayMovies<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> PlayMovies<C> {
        PlayMovies {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://playmoviespartner.googleapis.com/".to_string(),
            _root_url: "https://playmoviespartner.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C> {
        AccountMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://playmoviespartner.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://playmoviespartner.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// An Order tracks the fulfillment of an Edit when delivered using the
/// legacy, non-component-based delivery.
///
/// Each Order is uniquely identified by an `order_id`, which is generated
/// by Google.
///
/// Externally, Orders can also be identified by partners using its `custom_id`
/// (when provided).
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [orders get accounts](AccountOrderGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    /// Countries where the Order is available,
    /// using the "ISO 3166-1 alpha-2" format (example: "US").
    pub countries: Option<Vec<String>>,
    /// Detailed status of the order
    #[serde(rename = "statusDetail")]
    pub status_detail: Option<String>,
    /// High-level status of the order.
    pub status: Option<String>,
    /// Timestamp of the earliest start date of the Avails
    /// linked to this Order.
    #[serde(rename = "earliestAvailStartTime")]
    pub earliest_avail_start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Default Edit name,
    /// usually in the language of the country of origin.
    /// Example: "Googlers, The".
    pub name: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename = "studioName")]
    pub studio_name: Option<String>,
    /// Timestamp when the Order was fulfilled.
    #[serde(rename = "receivedTime")]
    pub received_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Default Season name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename = "seasonName")]
    pub season_name: Option<String>,
    /// ID that can be used to externally identify an Order.
    /// This ID is provided by partners when submitting the Avails.
    /// Example: 'GOOGLER_2006'
    #[serde(rename = "customId")]
    pub custom_id: Option<String>,
    /// YouTube Channel Name that should be used to fulfill the Order.
    /// Example: "Google_channel".
    #[serde(rename = "channelName")]
    pub channel_name: Option<String>,
    /// Timestamp when the Order was approved.
    #[serde(rename = "approvedTime")]
    pub approved_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Default Show name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename = "showName")]
    pub show_name: Option<String>,
    /// A simpler representation of the priority.
    #[serde(rename = "normalizedPriority")]
    pub normalized_priority: Option<String>,
    /// ID internally generated by Google to uniquely identify an Order.
    /// Example: 'abcde12_x'
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    /// Type of the Edit linked to the Order.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Field explaining why an Order has been rejected.
    /// Example: "Trailer audio is 2ch mono, please re-deliver in stereo".
    #[serde(rename = "rejectionNote")]
    pub rejection_note: Option<String>,
    /// YouTube Channel ID that should be used to fulfill the Order.
    /// Example: "UCRG64darCZhb".
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    /// Legacy Order priority, as defined by Google.
    /// Example: 'P0'
    #[serde(rename = "legacyPriority")]
    pub legacy_priority: Option<String>,
    /// Name of the post-production house that manages the Edit ordered.
    #[serde(rename = "pphName")]
    pub pph_name: Option<String>,
    /// Timestamp when the Order was created.
    #[serde(rename = "orderedTime")]
    pub ordered_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Order priority, as defined by Google.
    /// The higher the value, the higher the priority.
    /// Example: 90
    pub priority: Option<f64>,
    /// Google-generated ID identifying the video linked to this Order, once
    /// delivered.
    /// Example: 'gtry456_xc'.
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    /// Default Episode name,
    /// usually in the language of the country of origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - Pilot".
    #[serde(rename = "episodeName")]
    pub episode_name: Option<String>,
}

impl common::ResponseResult for Order {}

/// Response to the ‘ListStoreInfos’ method.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [store infos list accounts](AccountStoreInfoListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListStoreInfosResponse {
    /// See 'List methods rules' for info about this field.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
    /// List of StoreInfos that match the request criteria.
    #[serde(rename = "storeInfos")]
    pub store_infos: Option<Vec<StoreInfo>>,
}

impl common::ResponseResult for ListStoreInfosResponse {}

/// Response to the ‘ListAvails’ method.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [avails list accounts](AccountAvailListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListAvailsResponse {
    /// List of Avails that match the request criteria.
    pub avails: Option<Vec<Avail>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
}

impl common::ResponseResult for ListAvailsResponse {}

/// Information about a playable sequence (video) associated with an Edit
/// and available at the Google Play Store.
///
/// Internally, each StoreInfo is uniquely identified by a `video_id`
/// and `country`.
///
/// Externally, Title-level EIDR or Edit-level EIDR, if provided,
/// can also be used to identify a specific title or edit in a country.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [store infos country get accounts](AccountStoreInfoCountryGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StoreInfo {
    /// Timestamp when the Edit went live on the Store.
    #[serde(rename = "liveTime")]
    pub live_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Google-generated ID identifying the video linked to the Edit.
    /// Example: 'gtry456_xc'
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    /// Whether the Edit has info cards.
    #[serde(rename = "hasInfoCards")]
    pub has_info_cards: Option<bool>,
    /// Whether the Edit has a VOD offer.
    #[serde(rename = "hasVodOffer")]
    pub has_vod_offer: Option<bool>,
    /// Name of the post-production houses that manage the Edit.
    #[serde(rename = "pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<String>,
    /// Name of the studio that owns the Edit ordered.
    #[serde(rename = "studioName")]
    pub studio_name: Option<String>,
    /// Subtitles available for this Edit.
    pub subtitles: Option<Vec<String>>,
    /// Audio tracks available for this Edit.
    #[serde(rename = "audioTracks")]
    pub audio_tracks: Option<Vec<String>>,
    /// Default Show name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The".
    #[serde(rename = "showName")]
    pub show_name: Option<String>,
    /// Country where Edit is available in ISO 3166-1 alpha-2 country
    /// code.
    /// Example: "US".
    pub country: Option<String>,
    /// Google-generated ID identifying the show linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'et2hsue_x'
    #[serde(rename = "showId")]
    pub show_id: Option<String>,
    /// Edit type, like Movie, Episode or Season.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Google-generated ID identifying the trailer linked to the Edit.
    /// Example: 'bhd_4e_cx'
    #[serde(rename = "trailerId")]
    pub trailer_id: Option<String>,
    /// Whether the Edit has a HD offer.
    #[serde(rename = "hasHdOffer")]
    pub has_hd_offer: Option<bool>,
    /// Knowledge Graph ID associated to this Edit, if available.
    /// This ID links the Edit to its knowledge entity, externally accessible
    /// at http://freebase.com.
    /// In the absense of Title EIDR or Edit EIDR, this ID helps link together
    /// multiple Edits across countries.
    /// Example: '/m/0ffx29'
    pub mid: Option<String>,
    /// Whether the Edit has a 5.1 channel audio track.
    #[serde(rename = "hasAudio51")]
    pub has_audio51: Option<bool>,
    /// Default Edit name, usually in the language of the country of
    /// origin.
    /// Example: "Googlers, The".
    pub name: Option<String>,
    /// Google-generated ID identifying the season linked to the Edit.
    /// Only available for TV Edits.
    /// Example: 'ster23ex'
    #[serde(rename = "seasonId")]
    pub season_id: Option<String>,
    /// Title-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename = "titleLevelEidr")]
    pub title_level_eidr: Option<String>,
    /// Default Season name, usually in the language of the country of
    /// origin.
    /// Only available for TV Edits
    /// Example: "Googlers, The - A Brave New World".
    #[serde(rename = "seasonName")]
    pub season_name: Option<String>,
    /// The number assigned to the season within a show.
    /// Only available on TV Edits.
    /// Example: "1".
    #[serde(rename = "seasonNumber")]
    pub season_number: Option<String>,
    /// Whether the Edit has a EST offer.
    #[serde(rename = "hasEstOffer")]
    pub has_est_offer: Option<bool>,
    /// Edit-level EIDR ID.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-6".
    #[serde(rename = "editLevelEidr")]
    pub edit_level_eidr: Option<String>,
    /// Whether the Edit has a SD offer.
    #[serde(rename = "hasSdOffer")]
    pub has_sd_offer: Option<bool>,
}

impl common::ResponseResult for StoreInfo {}

/// An Avail describes the Availability Window of a specific Edit in a given
/// country, which means the period Google is allowed to sell or rent the Edit.
///
/// Avails are exposed in EMA format Version 1.6b (available at
/// http://www.movielabs.com/md/avails/)
///
/// Studios can see the Avails for the Titles they own.
/// Post-production houses cannot see any Avails.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [avails get accounts](AccountAvailGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Avail {
    /// Title used by involved parties to refer to this series.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename = "seriesTitleInternalAlias")]
    pub series_title_internal_alias: Option<String>,
    /// Indicates the format profile covered by the transaction.
    #[serde(rename = "formatProfile")]
    pub format_profile: Option<String>,
    /// Title Identifier. This should be the Title Level EIDR.
    /// Example: "10.5240/1489-49A2-3956-4B2D-FE16-5".
    #[serde(rename = "contentId")]
    pub content_id: Option<String>,
    /// Title used by involved parties to refer to this content.
    /// Example: "Googlers, The".
    /// Only available on Movie Avails.
    #[serde(rename = "titleInternalAlias")]
    pub title_internal_alias: Option<String>,
    /// Value representing the rating.
    /// Ratings should be formatted as per http://www.movielabs.com/md/ratings/
    /// Example: "PG"
    #[serde(rename = "ratingValue")]
    pub rating_value: Option<String>,
    /// Spoken language of the intended audience.
    /// Language shall be encoded in accordance with RFC 5646.
    /// Example: "fr".
    #[serde(rename = "storeLanguage")]
    pub store_language: Option<String>,
    /// Communicating an exempt category as defined by FCC regulations.
    /// It is not required for non-US Avails.
    /// Example: "1"
    #[serde(rename = "captionExemption")]
    pub caption_exemption: Option<String>,
    /// The name of the studio that owns the Edit referred in the Avail.
    /// This is the equivalent of `studio_name` in other resources, but it follows
    /// the EMA nomenclature.
    /// Example: "Google Films".
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Edit Identifier. This should be the Edit Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-6"
    #[serde(rename = "productId")]
    pub product_id: Option<String>,
    /// Title used by involved parties to refer to this season.
    /// Only available on TV Avails.
    /// Example: "Googlers, The".
    #[serde(rename = "seasonTitleInternalAlias")]
    pub season_title_internal_alias: Option<String>,
    /// Other identifier referring to the episode, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1_3".
    #[serde(rename = "episodeAltId")]
    pub episode_alt_id: Option<String>,
    /// Value to be applied to the pricing type.
    /// Example: "4" or "2.99"
    #[serde(rename = "priceValue")]
    pub price_value: Option<String>,
    /// ISO 3166-1 alpha-2 country code for the country or territory
    /// of this Avail.
    /// For Avails, we use Territory in lieu of Country to comply with
    /// EMA specifications.
    /// But please note that Territory and Country identify the same thing.
    /// Example: "US".
    pub territory: Option<String>,
    /// Work type as enumerated in EMA.
    #[serde(rename = "workType")]
    pub work_type: Option<String>,
    /// ID internally generated by Google to uniquely identify an Avail.
    /// Not part of EMA Specs.
    #[serde(rename = "availId")]
    pub avail_id: Option<String>,
    /// Value representing the rating reason.
    /// Rating reasons should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// and comma-separated for inclusion of multiple reasons.
    /// Example: "L, S, V"
    #[serde(rename = "ratingReason")]
    pub rating_reason: Option<String>,
    /// OPTIONAL.TV Only. Title used by involved parties to refer to this episode.
    /// Only available on TV Avails.
    /// Example: "Coding at Google".
    #[serde(rename = "episodeTitleInternalAlias")]
    pub episode_title_internal_alias: Option<String>,
    /// First date an Edit could be publically announced as becoming
    /// available at a specific future date in territory of Avail.
    /// *Not* the Avail start date or pre-order start date.
    /// Format is YYYY-MM-DD.
    /// Only available for pre-orders.
    /// Example: "2012-12-10"
    #[serde(rename = "suppressionLiftDate")]
    pub suppression_lift_date: Option<String>,
    /// Other identifier referring to the season, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers_s1".
    #[serde(rename = "seasonAltId")]
    pub season_alt_id: Option<String>,
    /// Manifestation Identifier. This should be the Manifestation
    /// Level EIDR.
    /// Example: "10.2340/1489-49A2-3956-4B2D-FE16-7"
    #[serde(rename = "encodeId")]
    pub encode_id: Option<String>,
    /// Type of pricing that should be applied to this Avail
    /// based on how the partner classify them.
    /// Example: "Tier", "WSP", "SRP", or "Category".
    #[serde(rename = "priceType")]
    pub price_type: Option<String>,
    /// Communicating if caption file will be delivered.
    #[serde(rename = "captionIncluded")]
    pub caption_included: Option<bool>,
    /// Type of transaction.
    #[serde(rename = "licenseType")]
    pub license_type: Option<String>,
    /// The number assigned to the season within a series.
    /// Only available on TV Avails.
    /// Example: "1".
    #[serde(rename = "seasonNumber")]
    pub season_number: Option<String>,
    /// Release date of the Title in earliest released territory.
    /// Typically it is just the year, but it is free-form as per EMA spec.
    /// Examples: "1979", "Oct 2014"
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    /// End of term in YYYY-MM-DD format in the timezone of the country
    /// of the Avail.
    /// "Open" if no end date is available.
    /// Example: "2019-02-17"
    pub end: Option<String>,
    /// Google-generated ID identifying the video linked to this Avail, once
    /// delivered.
    /// Not part of EMA Specs.
    /// Example: 'gtry456_xc'
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    /// Start of term in YYYY-MM-DD format in the timezone of the
    /// country of the Avail.
    /// Example: "2013-05-14".
    pub start: Option<String>,
    /// Rating system applied to the version of title within territory
    /// of Avail.
    /// Rating systems should be formatted as per
    /// [EMA ratings spec](http://www.movielabs.com/md/ratings/)
    /// Example: "MPAA"
    #[serde(rename = "ratingSystem")]
    pub rating_system: Option<String>,
    /// Name of the post-production houses that manage the Avail.
    /// Not part of EMA Specs.
    #[serde(rename = "pphNames")]
    pub pph_names: Option<Vec<String>>,
    /// Other identifier referring to the series, as defined by partner.
    /// Only available on TV avails.
    /// Example: "rs_googlers".
    #[serde(rename = "seriesAltId")]
    pub series_alt_id: Option<String>,
    /// Other identifier referring to the Edit, as defined by partner.
    /// Example: "GOOGLER_2006"
    #[serde(rename = "altId")]
    pub alt_id: Option<String>,
    /// The number assigned to the episode within a season.
    /// Only available on TV Avails.
    /// Example: "3".
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<String>,
}

impl common::ResponseResult for Avail {}

/// Response to the ‘ListOrders’ method.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [orders list accounts](AccountOrderListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListOrdersResponse {
    /// List of Orders that match the request criteria.
    pub orders: Option<Vec<Order>>,
    /// See _List methods rules_ for info about this field.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// See _List methods rules_ for more information about this field.
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
}

impl common::ResponseResult for ListOrdersResponse {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`PlayMovies`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
///
/// # async fn dox() {
/// use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlayMovies::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `avails_get(...)`, `avails_list(...)`, `orders_get(...)`, `orders_list(...)`, `store_infos_country_get(...)` and `store_infos_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
}

impl<'a, C> common::MethodsBuilder for AccountMethods<'a, C> {}

impl<'a, C> AccountMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// List Orders owned or managed by the partner.
    ///
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn orders_list(&self, account_id: &str) -> AccountOrderListCall<'a, C> {
        AccountOrderListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _custom_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get an Order given its id.
    ///
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `orderId` - REQUIRED. Order ID.
    pub fn orders_get(&self, account_id: &str, order_id: &str) -> AccountOrderGetCall<'a, C> {
        AccountOrderGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// List Avails owned or managed by the partner.
    ///
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn avails_list(&self, account_id: &str) -> AccountAvailListCall<'a, C> {
        AccountAvailListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _title: Default::default(),
            _territories: Default::default(),
            _studio_names: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alt_ids: Default::default(),
            _alt_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get an Avail given its avail group id and avail id.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `availId` - REQUIRED. Avail ID.
    pub fn avails_get(&self, account_id: &str, avail_id: &str) -> AccountAvailGetCall<'a, C> {
        AccountAvailGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _avail_id: avail_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get a StoreInfo given its video id and country.
    ///
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `videoId` - REQUIRED. Video ID.
    /// * `country` - REQUIRED. Edit country.
    pub fn store_infos_country_get(
        &self,
        account_id: &str,
        video_id: &str,
        country: &str,
    ) -> AccountStoreInfoCountryGetCall<'a, C> {
        AccountStoreInfoCountryGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_id: video_id.to_string(),
            _country: country.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// List StoreInfos owned or managed by the partner.
    ///
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    ///
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn store_infos_list(&self, account_id: &str) -> AccountStoreInfoListCall<'a, C> {
        AccountStoreInfoListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _video_id: Default::default(),
            _studio_names: Default::default(),
            _season_ids: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _mids: Default::default(),
            _countries: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// List Orders owned or managed by the partner.
///
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *orders.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_list("accountId")
///              .add_video_ids("sed")
///              .add_studio_names("amet.")
///              .add_status("takimata")
///              .add_pph_names("amet.")
///              .page_token("duo")
///              .page_size(-55)
///              .name("gubergren")
///              .custom_id("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _video_ids: Vec<String>,
    _studio_names: Vec<String>,
    _status: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _custom_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountOrderListCall<'a, C> {}

impl<'a, C> AccountOrderListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListOrdersResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.orders.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "accountId",
            "videoIds",
            "studioNames",
            "status",
            "pphNames",
            "pageToken",
            "pageSize",
            "name",
            "customId",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if !self._video_ids.is_empty() {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if !self._studio_names.is_empty() {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if !self._status.is_empty() {
            for f in self._status.iter() {
                params.push("status", f);
            }
        }
        if !self._pph_names.is_empty() {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }
        if let Some(value) = self._custom_id.as_ref() {
            params.push("customId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Orders that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter Orders that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._status.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountOrderListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches Orders with a `name`, `show`, `season` or `episode`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter Orders that match a case-insensitive, partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountOrderListCall<'a, C> {
        self._custom_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountOrderListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountOrderListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountOrderListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountOrderListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get an Order given its id.
///
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *orders.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_get("accountId", "orderId")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _order_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountOrderGetCall<'a, C> {}

impl<'a, C> AccountOrderGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Order)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.orders.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders/{orderId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{orderId}", "orderId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Order ID.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, C> {
        self._order_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountOrderGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountOrderGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountOrderGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountOrderGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// List Avails owned or managed by the partner.
///
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *avails.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_list("accountId")
///              .add_video_ids("ea")
///              .title("ipsum")
///              .add_territories("invidunt")
///              .add_studio_names("amet")
///              .add_pph_names("duo")
///              .page_token("ipsum")
///              .page_size(-93)
///              .add_alt_ids("ut")
///              .alt_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _video_ids: Vec<String>,
    _title: Option<String>,
    _territories: Vec<String>,
    _studio_names: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _alt_ids: Vec<String>,
    _alt_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountAvailListCall<'a, C> {}

impl<'a, C> AccountAvailListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListAvailsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.avails.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "accountId",
            "videoIds",
            "title",
            "territories",
            "studioNames",
            "pphNames",
            "pageToken",
            "pageSize",
            "altIds",
            "altId",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if !self._video_ids.is_empty() {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if let Some(value) = self._title.as_ref() {
            params.push("title", value);
        }
        if !self._territories.is_empty() {
            for f in self._territories.iter() {
                params.push("territories", f);
            }
        }
        if !self._studio_names.is_empty() {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if !self._pph_names.is_empty() {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if !self._alt_ids.is_empty() {
            for f in self._alt_ids.iter() {
                params.push("altIds", f);
            }
        }
        if let Some(value) = self._alt_id.as_ref() {
            params.push("altId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Avails that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter that matches Avails with a `title_internal_alias`,
    /// `series_title_internal_alias`, `season_title_internal_alias`,
    /// or `episode_title_internal_alias` that contains the given
    /// case-insensitive title.
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given country codes,
    /// using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *territories* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_territories(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._territories.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountAvailListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given partner-specific custom ids.
    ///
    /// Append the given value to the *alt ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_alt_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._alt_ids.push(new_value.to_string());
        self
    }
    /// Filter Avails that match a case-insensitive, partner-specific custom id.
    /// NOTE: this field is deprecated and will be removed on V2; `alt_ids`
    /// should be used instead.
    ///
    /// Sets the *alt id* query property to the given value.
    pub fn alt_id(mut self, new_value: &str) -> AccountAvailListCall<'a, C> {
        self._alt_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountAvailListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountAvailListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountAvailListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountAvailListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get an Avail given its avail group id and avail id.
///
/// A builder for the *avails.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _avail_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountAvailGetCall<'a, C> {}

impl<'a, C> AccountAvailGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Avail)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.avails.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "availId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("availId", self._avail_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails/{availId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{accountId}", "accountId"), ("{availId}", "availId")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["availId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Avail ID.
    ///
    /// Sets the *avail id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn avail_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, C> {
        self._avail_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountAvailGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountAvailGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountAvailGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountAvailGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get a StoreInfo given its video id and country.
///
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.country.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_country_get("accountId", "videoId", "country")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoCountryGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _video_id: String,
    _country: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountStoreInfoCountryGetCall<'a, C> {}

impl<'a, C> AccountStoreInfoCountryGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, StoreInfo)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.storeInfos.country.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "accountId", "videoId", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("videoId", self._video_id);
        params.push("country", self._country);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone()
            + "v1/accounts/{accountId}/storeInfos/{videoId}/country/{country}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{accountId}", "accountId"),
            ("{videoId}", "videoId"),
            ("{country}", "country"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["country", "videoId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Video ID.
    ///
    /// Sets the *video id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C> {
        self._video_id = new_value.to_string();
        self
    }
    /// REQUIRED. Edit country.
    ///
    /// Sets the *country* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn country(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, C> {
        self._country = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountStoreInfoCountryGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoCountryGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountStoreInfoCountryGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountStoreInfoCountryGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountStoreInfoCountryGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// List StoreInfos owned or managed by the partner.
///
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use playmoviespartner1::{PlayMovies, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlayMovies::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_list("accountId")
///              .add_video_ids("ea")
///              .video_id("dolor")
///              .add_studio_names("Lorem")
///              .add_season_ids("eos")
///              .add_pph_names("labore")
///              .page_token("sed")
///              .page_size(-70)
///              .name("sed")
///              .add_mids("no")
///              .add_countries("Stet")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlayMovies<C>,
    _account_id: String,
    _video_ids: Vec<String>,
    _video_id: Option<String>,
    _studio_names: Vec<String>,
    _season_ids: Vec<String>,
    _pph_names: Vec<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _name: Option<String>,
    _mids: Vec<String>,
    _countries: Vec<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AccountStoreInfoListCall<'a, C> {}

impl<'a, C> AccountStoreInfoListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListStoreInfosResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "playmoviespartner.accounts.storeInfos.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "accountId",
            "videoIds",
            "videoId",
            "studioNames",
            "seasonIds",
            "pphNames",
            "pageToken",
            "pageSize",
            "name",
            "mids",
            "countries",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if !self._video_ids.is_empty() {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if let Some(value) = self._video_id.as_ref() {
            params.push("videoId", value);
        }
        if !self._studio_names.is_empty() {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if !self._season_ids.is_empty() {
            for f in self._season_ids.iter() {
                params.push("seasonIds", f);
            }
        }
        if !self._pph_names.is_empty() {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }
        if !self._mids.is_empty() {
            for f in self._mids.iter() {
                params.push("mids", f);
            }
        }
        if !self._countries.is_empty() {
            for f in self._countries.iter() {
                params.push("countries", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/storeInfos";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter StoreInfos that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match a given `video_id`.
    /// NOTE: this field is deprecated and will be removed on V2; `video_ids`
    /// should be used instead.
    ///
    /// Sets the *video id* query property to the given value.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `season_id`s.
    ///
    /// Append the given value to the *season ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_season_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._season_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountStoreInfoListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches StoreInfos with a `name` or `show_name`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `mid`s.
    ///
    /// Append the given value to the *mids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_mids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._mids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match (case-insensitive) any of the given country
    /// codes, using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *countries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_countries(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, C> {
        self._countries.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> AccountStoreInfoListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountStoreInfoListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountStoreInfoListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountStoreInfoListCall<'a, C> {
        self._scopes.clear();
        self
    }
}
