use rand::{RngExt, distr::Alphanumeric};
use rounac_uci::v2_5::{
	choices::OwnerProducerChoiceType,
	enums::{ClassificationEnum, MessageModeEnum, OwnerProducerEnum},
	types::{
		HeaderType, IdType, MissionIdType, SecurityInformationType, ServiceIdType, SystemIdType,
	},
};
use uuid::Uuid;

pub fn random_alphanum(len: usize) -> String {
	let mut rng = rand::rng();
	(0..len).map(|_| rng.sample(Alphanumeric) as char).collect()
}

/// Returns empty security information for an unclassified USA producer.
pub fn security_info() -> SecurityInformationType {
	SecurityInformationType {
		classification: ClassificationEnum::U,
		owner_producer: vec![OwnerProducerChoiceType::GovernmentIdentifier(
			OwnerProducerEnum::Usa,
		)],
		joint: None,
		sci_controls: Vec::new(),
		sar_identifier: Vec::new(),
		atomic_energy_markings: Vec::new(),
		dissemination_controls: Vec::new(),
		display_only_to: Vec::new(),
		fgi_source_open: Vec::new(),
		fgi_source_protected: Vec::new(),
		releasable_to: Vec::new(),
		non_ic_markings: Vec::new(),
		classified_by: None,
		compilation_reason: None,
		derivatively_classified_by: None,
		classification_reason: None,
		non_us_controls: Vec::new(),
		derived_from: None,
		declass_date: None,
		declass_event: None,
		declass_exception: Vec::new(),
		has_approximate_markings: None,
		high_water_nato: Vec::new(),
		cui_basic: Vec::new(),
		cui_specified: Vec::new(),
		cui_decontrol_date: None,
		cui_decontrol_event: None,
		cui_controlled_by: None,
		cui_controlled_by_office: None,
		cui_poc: None,
		second_banner_line: Vec::new(),
		handle_via_channels: None,
	}
}

/// Returns a message header for the given parameters.
pub fn header() -> HeaderType {
	HeaderType {
		system_id: system_id(),
		timestamp: chrono::Utc::now(),
		schema_version: random_alphanum(16),
		mode: MessageModeEnum::NonexerciseSimulation,
		service_id: Some(service_id()),
		mission_id: Some(mission_id()),
	}
}

pub fn service_id() -> ServiceIdType {
	ServiceIdType {
		uuid: Uuid::new_v4(),
		descriptive_label: Some(random_alphanum(256)),
		service_version: Some(random_alphanum(12)),
	}
}

pub fn system_id() -> SystemIdType {
	SystemIdType {
		uuid: Uuid::new_v4(),
		descriptive_label: Some(random_alphanum(256)),
	}
}

pub fn mission_id() -> MissionIdType {
	MissionIdType {
		uuid: Uuid::new_v4(),
		descriptive_label: Some(random_alphanum(256)),
		version: Some(rand::random()),
	}
}

pub fn id() -> IdType {
	IdType {
		uuid: Uuid::new_v4(),
		descriptive_label: Some(random_alphanum(256)),
	}
}

pub mod service_status {
	use std::ops::Range;

	use super::*;
	pub use rounac_uci::v2_5::elements::ServiceStatus;
	use rounac_uci::v2_5::{
		enums::{CannotComplyEnum, ServiceStateEnum},
		types::{CannotComplyType, ServiceStatusMdt, ServiceStatusMt},
	};

	/// Returns a service status for `service_id` with 0 uptime and normal state.
	fn service_status_mdt() -> ServiceStatusMdt {
		const SECONDS_RANGE: Range<i64> = Range {
			start: 0,
			end: u32::MAX as _,
		};

		ServiceStatusMdt {
			service_id: service_id(),
			time_up: chrono::TimeDelta::seconds(rand::random_range(SECONDS_RANGE)).into(),
			service_state: ServiceStateEnum::Normal,
			service_state_reason: (0..4).map(|_| cannot_comply()).collect(),
			predicted_service_state: Vec::new(),
			enabled_settings: Vec::new(),
			supported_settings: Vec::new(),
		}
	}

	/// Creates a [ServiceStatus] with random values. Message should be mostly
	/// valid but will be a bit non-sensical.
	pub fn service_status() -> ServiceStatus {
		ServiceStatus(ServiceStatusMt {
			security_information: security_info(),
			message_header: header(),
			message_data: service_status_mdt(),
		})
	}

	fn cannot_comply() -> CannotComplyType {
		CannotComplyType {
			reason: CannotComplyEnum::Unknown,
			description: Some(random_alphanum(512)),
			associated_id: Some(id()),
		}
	}
}
