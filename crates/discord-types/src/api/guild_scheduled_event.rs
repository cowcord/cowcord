use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::api::guild::{GuildMember, PrivacyLevel};
use crate::api::users::PartialUser;
use crate::common::id::{ChannelId, EntityId, GenericSnowflake, GuildId, ScheduledEventId, UserId};
use crate::common::timestamp::Timestamp;

#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEvent {
	/// The ID of the scheduled event
	pub id: ScheduledEventId,
	/// The ID of the guild the scheduled event belongs to
	pub guild_id: GuildId,
	/// The ID of the channel in which the scheduled event will be hosted
	pub channel_id: Option<ChannelId>,
	/// The ID of the user that created the scheduled event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub creator_id: Option<Option<UserId>>,
	/// The user that created the scheduled event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub creator: Option<PartialUser>,
	/// The name of the scheduled event (1-100 characters)
	pub name: String,
	/// The description for the scheduled event (1-1000 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<Option<String>>,
	/// When the scheduled event will start
	pub scheduled_start_time: Timestamp,
	/// When the scheduled event will end
	pub scheduled_end_time: Option<Timestamp>,
	/// Whether the event should automatically start at the scheduled start time
	#[serde(skip_serializing_if = "Option::is_none")]
	pub auto_start: Option<bool>,
	/// The privacy level of the scheduled event
	pub privacy_level: PrivacyLevel,
	/// The status of the scheduled event
	pub status: GuildScheduledEventStatus,
	/// The type of scheduled event
	pub entity_type: GuildScheduledEventEntityType,
	/// The ID of an entity associated with the scheduled event
	pub entity_id: Option<EntityId>,
	/// Additional metadata for the scheduled event
	pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
	/// The number of users subscribed to the scheduled event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_count: Option<u32>,
	/// The cover image hash for the scheduled event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<Option<String>>,
	/// The definition for how often this event should recur
	pub recurrence_rule: Option<GuildScheduledEventRecurrenceRule>,
	/// Exceptions to the recurrence rule for this event
	pub guild_scheduled_event_exceptions: Vec<GuildScheduledEventException>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEventEntityMetadata {
	/// Location of the event (1-100 characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
	/// The scheduled event has not started yet
	SCHEDULED = 1,
	/// The scheduled event is currently active
	ACTIVE = 2,
	/// The scheduled event has ended
	COMPLETED = 3,
	/// The scheduled event was canceled
	CANCELED = 4,
}

/// [Validation rules](https://docs.discord.food/resources/guild-scheduled-event#guild-scheduled-event-entity-type-validation)
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventEntityType {
	/// The scheduled event is in a stage channel
	STAGE_INSTANCE = 1,
	/// The scheduled event is in a voice channel
	VOICE = 2,
	/// The scheduled event is somewhere else™ not associated with Discord
	EXTERNAL = 3,
	/// The scheduled event is a prime time event
	PRIME_TIME = 4,
}

/// The current system limitations are present due to how reccurring event data needs to be displayed in the client.
/// In the future, we would like to open the system up to have fewer / none of these restrictions.
///
/// #### The following fields cannot be set by the client
///
/// - `count`
/// - `end`
/// - `by_year_day`
///
/// #### The following combinations are mutually exclusive
///
/// - `by_weekday`
/// - `by_n_weekday`
/// - `by_month` + `by_month_day`
///
/// #### `by_weekday`
///
/// - Only valid for daily and weekly events (`frequency` of `DAILY` or `WEEKLY`)
/// - when used in a daily event (`frequency` is `DAILY`)
///   - The values present in the `by_weekday` event must be a "known set" of weekdays.
///   - The following are current allowed "sets"
///     - Monday - Friday (`[0, 1, 2, 3, 4]`)
///     - Tuesday - Saturday (`[1, 2, 3, 4, 5]`)
///     - Sunday - Thursday (`[6, 0, 1, 2, 3]`)
///     - Friday & Saturday (`[4, 5]`)
///     - Saturday & Sunday (`[5, 6]`)
///     - Sunday & Monday (`[6, 0]`)
/// - when used in a weekly event (`frequency` is `WEEKLY`)
///   - `by_weekday` array currently can only be have length of `1`
///     - i.e: You can only select a single day within a week to have a recurring event on
///     - If you wish to have multiple days within a week have a recurring event, please use a `frequency` of `DAILY`
///   - Also, see `interval` below for "every-other" week information
///
/// #### `by_n_weekday`
///
/// - Only valid for monthly events (`frequency` of `MONTHLY`)
/// - `by_n_weekday` array currently can only have a length of `1`
///   - i.e: You can only select a single day within a month to have a recurring event on
///
/// #### `by_month` and `by_month_day`
///
/// - Only valid for annual event (`frequency` is `YEARLY`)
/// - both `by_month` and `by_month_day` must be provided
/// - both `by_month` and `by_month_day` arrays must have a length of `1`
///   - (i.e. you can only set a single date for annual events)
///
/// #### `interval` can only be set to a value other than `1` when `frequency` is set to `WEEKLY`
///
/// - In this situation, interval can be set to `2`
/// - This allowance enables "every-other week" events
/// - Due to the limitations placed on `by_weekday` this means that if you wish to use "every-other week" functionality
///   you can only do so for a single day.
///
/// #### Every weekday
///
/// ```rs
/// let frequency = 3; // Frequency.DAILY
/// let interval = 1;
/// let by_weekday = vec![0, 1, 2, 3, 4]; // [Weekday::MONDAY, ..., Weekday::FRIDAY]
/// ```
///
/// #### Every Wednesday
///
/// ```rs
/// let frequency = 2; // Frequency::WEEKLY
/// let interval = 1;
/// let by_weekday = vec![2]; // [Weekday::WEDNESDAY]
/// ```
///
/// #### Every other Wednesday
///
/// ```rs
/// let frequency = 2; // Frequency::WEEKLY
/// let interval = 2;
/// let by_weekday = vec![2]; // [Weekday::WEDNESDAY]
/// ```
///
/// #### Monthly on the fourth Wednesday
///
/// ```rs
/// let frequency = 1; // Frequency::MONTHLY
/// let interval = 1;
/// let by_n_weekday = vec![
///   GuildScheduledEventRecurrenceRuleNWeekday {
///     n: 4,
///     day: 2, // Weekday::WEDNESDAY
///   },
/// ];
/// ```
///
/// #### Annually on July 24
///
/// ```rs
/// let frequency = 0; // Frequency::YEARLY
/// let interval = 1;
/// let by_month = vec![7]; // [Month::JULY]
/// let by_month_day = vec![24];
/// ```
#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEventRecurrenceRule {
	/// Starting time of the recurrence interval
	pub start: Timestamp,
	/// Ending time of the recurrence interval
	pub end: Option<Timestamp>,
	/// How often the event occurs
	pub frequency: GuildScheduledEventRecurrenceRuleFrequency,
	/// The spacing between the events, defined by frequency ; for example, frequency of WEEKLY and an interval of 2 would be "every other week"
	pub interval: u8,
	/// Specific days within a week for the event to recur on
	pub by_weekday: Option<Vec<GuildScheduledEventRecurrenceRuleWeekday>>,
	/// Specific days within a specific week (1-5) to recur on
	pub by_n_weekday: Option<Vec<GuildScheduledEventRecurrenceRuleNWeekday>>,
	/// Specific months to recur on
	pub by_month: Option<Vec<GuildScheduledEventRecurrenceRuleMonth>>,
	/// Specific dates within a month to recur on
	pub by_month_day: Option<Vec<u8>>,
	/// Specific days within a year to recur on (1-364)
	pub by_year_day: Option<Vec<NonZeroU16>>,
	/// The total amount of times that the event is allowed to recur before stopping
	pub count: Option<u32>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleFrequency {
	YEARLY = 0,
	MONTHLY = 1,
	WEEKLY = 2,
	DAILY = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleWeekday {
	MONDAY = 0,
	TUESDAY = 1,
	WEDNESDAY = 2,
	THURSDAY = 3,
	FRIDAY = 4,
	SATURDAY = 5,
	SUNDAY = 6,
}

#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEventRecurrenceRuleNWeekday {
	/// The week to reccur on (1-5)
	pub n: u8,
	/// The day within the week to reccur on
	pub day: u8,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventRecurrenceRuleMonth {
	JANUARY = 1,
	FEBRUARY = 2,
	MARCH = 3,
	APRIL = 4,
	MAY = 5,
	JUNE = 6,
	JULY = 7,
	AUGUST = 8,
	SEPTEMBER = 9,
	OCTOBER = 10,
	NOVEMBER = 11,
	DECEMBER = 12,
}

#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEventException {
	/// The ID of the scheduled event the exception is for
	pub event_id: ScheduledEventId,
	/// A snowflake representing when the scheduled event would have started without the exception
	pub event_exception_id: GenericSnowflake,
	/// Whether the scheduled event will be skipped on this recurrence
	pub is_canceled: bool,
	/// The scheduled event's modified start time for this recurrence
	pub scheduled_start_time: Option<Timestamp>,
	/// The scheduled event's modified end time for this recurrence
	pub scheduled_end_time: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildScheduledEventUser {
	/// The ID of the scheduled event the user subscribed to
	pub guild_scheduled_event_id: ScheduledEventId,
	/// The ID of the specific exception this subscription is for, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_scheduled_event_exception_id: Option<ScheduledEventId>,
	/// The user's response to the scheduled event
	pub response: GuildScheduledEventUserResponse,
	/// The ID of the user that subscribed to the scheduled event
	pub user_id: UserId,
	/// The user that subscribed to the scheduled event
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PartialUser>,
	/// Guild member data for the user in the scheduled event's guild, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub member: Option<GuildMember>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildScheduledEventUserResponse {
	/// User is not interested in the occurrence
	UNINTERESTED = 0,
	/// User is interested in the event or occurrence
	INTERESTED = 1,
}
