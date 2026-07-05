#![doc = r#"Module for all choice types."#]

mod serde_helpers;

use serde::{Deserialize, Serialize};
use serde_helpers::*;

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AccelerationChoiceTypeSerde")]
#[serde(try_from = "AccelerationChoiceTypeSerde")]
pub enum AccelerationChoiceType {
	#[doc = r#"Indicates the navigational acceleration the vehicle will maintain."#]
	AccelerationValue(crate::v2_5::common::AccelerationType),
	#[doc = r#"Indicates the navigational range of acceleration the vehicle will maintain."#]
	AccelerationValueRange(crate::v2_5::types::AccelerationRangeType),
}
choice_convert_impls! {
	AccelerationChoiceType - AccelerationChoiceTypeSerde
	AccelerationValue,
	AccelerationValueRange,
}

#[doc = r#"Specifies the Subject and the associated objects of the AccessAssessment."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AccessAssessmentResultTypeSerde")]
#[serde(try_from = "AccessAssessmentResultTypeSerde")]
pub enum AccessAssessmentResultType {
	#[doc = r#"This element references the results of the assessment in a separate AccessAssessment message.  It is optional based on whether the assessment request specified that the response should result in native message."#]
	AccessAssessmentId(Vec<crate::v2_5::types::AccessAssessmentIdType>),
	#[doc = r#"This element indicates the results of the assessment.  It is optional based on whether the assessment request specified that the response should result in native message."#]
	Assessment(Vec<crate::v2_5::types::AccessAssessmentType>),
}
choice_convert_impls! {
	AccessAssessmentResultType - AccessAssessmentResultTypeSerde
	AccessAssessmentId,
	Assessment,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActionCommandTypeSerde")]
#[serde(try_from = "ActionCommandTypeSerde")]
pub enum ActionCommandType {
	#[doc = r#"Indicates a new invocation of an Action Capability.  Generally, if accepted, the command will result in one or more new Action Activities being created and reported via the ActionActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::ActionCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Action Activity (which was previously reported via the ActionActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent ActionActivity messages."#]
	Activity(crate::v2_5::types::ActivityCommandBaseType),
}
choice_convert_impls! {
	ActionCommandType - ActionCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActionPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "ActionPlanCommandIdChoiceTypeSerde")]
pub enum ActionPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the ActionPlanCommand associated with the ActionPlan."#]
	ActionPlanCommandId(crate::v2_5::types::ActionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the ActionPlanValidationCommand associated with the ActionPlan."#]
	ActionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the ActionPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the ActionPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	ActionPlanCommandIdChoiceType - ActionPlanCommandIdChoiceTypeSerde
	ActionPlanCommandId,
	ActionPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivityActorIdChoiceTypeSerde")]
#[serde(try_from = "ActivityActorIdChoiceTypeSerde")]
pub enum ActivityActorIdChoiceType {
	#[doc = r#"Indicates the unique ID of the System performing the action."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the unique ID of the Capability being used to perform the action."#]
	CapabilityId(crate::v2_5::types::CapabilityIdType),
	#[doc = r#"Indicates the unique ID of the Entity performing the action."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	ActivityActorIdChoiceType - ActivityActorIdChoiceTypeSerde
	SystemId,
	CapabilityId,
	EntityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivityChoiceTypeSerde")]
#[serde(try_from = "ActivityChoiceTypeSerde")]
pub enum ActivityChoiceType {
	#[doc = r#"Indicates the unique ID of the Effect associated with the Planned Activity."#]
	EffectId(crate::v2_5::types::EffectIdType),
	#[doc = r#"Indicates the unique ID of the Action associated with the Planned Activity."#]
	ActionId(crate::v2_5::types::ActionIdType),
	#[doc = r#"Indicates the unique ID of the Task associated with the Planned Activity."#]
	TaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates the unique ID of the Response associated with the Planned Activity."#]
	ResponseId(crate::v2_5::types::ResponseIdType),
	#[doc = r#"Indicates the CommSupport associated with the Planned Activity."#]
	CommSupport(crate::v2_5::types::CommSupportReferenceType),
	#[doc = r#"Indicates the communications beam pointing associated with the Planned Activity."#]
	CommPointingId(crate::v2_5::types::CommPointingIdType),
	#[doc = r#"Indicates a Capability command that is planned to occur.  This is an abstract polymorphic extension point/element; the CapabilityCommandBaseType used here is an abstract base type used in a Capability-specific type that describes the unique Capability command such as EA_Command.  This abstract element must be replaced by a Capability-specific command type to form a valid planned Activity."#]
	CapabilityCommand(crate::v2_5::types::CapabilityCommandBaseType),
	#[doc = r#"Indicates a Supporting Capability command that is planned to occur.  This is an abstract polymorphic extension point/element; the SupportCapabilityCommandBaseType used here is an abstract base type used in a Capability-specific type that describes the unique Capability command such as StoreManagementCommand.  This abstract element must be replaced by a Supporting Capability-specific command type to form a valid planned Activity."#]
	SupportingCapabilityCommand(crate::v2_5::types::SupportCapabilityCommandBaseType),
	#[doc = r#"Indicates a planned system/vehicle setting that is generally independent of mission, tasking and kinematic planning from a UCI perspective.  These activities are related to operation of the system/vehicle itself as opposed to a mission Capability."#]
	VehicleSettings(crate::v2_5::types::VehicleCommandDataType),
	#[doc = r#"Indicates the unique ID of a Product management Task (ProductOrFileDisseminationTask, ProductProcessingTask, etc.) associated with the Planned Activity."#]
	ProductTaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates a Subsystem command that is planned to occur. This is an abstract polymorphic extension point/element; the SubsystemCommandBaseType used here is an abstract base type used in a Subsystem-Command specific type that describes the unique Subsystem command such as SubsystemCalibrationCommand. This abstract element must be replaced by a Subsystem-Command specific type to form a valid planned Activity."#]
	SubsystemCommand(crate::v2_5::types::SubsystemCommandBaseType),
}
choice_convert_impls! {
	ActivityChoiceType - ActivityChoiceTypeSerde
	EffectId,
	ActionId,
	TaskId,
	ResponseId,
	CommSupport,
	CommPointingId,
	CapabilityCommand,
	SupportingCapabilityCommand,
	VehicleSettings,
	ProductTaskId,
	SubsystemCommand,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivityPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "ActivityPlanCommandIdChoiceTypeSerde")]
pub enum ActivityPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the ActivityPlanCommand that the parent originated from."#]
	ActivityPlanCommandId(crate::v2_5::types::ActivityPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand that the parent originated from."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
}
choice_convert_impls! {
	ActivityPlanCommandIdChoiceType - ActivityPlanCommandIdChoiceTypeSerde
	ActivityPlanCommandId,
	MissionPlanCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivityPlansIdChoiceTypeSerde")]
#[serde(try_from = "ActivityPlansIdChoiceTypeSerde")]
pub enum ActivityPlansIdChoiceType {
	#[doc = r#"Indicates the unique ID of the RouteActivityPlan that is the source of the Activity."#]
	RouteActivityPlanId(crate::v2_5::types::RouteActivityPlanIdType),
	#[doc = r#"Indicates the unique ID of the OrbitActivityPlan that is the source of the Activity."#]
	OrbitActivityPlanId(crate::v2_5::types::OrbitActivityPlanIdType),
	#[doc = r#"Indicates the unique ID of the ActivityPlan that is the source of the Activity."#]
	ActivityPlanId(crate::v2_5::types::ActivityPlanIdType),
}
choice_convert_impls! {
	ActivityPlansIdChoiceType - ActivityPlansIdChoiceTypeSerde
	RouteActivityPlanId,
	OrbitActivityPlanId,
	ActivityPlanId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivitySourceTypeSerde")]
#[serde(try_from = "ActivitySourceTypeSerde")]
pub enum ActivitySourceType {
	#[doc = r#"Indicates a Requirement that is the source or cause of the Activity.  For example, a Task could be the direct source of an Activity when the Subsystem which implements the associated Capability monitors Tasks to determine when to initiate Activities."#]
	Requirement(RequirementInstanceIdChoiceType),
	#[doc = r#"Indicates a MDF that is the source or cause of the Activity.  A MDF could be the direct source of an Activity when the Subsystem which implements the associated Capability has MDF configurable/driven automation of its Capabilities."#]
	Mdf(crate::v2_5::types::MdfReferenceType),
	#[doc = r#"Indicates a *ActivityPlan that is the source or cause of the Activity.  A *ActivityPlan could be the direct source of an Activity when the Subsystem which implements the associated Capability directly follows along with a System *ActivityPlan to determine when to initiate Activities."#]
	ActivityPlan(crate::v2_5::types::ActivityPlanReferenceType),
	#[doc = r#"Indicates an associated message that is the source or cause of the Activity.  For example, a SettingsCommand could be the direct source of an Activity when the Subsystem utilizes Settings to determine when to initiate Activities."#]
	AssociatedMessage(AssociatedMessageSourceType),
}
choice_convert_impls! {
	ActivitySourceType - ActivitySourceTypeSerde
	Requirement,
	Mdf,
	ActivityPlan,
	AssociatedMessage,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ActivityTriggerTypeSerde")]
#[serde(try_from = "ActivityTriggerTypeSerde")]
pub enum ActivityTriggerType {
	#[doc = r#"The time or time window when the planned Activity transition should occur."#]
	TransitionTime(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"The inbound distance to the endpoint of the associated kinematic sequence/segment/path where the planned Activity transition should occur."#]
	DistanceToEndPoint(crate::v2_5::common::DistanceType),
}
choice_convert_impls! {
	ActivityTriggerType - ActivityTriggerTypeSerde
	TransitionTime,
	DistanceToEndPoint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AirSampleCommandTypeSerde")]
#[serde(try_from = "AirSampleCommandTypeSerde")]
pub enum AirSampleCommandType {
	#[doc = r#"Indicates a new invocation of an AirSample Capability.  Generally, if accepted, the command will result in one or more new AirSample Activities being created and reported via the AirSampleActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::AirSampleCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing AirSample Activity (which was previously reported via the AirSampleActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent AirSampleActivity messages."#]
	Activity(crate::v2_5::types::ActivityCommandBaseType),
}
choice_convert_impls! {
	AirSampleCommandType - AirSampleCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AirTargetVolumeCommandTypeSerde")]
#[serde(try_from = "AirTargetVolumeCommandTypeSerde")]
pub enum AirTargetVolumeCommandType {
	#[doc = r#"Indicates the commanded air volume extents."#]
	AirVolume(crate::v2_5::types::AirVolumeCueType),
	#[doc = r#"Indicates the unique ID of an Entity that is the target of a cued AMTI search.  This element can be used as the full specification of the target volume for the cued search.  It can also be used in combination with the sibling AirVolume element to define extents of the cue volume around the Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	AirTargetVolumeCommandType - AirTargetVolumeCommandTypeSerde
	AirVolume,
	EntityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AirTargetVolumeTypeSerde")]
#[serde(try_from = "AirTargetVolumeTypeSerde")]
pub enum AirTargetVolumeType {
	#[doc = r#"Indicates a 3D sensor referenced volume that an air volume capability is directed against.  This will define the volume in terms of azimuth and elevation angles."#]
	AirVolumeSensorReferenced(crate::v2_5::types::AirVolumeSensorReferencedType),
	#[doc = r#"Indicates a 3D area that an air volume capability is directed against.  This should define the latitude/longitude extents of the volume along with any guidance regarding the height of the volume."#]
	AirVolumeLocation(crate::v2_5::types::ZoneType),
}
choice_convert_impls! {
	AirTargetVolumeType - AirTargetVolumeTypeSerde
	AirVolumeSensorReferenced,
	AirVolumeLocation,
}

#[doc = r#"See the annotation in the associated message airfield status data."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AirfieldReferenceIdChoiceTypeSerde")]
#[serde(try_from = "AirfieldReferenceIdChoiceTypeSerde")]
pub enum AirfieldReferenceIdChoiceType {
	#[doc = r#"Indicates the unique ID of the airfield System for a self-report."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the unique Entity ID of the airfield when reported by a third party."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	AirfieldReferenceIdChoiceType - AirfieldReferenceIdChoiceTypeSerde
	SystemId,
	EntityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AirspeedHoldOrConstraintChoiceTypeSerde")]
#[serde(try_from = "AirspeedHoldOrConstraintChoiceTypeSerde")]
pub enum AirspeedHoldOrConstraintChoiceType {
	#[doc = r#"Indicates navigation by airspeed hold."#]
	AirspeedHold(crate::v2_5::types::AirspeedHoldType),
	#[doc = r#"Indicates navigation by updated airspeed hold constraint, seen in the sibling Constraints element."#]
	AirspeedHoldConstraint(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	AirspeedHoldOrConstraintChoiceType - AirspeedHoldOrConstraintChoiceTypeSerde
	AirspeedHold,
	AirspeedHoldConstraint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AltitudeHoldOrConstraintChoiceTypeSerde")]
#[serde(try_from = "AltitudeHoldOrConstraintChoiceTypeSerde")]
pub enum AltitudeHoldOrConstraintChoiceType {
	#[doc = r#"Indicates navigation by altitude hold."#]
	AltitudeHold(crate::v2_5::types::AltitudeReferenceType),
	#[doc = r#"Indicates navigation by updated altitude hold constraint, seen in the sibling Constraints element."#]
	AltitudeHoldConstraint(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	AltitudeHoldOrConstraintChoiceType - AltitudeHoldOrConstraintChoiceTypeSerde
	AltitudeHold,
	AltitudeHoldConstraint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AmtiCommandTypeSerde")]
#[serde(try_from = "AmtiCommandTypeSerde")]
pub enum AmtiCommandType {
	#[doc = r#"Indicates a new invocation of an AMTI Capability.  Generally, if accepted, the command will result in one or more new AMTI Activities being created and reported via the AMTI_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::AmtiCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing AMTI Activity (which was previously reported via the AMTI_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent AMTI_Activity messages."#]
	Activity(crate::v2_5::types::AmtiActivityCommandType),
}
choice_convert_impls! {
	AmtiCommandType - AmtiCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AmtiTargetTypeSerde")]
#[serde(try_from = "AmtiTargetTypeSerde")]
pub enum AmtiTargetType {
	#[doc = r#"Indicates a 3D sensor referenced volume that an air volume capability is directed against.  This will define the volume in terms of azimuth and elevation angles."#]
	AirVolumeSensorReferenced(crate::v2_5::types::AirVolumeSensorReferencedType),
	#[doc = r#"Indicates a 3D area that an A2A capability is directed against.  This should define the latitude/longitude extents of the volume along with any guidance regarding the height of the volume."#]
	AirVolumeLocation(crate::v2_5::types::ZoneType),
	#[doc = r#"Indicates the unique ID of an Entity that is the target of a cued AMTI search.  This element can be used as the full specification of the target volume for the cued search.  It can also be used in combination with the sibling AirVolume element to define extents of the cue volume around the Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	AmtiTargetType - AmtiTargetTypeSerde
	AirVolumeSensorReferenced,
	AirVolumeLocation,
	EntityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AngleChoiceTypeSerde")]
#[serde(try_from = "AngleChoiceTypeSerde")]
pub enum AngleChoiceType {
	#[doc = r#"Indicates the angle desired."#]
	AngleValue(crate::v2_5::common::AngleHalfType),
	#[doc = r#"Indicates the range of angles desired."#]
	AngleValueRange(crate::v2_5::types::AngleHalfPairType),
}
choice_convert_impls! {
	AngleChoiceType - AngleChoiceTypeSerde
	AngleValue,
	AngleValueRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AntennaResourceChoiceTypeSerde")]
#[serde(try_from = "AntennaResourceChoiceTypeSerde")]
pub enum AntennaResourceChoiceType {
	#[doc = r#"Indicates the unique ID of a Resource Type (such a "Transmit, Quarter Sub-Array ") on a Shared Antenna. Defined in Shared Aperture Information Files."#]
	AntennaResourceTypeId(crate::v2_5::types::AntennaResourceIdType),
	#[doc = r#"Indicates the unique ID of a specific resource (such a "Transmit, Quarter Sub-Array Number Three") on a Shared Antenna. Defined in Shared Aperture Information Files."#]
	AntennaResourceInstanceId(crate::v2_5::types::AntennaResourceIdType),
}
choice_convert_impls! {
	AntennaResourceChoiceType - AntennaResourceChoiceTypeSerde
	AntennaResourceTypeId,
	AntennaResourceInstanceId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AoCommandTypeSerde")]
#[serde(try_from = "AoCommandTypeSerde")]
pub enum AoCommandType {
	#[doc = r#"Indicates a new invocation of an AO Capability.  Generally, if accepted, the command will result in one or more new AO Activities being created and reported via the AO_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::AoCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing AO Activity (which was previously reported via the AO_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent AO_Activity messages."#]
	Activity(crate::v2_5::types::AoActivityCommandType),
}
choice_convert_impls! {
	AoCommandType - AoCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"Utilized by RF_ResourceAllocation and RF_ResourceAllocationRequest to indicate the spatial coverage needed for the Activity requesting resources."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AperturePointingOptionsTypeSerde")]
#[serde(try_from = "AperturePointingOptionsTypeSerde")]
pub enum AperturePointingOptionsType {
	#[doc = r#"The expected footprint of transmissions from the comm equipment being pointed, for this service/bandwidth type."#]
	FieldOfRegard(crate::v2_5::types::ForLimitsType),
	#[doc = r#"Indicates a particular installation point defined in the Platform Information File, defining a particular "face" of a multi-faced array, implying a desired FieldOfRegard for the antenna access. Defined in Shared Aperture Information Files."#]
	InstallationIndex(u32),
	#[doc = r#"If included, represents the LOS data to be used by beam management function to service request."#]
	RequestBeamPointingReference(BeamPointingReferenceType),
	#[doc = r#"If included, represents the entity ID of the target to be considered as reference for beam pointing function."#]
	RequestEntityReferenceId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	AperturePointingOptionsType - AperturePointingOptionsTypeSerde
	FieldOfRegard,
	InstallationIndex,
	RequestBeamPointingReference,
	RequestEntityReferenceId,
}

#[doc = r#"Indicates the Approach Angle either in Azimuth / Elevation or a unit vector relative to the body coordinate system of the target."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ApproachAngleTypeSerde")]
#[serde(try_from = "ApproachAngleTypeSerde")]
pub enum ApproachAngleType {
	#[doc = r#"Indicates the Azimuth and Elevation of the approach angle."#]
	AzEl(crate::v2_5::types::AzElReferenceType),
	#[doc = r#"Indicates a unit vector relative to the coordinate system of the target."#]
	Relative(crate::v2_5::types::UnitVectorType),
}
choice_convert_impls! {
	ApproachAngleType - ApproachAngleTypeSerde
	AzEl,
	Relative,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ApprovalRequestItemReferenceTypeSerde")]
#[serde(try_from = "ApprovalRequestItemReferenceTypeSerde")]
pub enum ApprovalRequestItemReferenceType {
	#[doc = r#"Indicates the unique ID of the *Plan that is under review for approval."#]
	PlanApproval(PlanReferenceIdChoiceType),
	#[doc = r#"Indicates the Requirement, including any associated DMPIs, that is under review for approval to execute."#]
	RequirementExecutionApproval(crate::v2_5::types::ApprovalRequestItemType),
	#[doc = r#"Indicates the MissionPlanActivationCommand details that are under review for approval. If more than one instance of this element is given, each should correspond to a different MissionPlanID.  For example, if the intent is to transition from one MissionPlan to another, the new MissionPlan can be activated in one instance and the old MissionPlan can be deactivated in another instance.  This transition would be subject to approval."#]
	MissionPlanActivationApproval(Vec<crate::v2_5::types::MissionPlanActivationCommandType>),
}
choice_convert_impls! {
	ApprovalRequestItemReferenceType - ApprovalRequestItemReferenceTypeSerde
	PlanApproval,
	RequirementExecutionApproval,
	MissionPlanActivationApproval,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ArchiveRequestTypeSerde")]
#[serde(try_from = "ArchiveRequestTypeSerde")]
pub enum ArchiveRequestType {
	#[doc = r#"The item or items should be kept until the Time given by this element."#]
	Time(chrono::DateTime<chrono::Utc>),
	#[doc = r#"The item or items should be kept for the number of days specified by this element."#]
	NumberOfDays(u32),
	#[doc = r#"The item or items should be kept until storage capacity is reached and "old" items must be deleted because space is needed for more recent items."#]
	SpaceNeeded(bool),
	#[doc = r#"The item or items should be kept until manually deleted."#]
	ManuallyDeleted(bool),
	#[doc = r#"The item or items should be kept until the specified number of missions has been exceeded."#]
	NumberOfMissions(u32),
}
choice_convert_impls! {
	ArchiveRequestType - ArchiveRequestTypeSerde
	Time,
	NumberOfDays,
	SpaceNeeded,
	ManuallyDeleted,
	NumberOfMissions,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AreaChoiceTypeSerde")]
#[serde(try_from = "AreaChoiceTypeSerde")]
pub enum AreaChoiceType {
	#[doc = r#"Specifies the vertices of a polygon; three or more points and sides.   These can be absolute geospatial points or they can be geospatial points relative to a specific reference frame."#]
	Polygon(crate::v2_5::types::PolygonType),
	#[doc = r#"An ellipse is defined by a center point and a circular or oval shape. An ellipse can also be defined relative to a reference frame.  Note that there are two degrees of freedom for the ellipse orientation:  1. Using the Orientation field within the ellipse the orientation of the ellipse within the reference frame may be specified.  2.  Using the orientation of the reference frame specifies the orientation of every object relative to the reference frame.  As an example, two ellipses could be specified with different orientations within the reference frame.  When the reference frame turns the orientation of the ellipses relative to each other would not change but they both would rotate relative to true North."#]
	Ellipse(crate::v2_5::types::LocatedEllipseType),
	#[doc = r#"A rectangle is defined by a center point, a width, height, and orientation."#]
	Rectangle(crate::v2_5::types::LocatedRectangleType),
	#[doc = r#"An area defined by two ranges and two angles relative to a geospatial point. The geospatial point can be stationary or it can be relative to a moving object/location so its definition is an offset to a location defined in ReferenceFrame.This allows for defining an area around a location/object that is moving, i.e. the area moves along with a system or entity."#]
	SlantRangeArea(crate::v2_5::types::SlantRangeAreaType),
}
choice_convert_impls! {
	AreaChoiceType - AreaChoiceTypeSerde
	Polygon,
	Ellipse,
	Rectangle,
	SlantRangeArea,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AssessmentRequestTypeSerde")]
#[serde(try_from = "AssessmentRequestTypeSerde")]
pub enum AssessmentRequestType {
	#[doc = r#"This assessment type is utilized to assess predicted Capability utilization along a mission planned route."#]
	CapabilityUtilization(crate::v2_5::types::CapabilityUtilizationRequestType),
	#[doc = r#"This element is used to specify that an assessment of the conflicts along a route is requested."#]
	RouteDeconfliction(crate::v2_5::types::RouteDeconflictionRequestType),
	#[doc = r#"This element is used to specify that detection metrics need to be recomputed along a route due to threats. The data generated for detection metrics are different from threat assessments because detection metrics break down the exposure numbers to a much greater detail in order to generate a route than a threat assessment will."#]
	RouteVulnerabilityMetrics(crate::v2_5::types::RouteVulnerabilityMetricsRequestType),
	#[doc = r#"This element is used to specify that an assessment of the threat exposure along a route is requested."#]
	RouteThreatAssessment(crate::v2_5::types::ThreatAssessmentRequestType),
	#[doc = r#"This element is used to specify that an assessment of the possible zone an Entity could have moved within some given period of time."#]
	TargetMobility(crate::v2_5::types::TargetMobilityRequestType),
	#[doc = r#"This element is used to specify that a threat assessment is being requested for the current position of a vehicle."#]
	VehicleThreatAssessment(crate::v2_5::types::VehicleThreatAssessmentRequestType),
	#[doc = r#"This element is used to specify that a threat nomination assessment is being requested for the given mission plans and entities."#]
	ThreatNominationAssessment(crate::v2_5::types::ThreatNominationAssessmentRequestType),
	#[doc = r#"This element is used to specify that achievability assessment is being requested for the given type of AchievabilityRequest."#]
	AchievabilityAssessment(crate::v2_5::types::AchievabilityAssessmentRequestPet),
}
choice_convert_impls! {
	AssessmentRequestType - AssessmentRequestTypeSerde
	CapabilityUtilization,
	RouteDeconfliction,
	RouteVulnerabilityMetrics,
	RouteThreatAssessment,
	TargetMobility,
	VehicleThreatAssessment,
	ThreatNominationAssessment,
	AchievabilityAssessment,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AssessmentTypeSerde")]
#[serde(try_from = "AssessmentTypeSerde")]
pub enum AssessmentType {
	#[doc = r#"This assessment type is utilized to assess predicted Capability utilization along a mission planned route."#]
	CapabilityUtilization(crate::v2_5::types::CapabilityUtilizationAssessmentType),
	#[doc = r#"This element defines the response to a route deconfliction assessment request."#]
	RouteDeconfliction(crate::v2_5::types::RouteDeconflictionAssessmentType),
	#[doc = r#"Indicates the results of assessment of vulnerability along a route."#]
	RouteVulnerabilityMetrics(crate::v2_5::types::RouteVulnerabilityMetricsAssessmentType),
	#[doc = r#"Indicates the results of assessment of threats along a route."#]
	RouteThreatAssessment(crate::v2_5::types::RouteThreatAssessmentType),
	#[doc = r#"This element defines the response to a target mobility assessment request."#]
	TargetMobility(crate::v2_5::types::TargetMobilityAssessmentType),
	#[doc = r#"This element defines the response to a vehicle threat assessment request."#]
	VehicleThreatAssessment(crate::v2_5::types::VehicleThreatAssessmentType),
	#[doc = r#"This element defines the response to a threat nomination assessment."#]
	ThreatNominationAssessment(crate::v2_5::types::ThreatNominationAssessmentType),
	#[doc = r#"This element defines the response to an achievability assessment."#]
	AchievabilityAssessment(crate::v2_5::types::AchievabilityAssessmentPet),
}
choice_convert_impls! {
	AssessmentType - AssessmentTypeSerde
	CapabilityUtilization,
	RouteDeconfliction,
	RouteVulnerabilityMetrics,
	RouteThreatAssessment,
	TargetMobility,
	VehicleThreatAssessment,
	ThreatNominationAssessment,
	AchievabilityAssessment,
}

#[doc = r#"Provides the container that allows for specifying ways to identify the battlespace object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AssetIdentityChoiceTypeSerde")]
#[serde(try_from = "AssetIdentityChoiceTypeSerde")]
pub enum AssetIdentityChoiceType {
	#[doc = r#"Indicates a System or Entity asset."#]
	ByInstance(AssetType),
	#[doc = r#"Indicates an asset based Identity."#]
	ByIdentity(crate::v2_5::types::IdentityType),
}
choice_convert_impls! {
	AssetIdentityChoiceType - AssetIdentityChoiceTypeSerde
	ByInstance,
	ByIdentity,
}

#[doc = r#"Provides a choice between a System and an Entity."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AssetTypeSerde")]
#[serde(try_from = "AssetTypeSerde")]
pub enum AssetType {
	#[doc = r#"This element contains the ID of a UCI compliant vehicle which is "discoverable" via published of PositionReports and/or SystemStatus messages."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"This element contains the ID of a protected asset which is not reporting PositionReports and/or SystemStatus messages.  The asset is being tracked and reported as a friendly Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	AssetType - AssetTypeSerde
	SystemId,
	EntityId,
}

#[doc = r#"Used to reference an associated message or message element which has generated an Activity.."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AssociatedMessageSourceTypeSerde")]
#[serde(try_from = "AssociatedMessageSourceTypeSerde")]
pub enum AssociatedMessageSourceType {
	#[doc = r#"ID for Settings Command command that defined Bin contents."#]
	SettingsCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"ID associated with a resource request for this activity."#]
	RfResouceRequestIdandOption(Vec<crate::v2_5::types::ResourceRequestIdAndOptionType>),
	#[doc = r#"Reference to the upper level identifier for the priority bin group that is associated with the sibling Priority Bin details."#]
	ActiveScanScheduleProfileIndex(u32),
	#[doc = r#"Identifies the association between access period and emitter bin."#]
	EmitterPriorityBinDetail(Vec<crate::v2_5::types::SharedSourceEmitterPriorityBinDetailType>),
}
choice_convert_impls! {
	AssociatedMessageSourceType - AssociatedMessageSourceTypeSerde
	SettingsCommandId,
	RfResouceRequestIdandOption,
	ActiveScanScheduleProfileIndex,
	EmitterPriorityBinDetail,
}

#[doc = r#"An atomic primitive value."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AtomicValueTypeSerde")]
#[serde(try_from = "AtomicValueTypeSerde")]
pub enum AtomicValueType {
	#[doc = r#"A boolean value."#]
	BooleanValue(bool),
	#[doc = r#"A byte value."#]
	ByteValue(i8),
	#[doc = r#"A unsignedByte value."#]
	UnsignedByteValue(u8),
	#[doc = r#"A short value."#]
	ShortValue(i16),
	#[doc = r#"A unsignedShort value."#]
	UnsignedShortValue(u16),
	#[doc = r#"A int value."#]
	IntValue(i32),
	#[doc = r#"A unsignedInt value."#]
	UnsignedIntValue(u32),
	#[doc = r#"A long value."#]
	LongValue(i64),
	#[doc = r#"A float value."#]
	FloatValue(f32),
	#[doc = r#"A double value."#]
	DoubleValue(f64),
	#[doc = r#"A dateTime value."#]
	DateTimeValue(chrono::DateTime<chrono::Utc>),
	#[doc = r#"A duration value."#]
	DurationValue(chrono::TimeDelta),
	#[doc = r#"A time value."#]
	TimeValue(chrono::NaiveTime),
	#[doc = r#"A string value that is evaluated case-sensitively."#]
	StringValueCaseSensitive(crate::v2_5::common::QueryString4096Type),
	#[doc = r#"A string value that is evaluated case-insensitively."#]
	StringValueCaseInsensitive(crate::v2_5::common::QueryString4096Type),
	#[doc = r#"An enumeration value."#]
	EnumValue(crate::v2_5::common::UciSchemaComponentNameType),
	#[doc = r#"A hexBinary value."#]
	HexBinaryValue(String),
	#[doc = r#"A UUID value."#]
	UuidValue(uuid::Uuid),
}
choice_convert_impls! {
	AtomicValueType - AtomicValueTypeSerde
	BooleanValue,
	ByteValue,
	UnsignedByteValue,
	ShortValue,
	UnsignedShortValue,
	IntValue,
	UnsignedIntValue,
	LongValue,
	FloatValue,
	DoubleValue,
	DateTimeValue,
	DurationValue,
	TimeValue,
	StringValueCaseSensitive,
	StringValueCaseInsensitive,
	EnumValue,
	HexBinaryValue,
	UuidValue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AuthorizationRequestTypeSerde")]
#[serde(try_from = "AuthorizationRequestTypeSerde")]
pub enum AuthorizationRequestType {
	#[doc = r#"Indicates a request to create a new Authorization."#]
	Create(crate::v2_5::types::AuthorizationDetailsType),
	#[doc = r#"Indicates a request to change the state of an existing Authorization."#]
	ChangeState(crate::v2_5::types::AuthorizationChangeStateType),
}
choice_convert_impls! {
	AuthorizationRequestType - AuthorizationRequestTypeSerde
	Create,
	ChangeState,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AutonomousActionStatusChoiceTypeSerde")]
#[serde(try_from = "AutonomousActionStatusChoiceTypeSerde")]
pub enum AutonomousActionStatusChoiceType {
	#[doc = r#"Indicates the status of the autonomous planning action or actions addressing the conflict."#]
	AutonomousPlanningActionStatus(Vec<crate::v2_5::types::AutonomousPlanningActionStatusType>),
	#[doc = r#"Indicates the triggering condition for an autonomous action has occurred but no action will be initiated because the PlanningFunction* indicates "Alert Only"."#]
	AlertOnly(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	AutonomousActionStatusChoiceType - AutonomousActionStatusChoiceTypeSerde
	AutonomousPlanningActionStatus,
	AlertOnly,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "AvailableFuelTypeSerde")]
#[serde(try_from = "AvailableFuelTypeSerde")]
pub enum AvailableFuelType {
	#[doc = r#"Indicates the actual weight of the fuel remaining."#]
	Fuel(crate::v2_5::common::MassType),
	#[doc = r#"Indicates the amount of fuel remaining as a percentage of capacity."#]
	Percent(crate::v2_5::common::PercentType),
}
choice_convert_impls! {
	AvailableFuelType - AvailableFuelTypeSerde
	Fuel,
	Percent,
}

#[doc = r#"Beam pointing reference types which includes Antenna, Body, Inertial, or Geodetic."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "BeamPointingReferenceTypeSerde")]
#[serde(try_from = "BeamPointingReferenceTypeSerde")]
pub enum BeamPointingReferenceType {
	#[doc = r#"Beam pointing in reference to antenna boresight. It is likely that some antenna may be steerable only in azimuth and some may only be steerable in elevation, thus both are optional."#]
	Antenna(crate::v2_5::types::AzElBeamPointingType),
	#[doc = r#"Beam pointing in reference to body of the platform. It is likely that some antenna may be steerable only in azimuth and some may only be steerable in elevation, thus both are optional."#]
	Body(crate::v2_5::types::AzElBeamPointingType),
	#[doc = r#"Beam pointing in reference to inertial frame of reference. It is likely that some antenna may be steerable only in azimuth and some may only be steerable in elevation, thus both Azimuth and Elevation are optional. Range may only be usable for some use case (perhaps with an Omni antenna) but not useful with an AESA antenna, thus it is optional."#]
	Inertial(crate::v2_5::types::AzElRangeBeamPointingType),
	#[doc = r#"Beam pointing in reference to a geodetic point."#]
	Geodetic(crate::v2_5::types::Point2DType),
}
choice_convert_impls! {
	BeamPointingReferenceType - BeamPointingReferenceTypeSerde
	Antenna,
	Body,
	Inertial,
	Geodetic,
}

#[doc = r#"This type defines limited Beam Shaping Direction to the Antenna."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "BeamShapingTypeSerde")]
#[serde(try_from = "BeamShapingTypeSerde")]
pub enum BeamShapingType {
	#[doc = r#"Select the Weighting Function and sidelobe levels."#]
	BeamWeighting(crate::v2_5::types::BeamWeightingType),
	#[doc = r#"Set the desired Beam 3 dB Beam Width."#]
	BeamWidth(crate::v2_5::types::BeamWidthType),
}
choice_convert_impls! {
	BeamShapingType - BeamShapingTypeSerde
	BeamWeighting,
	BeamWidth,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "BlankingSourceTypeSerde")]
#[serde(try_from = "BlankingSourceTypeSerde")]
pub enum BlankingSourceType {
	#[doc = r#"Maximum amount of time blanking is permitted by the sibling SubsystemID.  Set to 0 if blanking of the Activity will not be permitted.  The Tx Subsystem shall not set the TxPercentBlankingLimit for each Subsystem ID to any less than the RF_ResourceAllocation's TxPercentBlankLimit for those Subsystems."#]
	SubsystemId(crate::v2_5::types::SubsystemIdType),
	#[doc = r#"In a digital system, the MFA can issue blanking (when a local mode makes an interruption), or the DigitalPayloads can blank one another."#]
	DigitalPayload(DigitalFunctionType),
}
choice_convert_impls! {
	BlankingSourceType - BlankingSourceTypeSerde
	SubsystemId,
	DigitalPayload,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "BlueVehicleTypeSerde")]
#[serde(try_from = "BlueVehicleTypeSerde")]
pub enum BlueVehicleType {
	#[doc = r#"Defines the type of vehicle that the separation parameters apply to."#]
	EntityIdentity(crate::v2_5::types::IdentityType),
	#[doc = r#"Defines the type of vehicle that the separation parameters apply to. This field should match the Model defined in SystemStatus."#]
	Model(crate::v2_5::common::VisibleString32Type),
}
choice_convert_impls! {
	BlueVehicleType - BlueVehicleTypeSerde
	EntityIdentity,
	Model,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "BoundaryTypeSerde")]
#[serde(try_from = "BoundaryTypeSerde")]
pub enum BoundaryType {
	#[doc = r#"Indicates the footprint boundary as a ground/surface polygon, any part of which can be reached given the remaining endurance."#]
	Polygon(crate::v2_5::types::PolygonType),
	#[doc = r#"Indicates the footprint boundary as a ground/surface polygon, any part of which can be reached given the remaining endurance."#]
	Ellipse(crate::v2_5::types::LocatedEllipseType),
}
choice_convert_impls! {
	BoundaryType - BoundaryTypeSerde
	Polygon,
	Ellipse,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CapabilityAssessmentActionTypeSerde")]
#[serde(try_from = "CapabilityAssessmentActionTypeSerde")]
pub enum CapabilityAssessmentActionType {
	#[doc = r#"Estimated Capability usage (in terms of Tasks) at the specified assessment position."#]
	PlannedTaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates estimated Capability usage (in terms of Capability commands) at the specified assessment position.  This element is of an abstract polymorphic extension type; the CapabilityCommandBaseType PET (PolymorphicExtensionType) used here is an abstract base type that is the parent type for Capability category specific child types.  This abstract element must be replaced by a Capability category-specific type to be valid."#]
	EstimatedCapabilityCommand(crate::v2_5::types::CapabilityCommandBaseType),
	#[doc = r#"Indicates estimated Supporting Capability usage (in terms of Supporting Capability commands) at the specified assessment position.  This element is of an abstract polymorphic extension type; the SupportCapabilityCommandBaseType PET (PolymorphicExtensionType) used here is an abstract base type that is the parent type for Supporting Capability category specific child types.  This abstract element must be replaced by a Supporting Capability category-specific type to be valid."#]
	EstimatedSupportingCapabilityCommand(crate::v2_5::types::SupportCapabilityCommandBaseType),
}
choice_convert_impls! {
	CapabilityAssessmentActionType - CapabilityAssessmentActionTypeSerde
	PlannedTaskId,
	EstimatedCapabilityCommand,
	EstimatedSupportingCapabilityCommand,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CapabilityAssessmentActivityTypeSerde")]
#[serde(try_from = "CapabilityAssessmentActivityTypeSerde")]
pub enum CapabilityAssessmentActivityType {
	#[doc = r#"Indicates estimated Capability usage (in terms of Capability Activity report) at the specified assessment position. The ActivityBaseType used for this element should be replaced by a Capability category-specific type to be valid."#]
	EstimatedCapabilityActivity(crate::v2_5::types::ActivityBaseType),
	#[doc = r#"Indicates estimated Supporting Capability usage (in terms of Supporting Capability status) at the specified assessment position. The SupportCapabilityStatusBaseType used for this element should be replaced by a Supporting Capability category-specific type to be valid."#]
	EstimatedSupportingCapabilityReport(crate::v2_5::types::SupportCapabilityStatusBaseType),
}
choice_convert_impls! {
	CapabilityAssessmentActivityType - CapabilityAssessmentActivityTypeSerde
	EstimatedCapabilityActivity,
	EstimatedSupportingCapabilityReport,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CapabilityCrossReferenceTypeSerde")]
#[serde(try_from = "CapabilityCrossReferenceTypeSerde")]
pub enum CapabilityCrossReferenceType {
	#[doc = r#"Indicates a Capability associated with this Component."#]
	CapabilityId(crate::v2_5::types::CapabilityIdType),
	#[doc = r#"Indicates the unique ID of the Support Capability corresponding to this message."#]
	SupportCapabilityId(crate::v2_5::types::SupportCapabilityIdType),
}
choice_convert_impls! {
	CapabilityCrossReferenceType - CapabilityCrossReferenceTypeSerde
	CapabilityId,
	SupportCapabilityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CargoDeliveryTaskTypeSerde")]
#[serde(try_from = "CargoDeliveryTaskTypeSerde")]
pub enum CargoDeliveryTaskType {
	#[doc = r#"Indicates the details for cargo pickup."#]
	Pickup(crate::v2_5::types::CargoTransitionType),
	#[doc = r#"Indicates the details for cargo dropoff/delivery.  If multiple Dropoff elements are specified, the first element is the primary dropoff point and secondary points follow in priority order, highest to lowest."#]
	Dropoff(Vec<crate::v2_5::types::CargoTransitionType>),
}
choice_convert_impls! {
	CargoDeliveryTaskType - CargoDeliveryTaskTypeSerde
	Pickup,
	Dropoff,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CargoLocationTypeSerde")]
#[serde(try_from = "CargoLocationTypeSerde")]
pub enum CargoLocationType {
	#[doc = r#"When a cargo item is co-located with or being transported by a System, this element, which indicates the unique ID of the System, can be used to infer the cargo location via PositionReport, MissionPlan or other messages."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the geospatial position of the cargo item.   Generally, services are encouraged to provide altitude and/or time data whenever it is known."#]
	Point(crate::v2_5::types::Point2DType),
}
choice_convert_impls! {
	CargoLocationType - CargoLocationTypeSerde
	SystemId,
	Point,
}

#[doc = r#"The CharacterizationChoiceType is a choice type that allows the user to select a specific type of characterization of the object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CharacterizationChoiceTypeSerde")]
#[serde(try_from = "CharacterizationChoiceTypeSerde")]
pub enum CharacterizationChoiceType {
	#[doc = r#"The Frequency parameters are in regards to the RF Frequencies to measure. The min signal strength indicates the lowest threshold of collection. The RF Frequency resolution indicates the granularity of the collection. The frequency range indicates the specific RF band to collect. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	Frequency(crate::v2_5::types::FrequencyParamsType),
	#[doc = r#"The Infra-Red Image parameters describe the desired images to collect. The spectral frequency resolution parameter indicates the size of the wavelength to collect. The temperature resolution parameter indicates the amount of temperature variance to measure. The optional IR image setting parameter indicates the range of the IR frequency to measure.  The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	IrImage(crate::v2_5::types::IrImageParamsType),
	#[doc = r#"The metric observation parameters consist of the desired distance resolution. The optional product resolution time parameter indicates the minimum amount of time needed to get the desired level of resolution for the product."#]
	MetricObservations(crate::v2_5::types::MetricParamsType),
	#[doc = r#"The Narrowband Space Object of Interest (NB_SOI) parameters consists of the amplitude in decibels of the desired Resolution. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	NarrowbandSoi(crate::v2_5::types::NarrowbandSoiParamsType),
	#[doc = r#"The optical Image parameters consist of the overall desired image resolution. The optional product resolution parameter provides guidance on the color depth and horizontal and vertical pixel counts. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	OpticalImage(crate::v2_5::types::OpticalImageParamsType),
	#[doc = r#"The radar cross section (RCS) parameters include: the desired amplitude resolution measure in decibels per square meter, the optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	Rcs(crate::v2_5::types::RcsParamsType),
	#[doc = r#"The Visual Magnitude (VisMag) parameters consists of the amplitude resolution (measured in meters per pixel) and the optional product resolution time parameter (the minimum amount of time needed to get the desired level of resolution for the product)."#]
	VisMag(crate::v2_5::types::VisMagParamsType),
	#[doc = r#"The Wideband Space Object of Interest (WB SOI) parameters consist of the range resolution of both the range and cross range. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	WidebandSoi(crate::v2_5::types::WidebandSoiParamsType),
	#[doc = r#"The Photometry parameters consist of the amplitude in decibels of the desired resolution. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	Photometry(crate::v2_5::types::PhotometryParamsType),
	#[doc = r#"The Color Photometry parameters consist of the desired sensor spectrum to use for the collection. The optional product resolution time parameter is the minimum amount of time needed to get the desired level of resolution for the product."#]
	ColorPhotometry(crate::v2_5::types::ColorPhotometryParamsType),
}
choice_convert_impls! {
	CharacterizationChoiceType - CharacterizationChoiceTypeSerde
	Frequency,
	IrImage,
	MetricObservations,
	NarrowbandSoi,
	OpticalImage,
	Rcs,
	VisMag,
	WidebandSoi,
	Photometry,
	ColorPhotometry,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CharacterizationObjectiveTypeSerde")]
#[serde(try_from = "CharacterizationObjectiveTypeSerde")]
pub enum CharacterizationObjectiveType {
	#[doc = r#"Indicates a task to characterize an object using one or more specified phenomenologies."#]
	PhenomenologyCollection(crate::v2_5::types::CharacterizationOptionsType),
	#[doc = r#"Indicates a task to assess the stability and orientation of an object."#]
	StabilityAndOrientationAssessment(crate::v2_5::types::StabilityCharacterizationType),
	#[doc = r#"Indicates a task to characterize or assess changes to the structure of a spacecraft."#]
	StructureAssessment(StructureAssessmentType),
	#[doc = r#"Indicates a task to verify that the target on the designated orbit is the expected spacecraft."#]
	IdentificationVerification(crate::v2_5::types::IdentificationVerificationType),
	#[doc = r#"Indicates a task to perform collections to detect operations changes of the target object."#]
	OperationsChanges(crate::v2_5::types::SatelliteOperationsChangesCharacterizationType),
}
choice_convert_impls! {
	CharacterizationObjectiveType - CharacterizationObjectiveTypeSerde
	PhenomenologyCollection,
	StabilityAndOrientationAssessment,
	StructureAssessment,
	IdentificationVerification,
	OperationsChanges,
}

#[doc = r#"Used to specify the choice Civil Path Terminator Type associated with the End Point, to include its specific parameters needed."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CivilPathTerminatorTypeSerde")]
#[serde(try_from = "CivilPathTerminatorTypeSerde")]
pub enum CivilPathTerminatorType {
	#[doc = r#"Arc to Fix Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	AfArcToFix(crate::v2_5::common::EmptyType),
	#[doc = r#"Course to Altitude Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	CaCourseToAltitude(crate::v2_5::common::EmptyType),
	#[doc = r#"Course to DME Distance Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	CdCourseToDmedistance(crate::v2_5::common::EmptyType),
	#[doc = r#"Course to Fix Civil Path Terminator Type. This path terminates at a fixed point with a specified course at that fix."#]
	CfCourseToFix(crate::v2_5::types::CfCourseToFixType),
	#[doc = r#"Course to Intercept Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	CiCourseToIntercept(crate::v2_5::common::EmptyType),
	#[doc = r#"Course to Radial Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	CrCourseToRadial(crate::v2_5::common::EmptyType),
	#[doc = r#"Direct to Fix Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	DfDirectToFix(crate::v2_5::common::EmptyType),
	#[doc = r#"Track to Altitude Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	FaTrackToAltitude(crate::v2_5::common::EmptyType),
	#[doc = r#"Track From a Fix to a Distance Along Track Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	FcTrackFromFixToDistanceAlongTrack(crate::v2_5::common::EmptyType),
	#[doc = r#"Track From a Fix to a DME Distance Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	FdTrackFromFixToDmedistance(crate::v2_5::common::EmptyType),
	#[doc = r#"Fix to Manual Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	FmFixToManualTermination(crate::v2_5::common::EmptyType),
	#[doc = r#"Holding With an Altitude Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	HaHoldingWithAltitudeTermination(crate::v2_5::common::EmptyType),
	#[doc = r#"Holding With a Fix Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	HfHoldingWithFixTermination(crate::v2_5::common::EmptyType),
	#[doc = r#"Holding With a Manual Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	HmHoldingWithManualTermination(crate::v2_5::common::EmptyType),
	#[doc = r#"Initial Fix Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	IfInitialFix(crate::v2_5::common::EmptyType),
	#[doc = r#"Procedure Turn to Intercept Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	PiProcedureTurnToIntercept(crate::v2_5::common::EmptyType),
	#[doc = r#"Radius to Fix Civil Path Terminator Type. This path is a constant radius circular path around a defined turn center that terminates at a fixed point."#]
	RfRadiusToFix(crate::v2_5::types::RfRadiusToFixType),
	#[doc = r#"Track to Fix Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	TfTrackToFix(crate::v2_5::common::EmptyType),
	#[doc = r#"Heading to Altitude Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	VaHeadingToAltitude(crate::v2_5::common::EmptyType),
	#[doc = r#"Heading to DME Distance Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	VdHeadingToDmedistanceTermination(crate::v2_5::common::EmptyType),
	#[doc = r#"Heading to Intercept Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	ViHeadingToIntercept(crate::v2_5::common::EmptyType),
	#[doc = r#"Heading to Manual Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	VmHeadingToManual(crate::v2_5::common::EmptyType),
	#[doc = r#"Heading to Radial Termination Civil Path Terminator Type. This element's children elements are subject to change over the course of development."#]
	VrHeadingToRadialTermination(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	CivilPathTerminatorType - CivilPathTerminatorTypeSerde
	AfArcToFix,
	CaCourseToAltitude,
	CdCourseToDmedistance,
	CfCourseToFix,
	CiCourseToIntercept,
	CrCourseToRadial,
	DfDirectToFix,
	FaTrackToAltitude,
	FcTrackFromFixToDistanceAlongTrack,
	FdTrackFromFixToDmedistance,
	FmFixToManualTermination,
	HaHoldingWithAltitudeTermination,
	HfHoldingWithFixTermination,
	HmHoldingWithManualTermination,
	IfInitialFix,
	PiProcedureTurnToIntercept,
	RfRadiusToFix,
	TfTrackToFix,
	VaHeadingToAltitude,
	VdHeadingToDmedistanceTermination,
	ViHeadingToIntercept,
	VmHeadingToManual,
	VrHeadingToRadialTermination,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ClimbRateTypeSerde")]
#[serde(try_from = "ClimbRateTypeSerde")]
pub enum ClimbRateType {
	#[doc = r#"Indicates the climb rate during this PathSegment."#]
	ClimbRateValue(crate::v2_5::common::SpeedType),
	#[doc = r#"Indicates the climb rate range during this PathSegment."#]
	ClimbRateRange(crate::v2_5::types::ClimbRateRangeType),
}
choice_convert_impls! {
	ClimbRateType - ClimbRateTypeSerde
	ClimbRateValue,
	ClimbRateRange,
}

#[doc = r#"Indicates the orientation of the orbital plane in space and of the orbit within its plane."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CoeOrientationTypeSerde")]
#[serde(try_from = "CoeOrientationTypeSerde")]
pub enum CoeOrientationType {
	#[doc = r#"Indicates that the orbit is non-equatorial. A non-equatorial orbit is an orbit with a non-zero inclination."#]
	NonEquatorialOrbit(crate::v2_5::types::CoeNonEquatorialOrientationType),
	#[doc = r#"Indicates the angle measured from the principal direction to perigee in the direction of the spacecraft's motion.  Used for equatorial orbit where RAAN and argument of perigee are undefined.  Units in radians.  Valid values range from zero to 2*PI."#]
	EquatorialOrbit(crate::v2_5::types::CoeEquatorialOrientationType),
}
choice_convert_impls! {
	CoeOrientationType - CoeOrientationTypeSerde
	NonEquatorialOrbit,
	EquatorialOrbit,
}

#[doc = r#"Indicates the set of classic orbital elements (COE) describing a spacecraft's position in an orbit.  Elements describing the size, shape and other characteristics of the orbit are in other types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CoePositionTypeSerde")]
#[serde(try_from = "CoePositionTypeSerde")]
pub enum CoePositionType {
	#[doc = r#"Indicates position of the spacecraft expressed as the angular distance from the pericenter which a fictitious body would have if it moved in a circular orbit, with constant speed, in the same orbital period as the actual body in its elliptical orbit. This means of expressing position is used for non-circular orbits.  Units in radians. Valid values range from zero to 2*PI()."#]
	MeanAnomaly(crate::v2_5::common::AnglePositiveType),
	#[doc = r#"Indicates the position of the spacecraft expressed as the angle along the orbital path from the ascending node to the spacecraft's position in the direction of the spacecraft's motion.  This means of expressing position is used for circular orbits where mean anomaly is undefined.  Units in radians. Valid values range from zero to 2*PI()."#]
	ArgumentOfLatitude(crate::v2_5::common::AnglePositiveType),
	#[doc = r#"Indicates the position expressed as the angle from the principal direction (vernal equinox direction) to the spacecraft's position vector in the direction of the spacecraft's motion.  This means of expressing position is used for circular equatorial orbits where mean anomaly and argument of perigee are undefined.  Units in radians. Valid values range from zero to 2*PI()."#]
	TrueLongitude(crate::v2_5::common::AnglePositiveType),
}
choice_convert_impls! {
	CoePositionType - CoePositionTypeSerde
	MeanAnomaly,
	ArgumentOfLatitude,
	TrueLongitude,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComintCommandTypeSerde")]
#[serde(try_from = "ComintCommandTypeSerde")]
pub enum ComintCommandType {
	#[doc = r#"Indicates a new invocation of a COMINT Capability.  Generally, if accepted, the command will result in one or more new Activities being created and reported via the COMINT_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::ComintCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Activity (which was previously reported via the Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent Activity messages."#]
	Activity(crate::v2_5::types::ComintActivityCommandType),
}
choice_convert_impls! {
	ComintCommandType - ComintCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComintSubCapabilityDetailsTypeSerde")]
#[serde(try_from = "ComintSubCapabilityDetailsTypeSerde")]
pub enum ComintSubCapabilityDetailsType {
	#[doc = r#"Indicates a target the Activity is attempting to acquire."#]
	Acquisition(crate::v2_5::types::ComintAcquisitionTargetType),
	#[doc = r#"Indicates a target the Activity is attempting to determine its bearing or angle of arrival."#]
	DirectionFinding(crate::v2_5::types::ComintDirectionFindingType),
	#[doc = r#"Indicates a target the Activity is attempting to determine its identification."#]
	Identification(crate::v2_5::types::ComintIdentificationType),
	#[doc = r#"Indicates a target the Activity is attempting to determine its location."#]
	Geolocation(crate::v2_5::types::ComintGeolocationType),
	#[doc = r#"Indicates a target the Activity is attempting to determine a finer measurement of the target."#]
	Measurement(crate::v2_5::types::ComintMeasurementType),
}
choice_convert_impls! {
	ComintSubCapabilityDetailsType - ComintSubCapabilityDetailsTypeSerde
	Acquisition,
	DirectionFinding,
	Identification,
	Geolocation,
	Measurement,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComintSubcapabilityChoiceTypeSerde")]
#[serde(try_from = "ComintSubcapabilityChoiceTypeSerde")]
pub enum ComintSubcapabilityChoiceType {
	#[doc = r#"Indicates details for a Command or Task that invokes an acquisition related Subcapability of the associated Capability.  This element is required when an acquisition related Subcapability is being invoked."#]
	Acquisition(crate::v2_5::types::ComintSubcapabilityAcquisitionType),
	#[doc = r#"Indicates details for a Command or Task that invokes an identification related Subcapability of the associated Capability.  This element is required when an identification related Subcapability is being invoked."#]
	Identification(crate::v2_5::types::ComintSubcapabilityIdentificationType),
	#[doc = r#"Indicates details for a Command or Task that invokes a geolocation related Subcapability of the associated Capability.  This element is required when a geolocation related Subcapability is being invoked."#]
	Geolocation(crate::v2_5::types::ComintSubcapabilityGeolocationType),
	#[doc = r#"Indicates details for a Command or Task that invokes a measurement related Subcapability of the associated Capability.  This element is required when a measurement related Subcapability is being invoked."#]
	Measurement(crate::v2_5::types::ComintSubcapabilityMeasurementType),
	#[doc = r#"Indicates details for a Command or Task that invokes a pulse data collection related Subcapability of the associated Capability.  This element is required when a pulse data collection related Subcapability is being invoked."#]
	DataCollect(crate::v2_5::types::ComintSubcapabilityDataCollectType),
}
choice_convert_impls! {
	ComintSubcapabilityChoiceType - ComintSubcapabilityChoiceTypeSerde
	Acquisition,
	Identification,
	Geolocation,
	Measurement,
	DataCollect,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComintSubcapabilityTargetLocationDataTypeSerde")]
#[serde(try_from = "ComintSubcapabilityTargetLocationDataTypeSerde")]
pub enum ComintSubcapabilityTargetLocationDataType {
	#[doc = r#"Indicates the EntityID associated with the pulse data collection."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the dwell's Field of View details associated with the pulse data collection."#]
	DwellFov(crate::v2_5::types::NedConeType),
	#[doc = r#"Indicates the Point Target details associated with the pulse data collection."#]
	PointTarget(crate::v2_5::types::PointTargetType),
}
choice_convert_impls! {
	ComintSubcapabilityTargetLocationDataType - ComintSubcapabilityTargetLocationDataTypeSerde
	EntityId,
	DwellFov,
	PointTarget,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComintTargetTypeSerde")]
#[serde(try_from = "ComintTargetTypeSerde")]
pub enum ComintTargetType {
	#[doc = r#"Indicates the unique ID of a specific Entity that is a target of the acquisition."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates a type of emitter that is a target of the acquisition."#]
	EmitterType(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"Indicates a specific emitter type ***instance*** that is a target of the acquisition."#]
	SpecificEmitter(crate::v2_5::types::SpecificEmitterIdentityType),
	#[doc = r#"Indicates summary signal characteristics that are a target of the acquisition."#]
	SignalDescription(crate::v2_5::types::SignalSummaryType),
	#[doc = r#"Indicates the unique ID of a specific Signal that is a target of the acquisition."#]
	SignalId(crate::v2_5::types::SignalIdType),
	#[doc = r#"Supplements the DwellTarget identification with more general classes of signals to be looked for, such as "CW"."#]
	TargetClass(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	ComintTargetType - ComintTargetTypeSerde
	EntityId,
	EmitterType,
	SpecificEmitter,
	SignalDescription,
	SignalId,
	TargetClass,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommLinkDetailsTypeSerde")]
#[serde(try_from = "CommLinkDetailsTypeSerde")]
pub enum CommLinkDetailsType {
	#[doc = r#"Indicates the uplink and downlink data rate in bits per second when the network is configured to use a Full Duplex communication system."#]
	FullDuplex(crate::v2_5::types::LinkRatesType),
	#[doc = r#"Indicates the data rate in bits per second when the network is configured to use a communication system that can only transmit data, and is not able to receive."#]
	TxSimplex(crate::v2_5::common::DataRateType),
	#[doc = r#"Indicates the uplink and downlink data rate in bits per second when the network is configured to use a Half Duplex communication system."#]
	HalfDuplex(crate::v2_5::types::LinkRatesType),
	#[doc = r#"Indicates the data rate in bits per second when the network is configured to use a communication system that can only receive data, and is not able to transmit."#]
	RxSimplex(crate::v2_5::common::DataRateType),
}
choice_convert_impls! {
	CommLinkDetailsType - CommLinkDetailsTypeSerde
	FullDuplex,
	TxSimplex,
	HalfDuplex,
	RxSimplex,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommPointingChoiceTypeSerde")]
#[serde(try_from = "CommPointingChoiceTypeSerde")]
pub enum CommPointingChoiceType {
	#[doc = r#"Indicates pointing at a fixed location."#]
	Location(PointMultiType),
	#[doc = r#"Indicates pointing at an asset whose location is determined from data in messages associated with the asset such as Entity, PositionReport or OrbitPlan."#]
	Asset(AssetType),
	#[doc = r#"Indicates pointing toward multiple assets whose locations are determined from data in messages associated with the assets such as Entity, PositionReport or OrbitPlan."#]
	MultipleAsset(crate::v2_5::types::AssetMultipleType),
	#[doc = r#"Indicates omnidirectional "pointing"."#]
	Omni(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	CommPointingChoiceType - CommPointingChoiceTypeSerde
	Location,
	Asset,
	MultipleAsset,
	Omni,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommRangeDelayChoiceTypeSerde")]
#[serde(try_from = "CommRangeDelayChoiceTypeSerde")]
pub enum CommRangeDelayChoiceType {
	#[doc = r#"When Range Mode is Active, no time delay is specified (EmptyType) between sending a signal from the terminal to the satellite and from the satellite back to the terminal."#]
	Active(crate::v2_5::common::EmptyType),
	#[doc = r#"When Range Mode is Passive, the time delay is specified in seconds (DurationType) between sending a signal from the terminal to the satellite and from the satellite back to the terminal."#]
	Passive(chrono::TimeDelta),
}
choice_convert_impls! {
	CommRangeDelayChoiceType - CommRangeDelayChoiceTypeSerde
	Active,
	Passive,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommSupportCommandTypeSerde")]
#[serde(try_from = "CommSupportCommandTypeSerde")]
pub enum CommSupportCommandType {
	#[doc = r#"Indicates a new invocation of CommSupport Capability.  Generally, if accepted, the command will result in one or more new CommSupport Activities being created and reported via the CommSupportActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::CommSupportCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing CommSupport Activity (which was previously reported via the CommSupportActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent CommSupportActivity messages."#]
	CommSupportActivityCommand(crate::v2_5::types::CommSupportActivityCommandType),
}
choice_convert_impls! {
	CommSupportCommandType - CommSupportCommandTypeSerde
	Capability,
	CommSupportActivityCommand,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommSupportPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "CommSupportPlanCommandIdChoiceTypeSerde")]
pub enum CommSupportPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the CommSupportPlanCommand associated with the CommSupportPlan."#]
	CommSupportPlanCommandId(crate::v2_5::types::CommSupportPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the CommSupportPlanValidationCommand associated with the CommSupportPlan."#]
	CommSupportPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the CommSupportPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the CommSupportPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	CommSupportPlanCommandIdChoiceType - CommSupportPlanCommandIdChoiceTypeSerde
	CommSupportPlanCommandId,
	CommSupportPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommTerminalCommandTypeSerde")]
#[serde(try_from = "CommTerminalCommandTypeSerde")]
pub enum CommTerminalCommandType {
	#[doc = r#"Indicates a new invocation of a Waveform Capability.  Generally, if accepted, the command will result in one or more new Waveform Activities being created and reported via the Waveform Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::CommTerminalCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Waveform Activity (which was previously reported via the Waveform Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent CommActivity messages."#]
	Activity(crate::v2_5::types::CommTerminalActivityCommandType),
}
choice_convert_impls! {
	CommTerminalCommandType - CommTerminalCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommandResponseTypeSerde")]
#[serde(try_from = "CommandResponseTypeSerde")]
pub enum CommandResponseType {
	#[doc = r#"Indicates an AirSampleCommand response is required."#]
	AirSample(crate::v2_5::types::AirSampleCommandResponseType),
	#[doc = r#"Indicates an AMTI_Command response is required."#]
	Amti(crate::v2_5::types::AmtiCommandResponseType),
	#[doc = r#"Indicates an AO_Command response is required."#]
	Ao(crate::v2_5::types::AoCommandResponseType),
	#[doc = r#"Indicates an COMINT_Command response is required."#]
	Comint(crate::v2_5::types::ComintCommandResponseType),
	#[doc = r#"Indicates a CommandedTimeline response is required."#]
	CommandedTimeline(crate::v2_5::types::CommandedTimelineCommandResponseType),
	#[doc = r#"Indicates a CommSupportCommand response is required."#]
	CommSupport(crate::v2_5::types::CommSupportType),
	#[doc = r#"Indicates a DE_Command response is required."#]
	De(crate::v2_5::types::DeCommandResponseType),
	#[doc = r#"Indicates a CommTerminal command response is required."#]
	CommTerminal(crate::v2_5::enums::CommCapabilityEnum),
	#[doc = r#"Indicates an EA_Command response is required."#]
	Ea(crate::v2_5::types::EaCommandResponseType),
	#[doc = r#"Indicates an ESM_Command response is required."#]
	Esm(crate::v2_5::types::EsmCommandResponseType),
	#[doc = r#"Indicates an IFF command response is required."#]
	Iff(crate::v2_5::types::IffCommandResponseType),
	#[doc = r#"Indicates a Navigation Command response is required."#]
	Navigation(crate::v2_5::types::NavigationCommandDetailsType),
	#[doc = r#"Indicates a Gateway Command response is required."#]
	Gateway(crate::v2_5::types::GatewayCommandResponseType),
	#[doc = r#"Selection indicates a PO capability command response is required."#]
	Po(crate::v2_5::types::PoCommandResponseType),
	#[doc = r#"Indicates a Refuel Command response is required."#]
	Refuel(crate::v2_5::types::RefuelCommandResponseType),
	#[doc = r#"Selection indicates a RadarAltimeter capability command response is required."#]
	RadarAltimeter(crate::v2_5::types::RadarAltimeterCommandResponseType),
	#[doc = r#"Selection indicates a SAR capability command response is required."#]
	Sar(crate::v2_5::types::SarCommandResponseType),
	#[doc = r#"Selection indicates a SMTI capability command response is required."#]
	Smti(crate::v2_5::types::SmtiCommandResponseType),
	#[doc = r#"Selection indicates a Strike capability command response is required."#]
	Strike(StrikeWeaponCommandType),
	#[doc = r#"Selection indicates a Weather Radar capability command response is required."#]
	WeatherRadar(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	CommandResponseType - CommandResponseTypeSerde
	AirSample,
	Amti,
	Ao,
	Comint,
	CommandedTimeline,
	CommSupport,
	De,
	CommTerminal,
	Ea,
	Esm,
	Iff,
	Navigation,
	Gateway,
	Po,
	Refuel,
	RadarAltimeter,
	Sar,
	Smti,
	Strike,
	WeatherRadar,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CommandedTimelineCommandTypeSerde")]
#[serde(try_from = "CommandedTimelineCommandTypeSerde")]
pub enum CommandedTimelineCommandType {
	#[doc = r#"Indicates a new invocation of an CommandedTimeline Capability.  Generally, if accepted, the command will result in one or more new CommandedTimeline Activities being created and reported via the CommandedTimelineActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::CommandedTimelineCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing CommandedTimeline Activity (which was previously reported via the CommandedTimelineActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent CommandedTimelineActivity messages."#]
	Activity(crate::v2_5::types::CommandedTimelineActivityCommandType),
}
choice_convert_impls! {
	CommandedTimelineCommandType - CommandedTimelineCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"A comparable atomic primitive value."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComparableAtomicValueTypeSerde")]
#[serde(try_from = "ComparableAtomicValueTypeSerde")]
pub enum ComparableAtomicValueType {
	#[doc = r#"A byte value."#]
	ByteValue(i8),
	#[doc = r#"An unsignedByte value."#]
	UnsignedByteValue(u8),
	#[doc = r#"A short value."#]
	ShortValue(i16),
	#[doc = r#"An unsignedShort value."#]
	UnsignedShortValue(u16),
	#[doc = r#"An int value."#]
	IntValue(i32),
	#[doc = r#"An unsignedInt value."#]
	UnsignedIntValue(u32),
	#[doc = r#"A long value."#]
	LongValue(i64),
	#[doc = r#"A float value."#]
	FloatValue(f32),
	#[doc = r#"A double value."#]
	DoubleValue(f64),
	#[doc = r#"A dateTime value."#]
	DateTimeValue(chrono::DateTime<chrono::Utc>),
	#[doc = r#"A duration value."#]
	DurationValue(chrono::TimeDelta),
	#[doc = r#"A time value."#]
	TimeValue(chrono::NaiveTime),
	#[doc = r#"A string value. String values are compared by the Unicode Collation Algorithm."#]
	StringValue(crate::v2_5::common::QueryString4096Type),
}
choice_convert_impls! {
	ComparableAtomicValueType - ComparableAtomicValueTypeSerde
	ByteValue,
	UnsignedByteValue,
	ShortValue,
	UnsignedShortValue,
	IntValue,
	UnsignedIntValue,
	LongValue,
	FloatValue,
	DoubleValue,
	DateTimeValue,
	DurationValue,
	TimeValue,
	StringValue,
}

#[doc = r#"The choice type for component configuration type.  Used within the component configuration type to allow representing a recursive tree structure of components/units/parts within a component."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComponentConfigurationChoiceTypeSerde")]
#[serde(try_from = "ComponentConfigurationChoiceTypeSerde")]
pub enum ComponentConfigurationChoiceType {
	#[doc = r#"List of component configurations.  This can be used to specify a recursive tree structure."#]
	ComponentConfigurationList(Vec<crate::v2_5::types::ComponentConfigurationPet>),
	#[doc = r#"A uci:EmptyType used to signal the end of recursion."#]
	Terminator(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	ComponentConfigurationChoiceType - ComponentConfigurationChoiceTypeSerde
	ComponentConfigurationList,
	Terminator,
}

#[doc = r#"Indicates an externally defined identifier for a type of component element that is specific to a system, subsystem, component, or service."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComponentElementIdentifierChoiceTypeSerde")]
#[serde(try_from = "ComponentElementIdentifierChoiceTypeSerde")]
pub enum ComponentElementIdentifierChoiceType {
	#[doc = r#"Indicates an externally defined numeric identifier for a type of component element that is specific to a system, subsystem, component, or service."#]
	ComponentElementNumericIdentifier(u32),
	#[doc = r#"Indicates an externally defined text identifier for a type of component element that is specific to a system, subsystem, component, or service."#]
	ComponentElementKey(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	ComponentElementIdentifierChoiceType - ComponentElementIdentifierChoiceTypeSerde
	ComponentElementNumericIdentifier,
	ComponentElementKey,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ComponentResourceTypeSerde")]
#[serde(try_from = "ComponentResourceTypeSerde")]
pub enum ComponentResourceType {
	#[doc = r#"Allows the requester to identify RF aperture and payload resources, connected through an RF Distribution matrix and controlled by an RF Distribution and Control Subsystem. If AntennaResourceAndBeamConfigurations or SelectPayloadResource are not present, they are controlled by the DependentActivity."#]
	SelectRfComponents(crate::v2_5::types::RfComponentResourceType),
	#[doc = r#"Allows the requester to identify digital MFA and digital MFP resources, along with sample rates and network rates, connected on a digital backbone. If MFA_Address or MFP_Address are not populated, they are controlled by the DependentActivity."#]
	SelectDigitalComponents(crate::v2_5::types::DigitalComponentResourceType),
}
choice_convert_impls! {
	ComponentResourceType - ComponentResourceTypeSerde
	SelectRfComponents,
	SelectDigitalComponents,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ConfigurationParameterValueRestrictionsTypeSerde")]
#[serde(try_from = "ConfigurationParameterValueRestrictionsTypeSerde")]
pub enum ConfigurationParameterValueRestrictionsType {
	#[doc = r#"Specifies the list of supported enumerate values.  This value should be populated if and only if the Type element has been set to ENUM."#]
	Enumeration(Vec<crate::v2_5::common::VisibleString32Type>),
	#[doc = r#"Specifies the range of the parameter.  This value only applies to a parameter type of FLOAT and INT."#]
	Range(crate::v2_5::types::ConfigurationParameterRangeType),
}
choice_convert_impls! {
	ConfigurationParameterValueRestrictionsType - ConfigurationParameterValueRestrictionsTypeSerde
	Enumeration,
	Range,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ContingencyPathSpacingTypeSerde")]
#[serde(try_from = "ContingencyPathSpacingTypeSerde")]
pub enum ContingencyPathSpacingType {
	#[doc = r#"Time along the primary path between contingency path start points."#]
	Duration(chrono::TimeDelta),
	#[doc = r#"Distance along the primary path between contingency path start points."#]
	Distance(crate::v2_5::common::DistanceType),
	#[doc = r#"Specifies that contingency paths should be generated starting at previously existing segment endpoints that are contained in the system's primary path."#]
	Endpoints(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	ContingencyPathSpacingType - ContingencyPathSpacingTypeSerde
	Duration,
	Distance,
	Endpoints,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ControlInterfacesControlTypeSerde")]
#[serde(try_from = "ControlInterfacesControlTypeSerde")]
pub enum ControlInterfacesControlType {
	#[doc = r#"Indicates commanded settings for mission control."#]
	MissionControl(crate::v2_5::types::MissionControlInterfacesCommandType),
	#[doc = r#"Indicates commanded settings for capability control.  Includes associated Capability ID and optional capability control interfaces and optional capability manager that is allowed to control this capability."#]
	CapabilityControl(Vec<crate::v2_5::types::ControlInterfacesCapabilityControlType>),
}
choice_convert_impls! {
	ControlInterfacesControlType - ControlInterfacesControlTypeSerde
	MissionControl,
	CapabilityControl,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ControlSourcesControlTypeSerde")]
#[serde(try_from = "ControlSourcesControlTypeSerde")]
pub enum ControlSourcesControlType {
	#[doc = r#"Indicates the unique ID of the System that has mission control."#]
	ControllerSystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the controllers that have Capability control."#]
	CapabilityControl(Vec<crate::v2_5::types::ControlSourcesCapabilityControlType>),
}
choice_convert_impls! {
	ControlSourcesControlType - ControlSourcesControlTypeSerde
	ControllerSystemId,
	CapabilityControl,
}

#[doc = r#"Choice indicating transfer of control, or the details of the new control status."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ControlTransferChoiceTypeSerde")]
#[serde(try_from = "ControlTransferChoiceTypeSerde")]
pub enum ControlTransferChoiceType {
	#[doc = r#"The mission control status that will be replacing the current control."#]
	InProgress(crate::v2_5::common::EmptyType),
	#[doc = r#"The mission control status that will be replacing the current control."#]
	NewMissionControl(crate::v2_5::types::ControlStatusMissionControlType),
}
choice_convert_impls! {
	ControlTransferChoiceType - ControlTransferChoiceTypeSerde
	InProgress,
	NewMissionControl,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CountryCodeTypeSerde")]
#[serde(try_from = "CountryCodeTypeSerde")]
pub enum CountryCodeType {
	#[doc = r#"Indicates the actual Country name from the GENC Standard.  It is important to note that FIPS PUB 10-4 and GEC are not explicitly supported; there will be no Country names that conform to either FIPS PUB 10-4 or GEC legacy standards."#]
	CountryName(crate::v2_5::enums::GencCountryNameEnum),
	#[doc = r#"Indicates a name for an affiliated asset reported by the Operator(s) that is outside the scope of the GENC Standard."#]
	OperatorUniqueAssetName(crate::v2_5::enums::OperatorUniqueNameEnum),
}
choice_convert_impls! {
	CountryCodeType - CountryCodeTypeSerde
	CountryName,
	OperatorUniqueAssetName,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CoverageAreaSpecificationChoiceTypeSerde")]
#[serde(try_from = "CoverageAreaSpecificationChoiceTypeSerde")]
pub enum CoverageAreaSpecificationChoiceType {
	#[doc = r#"Defines a region in space where the Capability is applicable."#]
	Region(crate::v2_5::types::ZoneType),
	#[doc = r#"Indicates the ProductID of the product that defines where the Capability Coverage Area is applicable."#]
	ProductId(Vec<crate::v2_5::types::ProductMetadataIdType>),
}
choice_convert_impls! {
	CoverageAreaSpecificationChoiceType - CoverageAreaSpecificationChoiceTypeSerde
	Region,
	ProductId,
}

#[doc = r#"This type represents the source of a key."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CryptoKeySourceTypeSerde")]
#[serde(try_from = "CryptoKeySourceTypeSerde")]
pub enum CryptoKeySourceType {
	#[doc = r#"Indicates a file to read the key from.  Would support either a red or black key file (usually black) depending on system design."#]
	KeyFileId(crate::v2_5::types::FileLocationIdType),
	#[doc = r#"Indicates to enable or disable DS101 load actions."#]
	Ds101(crate::v2_5::enums::CryptoDs101Enum),
	#[doc = r#"Indicates to initiate an over the air management action associated with the identified key.  Valid for those crypto capable capabilities that have receive / transmit capability."#]
	Otam(crate::v2_5::enums::CryptoOtamEnum),
}
choice_convert_impls! {
	CryptoKeySourceType - CryptoKeySourceTypeSerde
	KeyFileId,
	Ds101,
	Otam,
}

#[doc = r#"This type indicates the particular kind of CSO and provides additional details about the characteristics of the event."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "CsoDetailsTypeSerde")]
#[serde(try_from = "CsoDetailsTypeSerde")]
pub enum CsoDetailsType {
	#[doc = r#"Indicates CSO consistent with spacecraft docking. Involves the joining and mating of two separate vehicles or satellites. This also includes berthing of two separate spacecraft, where an interface (e.g. robotic arm)  is used to connect an inactive module/vehicle to an active module/vehicle."#]
	DockingEvent(crate::v2_5::types::OrbitalDockingEventType),
	#[doc = r#"Indicates CSO consistent with spacecraft undocking. Involves the disjoining and unmating of two separate vehicles or satellites."#]
	UndockingEvent(crate::v2_5::types::OrbitalUndockingEventType),
	#[doc = r#"Indicates CSO consistent with spacecraft separation. This includes a spacecraft separating from its flight vehicle (rocket). A separation event and undocking event are two different events."#]
	SeparationEvent(crate::v2_5::types::OrbitalSeparationEventType),
	#[doc = r#"Indicates CSO consistent with spacecraft rendezvous. This results in the orbit and position of a satellite closely matching those of another satellite."#]
	RendezvousEvent(crate::v2_5::types::OrbitalRendezvousEventType),
	#[doc = r#"Indicates CSO consistent with spacecraft proximity operations. This is a phase of a rendezvous maneuver in which a satellite approaches another satellite more closely."#]
	ProximityOperationsEvent(crate::v2_5::types::OrbitalProximityOperationsEventType),
}
choice_convert_impls! {
	CsoDetailsType - CsoDetailsTypeSerde
	DockingEvent,
	UndockingEvent,
	SeparationEvent,
	RendezvousEvent,
	ProximityOperationsEvent,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamageEstimateTargetTypeSerde")]
#[serde(try_from = "DamageEstimateTargetTypeSerde")]
pub enum DamageEstimateTargetType {
	#[doc = r#"Indicates the weaponeering to use for the estimate should come from an existing Task and existing DMPI associated with the Task."#]
	TaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates the weaponeering to use for the estimate should come from that matched to an existing target and existing DMPI associated with the target."#]
	Target(TargetType),
}
choice_convert_impls! {
	DamageEstimateTargetType - DamageEstimateTargetTypeSerde
	TaskId,
	Target,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamageObjectClassTypeSerde")]
#[serde(try_from = "DamageObjectClassTypeSerde")]
pub enum DamageObjectClassType {
	#[doc = r#"Indicates the object is a general platform or category."#]
	PlatformType(crate::v2_5::types::PlatformIdentityType),
	#[doc = r#"Indicates the object is a specific type of object."#]
	SpecificType(crate::v2_5::types::SpecificIdentityType),
	#[doc = r#"Indicates the object is a human."#]
	Human(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	DamageObjectClassType - DamageObjectClassTypeSerde
	PlatformType,
	SpecificType,
	Human,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamageSubjectTypeSerde")]
#[serde(try_from = "DamageSubjectTypeSerde")]
pub enum DamageSubjectType {
	#[doc = r#"Indicates unique ID of a Task associated with the damage."#]
	TaskId(Vec<crate::v2_5::types::TaskIdType>),
	#[doc = r#"Indicates the target, which was previously engaged, associated with the damage."#]
	Target(TargetType),
	#[doc = r#"Indicates the location associated with the damage."#]
	Point(crate::v2_5::types::Point2DType),
}
choice_convert_impls! {
	DamageSubjectType - DamageSubjectTypeSerde
	TaskId,
	Target,
	Point,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamageTypeSerde")]
#[serde(try_from = "DamageTypeSerde")]
pub enum DamageType {
	#[doc = r#"Details on that damage estimated for the ObjectType."#]
	FunctionalDamage(crate::v2_5::types::DamagedFunctionType),
	#[doc = r#"Information relating to people killed or injured."#]
	HumanCasualty(crate::v2_5::common::EmptyType),
	#[doc = r#"Information relating to people injured."#]
	HumanInjury(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	DamageType - DamageTypeSerde
	FunctionalDamage,
	HumanCasualty,
	HumanInjury,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamagedObjectIdentityTypeSerde")]
#[serde(try_from = "DamagedObjectIdentityTypeSerde")]
pub enum DamagedObjectIdentityType {
	#[doc = r#"Indicates a possible "platform identity" of the Entity."#]
	PlatformType(crate::v2_5::types::PlatformIdentityType),
	#[doc = r#"Indicates a possible "specific identity" of the Entity."#]
	SpecificType(crate::v2_5::types::SpecificIdentityType),
}
choice_convert_impls! {
	DamagedObjectIdentityType - DamagedObjectIdentityTypeSerde
	PlatformType,
	SpecificType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DamagedObjectTypeSerde")]
#[serde(try_from = "DamagedObjectTypeSerde")]
pub enum DamagedObjectType {
	#[doc = r#"The ID of the Entity damaged."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the type of damaged object (non-entity) and its location."#]
	NonEntity(crate::v2_5::types::DamagedObjectNonEntityType),
}
choice_convert_impls! {
	DamagedObjectType - DamagedObjectTypeSerde
	EntityId,
	NonEntity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DataDeleteChoiceTypeSerde")]
#[serde(try_from = "DataDeleteChoiceTypeSerde")]
pub enum DataDeleteChoiceType {
	#[doc = r#"This element represents the unique identifier of the ProductMetadata associated with the product to be deleted.  All applicable instances of the product file, all applicable instances of the ProductMetadata message/object and associated instances of the ProductLocation message/object should be deleted."#]
	ProductMetadataId(crate::v2_5::types::ProductMetadataIdType),
	#[doc = r#"This element represents the unique identifier of the ProductLocation associated with the product to be deleted.  The single instance of the product file and all applicable instances of the ProductLocation message/object should be deleted."#]
	ProductLocationId(crate::v2_5::types::ProductLocationIdType),
	#[doc = r#"This element represents the unique identifier of the FileMetadata associated with the file to be deleted.  All applicable instances of the file, all applicable instances of the FileMetadata message/object and associated instances of the FileLocation message/object should be deleted."#]
	FileMetadataId(crate::v2_5::types::FileMetadataIdType),
	#[doc = r#"This element represents the unique identifier of the FileLocation associated with the file to be deleted.  The single instance of the file and all applicable instances of the FileLocation message/object should be deleted."#]
	FileLocationId(crate::v2_5::types::FileLocationIdType),
	#[doc = r#"Indicates a specific Capability whose associated data is to be deleted."#]
	CapabilityId(Vec<crate::v2_5::types::CapabilityIdType>),
	#[doc = r#"Indicates a specific Component whose associated data is to be deleted."#]
	ComponentId(Vec<crate::v2_5::types::ComponentIdType>),
}
choice_convert_impls! {
	DataDeleteChoiceType - DataDeleteChoiceTypeSerde
	ProductMetadataId,
	ProductLocationId,
	FileMetadataId,
	FileLocationId,
	CapabilityId,
	ComponentId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DataManagementCategoryTypeSerde")]
#[serde(try_from = "DataManagementCategoryTypeSerde")]
pub enum DataManagementCategoryType {
	#[doc = r#"Indicates a request to save a mission.  This command tells the data persistence services within a system to persist all data associated with the specified mission session."#]
	SaveMissionId(crate::v2_5::types::MissionIdType),
	#[doc = r#"Indicates a request to save a mission.   This command tells the data persistence services within a system to persist all data associated with the specified mission session to a new MissionID and/or MissionVersion.  This allows an operator to create a "save point" and continue with planning in the current session."#]
	SaveAs(crate::v2_5::types::DataManagementSaveAsType),
	#[doc = r#"This command tells the data sources and data persistence services within a system to delete all data associated with the specified data type."#]
	DeleteMissionId(crate::v2_5::types::MissionIdType),
	#[doc = r#"This command tells the data persistence services within a system to import data from a specified URI into the specified mission session.  For example, if data was being loaded from removable media."#]
	ImportData(crate::v2_5::types::DataManagementImportExportType),
	#[doc = r#"This command tells the data persistence services within a system to export data from a specified URI into the specified mission session.  For example, if data was being written to removable media.  This could be used to transfer data from pre-mission planning to operations."#]
	ExportData(crate::v2_5::types::DataManagementImportExportType),
}
choice_convert_impls! {
	DataManagementCategoryType - DataManagementCategoryTypeSerde
	SaveMissionId,
	SaveAs,
	DeleteMissionId,
	ImportData,
	ExportData,
}

#[doc = r#"Identifies the destination data port through which this message will be transmitted. This specifies the specific location the data shall transition through."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DataPortTypeSerde")]
#[serde(try_from = "DataPortTypeSerde")]
pub enum DataPortType {
	#[doc = r#"Identifies an input/output port used to inject or withdraw data to/from an I/O component.  Where the subsystem provides RED/BLACK separation, UserIO components exist on the RED side of the communication subsystem."#]
	UserIoId(crate::v2_5::types::CommUserIoIdType),
	#[doc = r#"Identifies an input/output RF link used to send or receive payload data.  Where the subsystem provides RED/BLACK separation, user link components exist on the BLACK side of the communication subsystem."#]
	RfUserLinkId(crate::v2_5::types::CommUserLinkIdType),
	#[doc = r#"Identifies input or output port of a crypto component.  Where the subsystem provides RED/BLACK separation, crypto components straddle the boundary between RED and BLACK sides of the communication subsystem."#]
	CryptoId(crate::v2_5::types::SupportCapabilityIdType),
}
choice_convert_impls! {
	DataPortType - DataPortTypeSerde
	UserIoId,
	RfUserLinkId,
	CryptoId,
}

#[doc = r#"Description of the data producer or producers that are expected to respond to a data update request."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DataUpdateOriginatorTypeSerde")]
#[serde(try_from = "DataUpdateOriginatorTypeSerde")]
pub enum DataUpdateOriginatorType {
	#[doc = r#"Request data from all producers of the requested data."#]
	AllProducers(crate::v2_5::common::EmptyType),
	#[doc = r#"Request data from the last producer of the requested object."#]
	Source(crate::v2_5::types::DataUpdateSourceType),
}
choice_convert_impls! {
	DataUpdateOriginatorType - DataUpdateOriginatorTypeSerde
	AllProducers,
	Source,
}

#[doc = r#"Parameters describing the specific kind of data that is being requested."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DataUpdateRequestTypeSerde")]
#[serde(try_from = "DataUpdateRequestTypeSerde")]
pub enum DataUpdateRequestType {
	#[doc = r#"Indicates the type of message data being requested and filter parameters applicable.  The QueryMessageType here consists of an optional list of MessageTypes to limit the query and an abstract polymorphic extension point/element; the QueryPET (PolymorphicExtensionType) used here is an abstract base type as a polymorphic extension point consisting of number of extended types."#]
	Query(crate::v2_5::types::QueryMessageType),
	#[doc = r#"Extension point to allow for application specific queries that cannot be specified using QueryPETs."#]
	QuerySpecificData(crate::v2_5::types::QuerySpecificDataPet),
}
choice_convert_impls! {
	DataUpdateRequestType - DataUpdateRequestTypeSerde
	Query,
	QuerySpecificData,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DeCommandTypeSerde")]
#[serde(try_from = "DeCommandTypeSerde")]
pub enum DeCommandType {
	#[doc = r#"Indicates a new invocation of a DE Capability. Generally, if accepted, the command will result in one or more new DE Activities being created and reported via the DE_Activity message. The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command. Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::DeCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing DE Activity (which was previously reported via the DE_Activity message and was marked as "interactive"). The request/response interaction terminates as soon as the modification is accepted or rejected. The modifications are reflected in subsequent DE_Activity messages."#]
	Activity(crate::v2_5::types::DeActivityCommandType),
}
choice_convert_impls! {
	DeCommandType - DeCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"Identifier of a Digital Payload or a MutiFunctionArray (MFA)."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DigitalFunctionTypeSerde")]
#[serde(try_from = "DigitalFunctionTypeSerde")]
pub enum DigitalFunctionType {
	#[doc = r#"The CapabilityID of a Digital Payload function operating within a Multi-Function Payload."#]
	DigitalPayloadCapabilityId(crate::v2_5::types::CapabilityIdType),
	#[doc = r#"The SupportCapabilityID of a MultiFunctionArray (MFA); effectively, the "AntennaID" of the MFA."#]
	MfaSupportCapabilityId(crate::v2_5::types::SupportCapabilityIdType),
}
choice_convert_impls! {
	DigitalFunctionType - DigitalFunctionTypeSerde
	DigitalPayloadCapabilityId,
	MfaSupportCapabilityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DisseminationProductOrFileTypeSerde")]
#[serde(try_from = "DisseminationProductOrFileTypeSerde")]
pub enum DisseminationProductOrFileType {
	#[doc = r#"Identifies a specific product instance."#]
	ProductByReference(ProductReferenceType),
	#[doc = r#"Identifies a specific file instance."#]
	FileByReference(FileReferenceType),
	#[doc = r#"Specifies a product or file type.  This applies more to streaming products which stream continuously and are therefore classified continuously.  For example, EOIR streaming video."#]
	ByType(crate::v2_5::types::DisseminationByType),
}
choice_convert_impls! {
	DisseminationProductOrFileType - DisseminationProductOrFileTypeSerde
	ProductByReference,
	FileByReference,
	ByType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DmpiPatternChoiceTypeSerde")]
#[serde(try_from = "DmpiPatternChoiceTypeSerde")]
pub enum DmpiPatternChoiceType {
	#[doc = r#"Indicates a rectangular X by Y DMPI pattern; a pattern of rows and columns with a regular grid spacing.  The columns are oriented parallel to the PatternOrientation.  The DMPI IDs listed correspond to the pattern beginning at the top of the leftmost column, proceeding right across the first row and then on to the leftmost column of the second row and so forth."#]
	XbyYPattern(crate::v2_5::types::DmpiXbyYPatternType),
	#[doc = r#"A radial pattern is a wheel-and-spoke pattern.  The first spoke/radial originates at the PatternLocationCenter in the direction given by PatternOrientation.  The DMPI IDs listed correspond to the pattern beginning at the centermost DMPI on the first radial then proceeding clockwise to the centermost DMPI on the next radial and subsequent radials followed by the next outermost DMPI of the first radial and so on."#]
	RadialPattern(crate::v2_5::types::DmpiRadialPatternType),
	#[doc = r#"Indicates a free pattern used to group DMPIs that can't be defined as an X by Y or radial pattern."#]
	FreePatternDmpiId(Vec<crate::v2_5::types::DmpiIdType>),
}
choice_convert_impls! {
	DmpiPatternChoiceType - DmpiPatternChoiceTypeSerde
	XbyYPattern,
	RadialPattern,
	FreePatternDmpiId,
}

#[doc = r#"Indicates the target of the DMPI. The target can be specified by location or by identity. This allows DMPI targets to be specified based on target types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DmpiTargetTypeSerde")]
#[serde(try_from = "DmpiTargetTypeSerde")]
pub enum DmpiTargetType {
	#[doc = r#"Indicates the geographic location and location error for the DMPI."#]
	ByLocation(crate::v2_5::types::DmpiLocationType),
	#[doc = r#"Indicates the target of the DMPI in the form of an Identity."#]
	ByIdentity(crate::v2_5::types::IdentityType),
}
choice_convert_impls! {
	DmpiTargetType - DmpiTargetTypeSerde
	ByLocation,
	ByIdentity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DmpiViolationChoiceTypeSerde")]
#[serde(try_from = "DmpiViolationChoiceTypeSerde")]
pub enum DmpiViolationChoiceType {
	#[doc = r#"The air corridor violated by a line between the release point and the indicated DMPI."#]
	OpLineId(crate::v2_5::types::OpLineIdType),
	#[doc = r#"The NoFire OpZone violated by the minimum safe distance (blast radius) around the indicated DMPI."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"The NoFire OpVolume violated by the minimum safe distance (blast radius) around the indicated DMPI."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
}
choice_convert_impls! {
	DmpiViolationChoiceType - DmpiViolationChoiceTypeSerde
	OpLineId,
	OpZoneId,
	OpVolumeId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DoorCommandChoiceTypeSerde")]
#[serde(try_from = "DoorCommandChoiceTypeSerde")]
pub enum DoorCommandChoiceType {
	#[doc = r#"Command the door to open or close.  In some systems this may not be required and the stores management system will automatically open the door when a release is commanded."#]
	DoorState(crate::v2_5::enums::DoorCommandEnum),
	#[doc = r#"This setting is used with systems that automatically open and close the bay door as part of a store release command.  When a release fails and there is a hung store, this setting dictates whether the door should be left open or forced closed."#]
	CloseOnHungStore(bool),
}
choice_convert_impls! {
	DoorCommandChoiceType - DoorCommandChoiceTypeSerde
	DoorState,
	CloseOnHungStore,
}

#[doc = r#"A choice of drag coefficient to use. It is a choice between a simple drag coefficient and VCM drag parameters."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "DragCoefficientChoiceTypeSerde")]
#[serde(try_from = "DragCoefficientChoiceTypeSerde")]
pub enum DragCoefficientChoiceType {
	#[doc = r#"A set of drag parameters used for VCM propagation."#]
	VcmDragParameters(crate::v2_5::types::VcmDragParametersType),
	#[doc = r#"A dimensionless value based on the shape of the RSO that is used to help quantify the drag force on the RSO."#]
	DragCoefficientValue(f64),
}
choice_convert_impls! {
	DragCoefficientChoiceType - DragCoefficientChoiceTypeSerde
	VcmDragParameters,
	DragCoefficientValue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaCommandTypeSerde")]
#[serde(try_from = "EaCommandTypeSerde")]
pub enum EaCommandType {
	#[doc = r#"Indicates a new invocation of an EA Capability.  Generally, if accepted, the command will result in one or more new EA Activities being created and reported via the EA_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::EaCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing EA Activity (which was previously reported via the EA_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent EA_Activity messages."#]
	Activity(crate::v2_5::types::EaActivityCommandType),
}
choice_convert_impls! {
	EaCommandType - EaCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaDetailsTypeSerde")]
#[serde(try_from = "EaDetailsTypeSerde")]
pub enum EaDetailsType {
	#[doc = r#"Indicates current jamming being performed by a specific EA Component."#]
	Assigned(crate::v2_5::types::EaDetailsAssignedType),
	#[doc = r#"Indicates jamming Activity that hasn't been assigned to a Component."#]
	Unassigned(crate::v2_5::types::EaDetailsUnassignedType),
}
choice_convert_impls! {
	EaDetailsType - EaDetailsTypeSerde
	Assigned,
	Unassigned,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaEmitterDataTypeSerde")]
#[serde(try_from = "EaEmitterDataTypeSerde")]
pub enum EaEmitterDataType {
	#[doc = r#"Indicates the target is being attacked according to Signal/emitter data from a corresponding Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the target is being attacked according to Signal/emitter data associated with its emitter type."#]
	EmitterType(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"Indicates the target is being attacked according to specifically commanded Signal data."#]
	SignalDescription(crate::v2_5::types::SignalSummaryType),
	#[doc = r#"Indicates the target is being attacked according to a previously detected Signal."#]
	SignalId(crate::v2_5::types::SignalIdType),
}
choice_convert_impls! {
	EaEmitterDataType - EaEmitterDataTypeSerde
	EntityId,
	EmitterType,
	SignalDescription,
	SignalId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaPowerTypeSerde")]
#[serde(try_from = "EaPowerTypeSerde")]
pub enum EaPowerType {
	#[doc = r#"Power at target In dBW. This will use the center point for calculation if an area or line is selected."#]
	PowerAtTarget(crate::v2_5::types::DecibelRangeType),
	#[doc = r#"Effective Radiate Power in dBW. It is an IEEE standardized definition of directional radio frequency (RF) power transmitted from a theoretical half-wave dipole antenna."#]
	Erp(crate::v2_5::types::DecibelRangeType),
	#[doc = r#"Jam to Signal ratio In dB. The ratio of the signal strength of the jamming signal (J) to the signal strength of the target return signal (S)."#]
	JtoS(crate::v2_5::types::DecibelRangeType),
}
choice_convert_impls! {
	EaPowerType - EaPowerTypeSerde
	PowerAtTarget,
	Erp,
	JtoS,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaProposedTypeSerde")]
#[serde(try_from = "EaProposedTypeSerde")]
pub enum EaProposedType {
	#[doc = r#"Indicates the ActivityID of the EA_Activity which describes the proposed EA."#]
	ActivityId(crate::v2_5::types::ActivityIdType),
	#[doc = r#"Indicates the EntityID and characteristics of the proposed EA activity."#]
	Entity(crate::v2_5::types::EaEntityType),
}
choice_convert_impls! {
	EaProposedType - EaProposedTypeSerde
	ActivityId,
	Entity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaTargetPointingTypeSerde")]
#[serde(try_from = "EaTargetPointingTypeSerde")]
pub enum EaTargetPointingType {
	#[doc = r#"Indicates the source of or explicit values for geospatial characteristics of the EA target."#]
	LocationData(TargetType),
	#[doc = r#"Indicates the pointing volume for the EA transmission."#]
	AirVolume(crate::v2_5::types::AirVolumeSensorReferencedType),
}
choice_convert_impls! {
	EaTargetPointingType - EaTargetPointingTypeSerde
	LocationData,
	AirVolume,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaTaskRouteRequirementsTypeSerde")]
#[serde(try_from = "EaTaskRouteRequirementsTypeSerde")]
pub enum EaTaskRouteRequirementsType {
	#[doc = r#"This element specifies specific waypoints that the vehicle being tasked should fly."#]
	Path(crate::v2_5::types::PathType),
	#[doc = r#"This element defines the loiter location and pattern from which the tasked vehicle should provide protection."#]
	Loiter(LoiterType),
	#[doc = r#"This element specifies a task to escort an asset.  The tasked vehicle shall provide protection for the escorted asset.  Additional protected assets can be specified in the ProtectedAssets element."#]
	Escort(crate::v2_5::types::EaTaskEscortType),
	#[doc = r#"This specifies the geospatial location zone for the vehicle to execute the EA Task."#]
	ZoneConstraints(ZoneChoiceType),
	#[doc = r#"This specifies the geospatial location volume for the vehicle to execute the EA Task."#]
	VolumeConstraints(VolumeChoiceType),
}
choice_convert_impls! {
	EaTaskRouteRequirementsType - EaTaskRouteRequirementsTypeSerde
	Path,
	Loiter,
	Escort,
	ZoneConstraints,
	VolumeConstraints,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EaTaskThreatsTypeSerde")]
#[serde(try_from = "EaTaskThreatsTypeSerde")]
pub enum EaTaskThreatsType {
	#[doc = r#"If selected, this element implies that all known threats should be jammed to the extent resources are available. Child elements indicate optional additional suppression constraints. If no child elements are provided, suppression is unconstrained."#]
	SuppressAll(crate::v2_5::types::EaTaskSuppressAllType),
	#[doc = r#"This element defines specific constraints on the threats or frequencies to suppress. This structure provides the flexibility to define multiple target sets with different values of priority, activation, and/or technique identifier."#]
	SuppressBySelection(Vec<crate::v2_5::types::EaTaskSuppressionConstraintsType>),
}
choice_convert_impls! {
	EaTaskThreatsType - EaTaskThreatsTypeSerde
	SuppressAll,
	SuppressBySelection,
}

#[doc = r#"Defines the type that allows a choice of Earth Orientation Parameters data type: EarthOrientatonParameters message or static values."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EarthOrientationParametersDataChoiceTypeSerde")]
#[serde(try_from = "EarthOrientationParametersDataChoiceTypeSerde")]
pub enum EarthOrientationParametersDataChoiceType {
	#[doc = r#"Indicates the ID of the set of EarthOrientatonParameters to use."#]
	EarthOrientationParametersId(crate::v2_5::types::EarthOrientationParametersIdType),
	#[doc = r#"The static (non-changing) time and polar motion data to use."#]
	StaticValues(crate::v2_5::types::TimeAndPolarDataType),
}
choice_convert_impls! {
	EarthOrientationParametersDataChoiceType - EarthOrientationParametersDataChoiceTypeSerde
	EarthOrientationParametersId,
	StaticValues,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EffectCommandTypeSerde")]
#[serde(try_from = "EffectCommandTypeSerde")]
pub enum EffectCommandType {
	#[doc = r#"Indicates a new invocation of an Effect Capability.  Generally, if accepted, the command will result in one or more new Effect Activities being created and reported via the EffectActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::EffectCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Effect Activity (which was previously reported via the EffectActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent EffectActivity messages."#]
	Activity(crate::v2_5::types::ActivityCommandBaseType),
}
choice_convert_impls! {
	EffectCommandType - EffectCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EffectPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "EffectPlanCommandIdChoiceTypeSerde")]
pub enum EffectPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the EffectPlanCommand associated with the EffectPlan."#]
	EffectPlanCommandId(crate::v2_5::types::EffectPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the EffectPlanValidationCommand associated with the EffectPlan."#]
	EffectPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the EffectPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the EffectPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	EffectPlanCommandIdChoiceType - EffectPlanCommandIdChoiceTypeSerde
	EffectPlanCommandId,
	EffectPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EffectiveForTypeSerde")]
#[serde(try_from = "EffectiveForTypeSerde")]
pub enum EffectiveForType {
	#[doc = r#"One or more systems for which this is effective."#]
	SystemId(Vec<crate::v2_5::types::SystemIdType>),
	#[doc = r#"One or more platforms for which this is effective."#]
	Platform(Vec<crate::v2_5::types::PlatformIdentityType>),
	#[doc = r#"One or more specific identities for which this is effective."#]
	Specific(Vec<crate::v2_5::types::SpecificIdentityType>),
	#[doc = r#"One or more models for which this is effective."#]
	Model(Vec<crate::v2_5::common::VisibleString32Type>),
}
choice_convert_impls! {
	EffectiveForType - EffectiveForTypeSerde
	SystemId,
	Platform,
	Specific,
	Model,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmconErpTypeSerde")]
#[serde(try_from = "EmconErpTypeSerde")]
pub enum EmconErpType {
	#[doc = r#"This value determines the maximum absolute radiated effective radiated power (ERP) at the face of the array to be used for this command. This field can be used to limit output during testing to comply with RF Licensing requirements."#]
	MaximumRadiatedErp(crate::v2_5::common::MilliwattPowerRatioType),
	#[doc = r#"Specifies that the command should radiate at full power. Use of this flag may cause degraded performance due to saturation of the receiver."#]
	RadiateFullPower(bool),
}
choice_convert_impls! {
	EmconErpType - EmconErpTypeSerde
	MaximumRadiatedErp,
	RadiateFullPower,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmconOverrideTypeSerde")]
#[serde(try_from = "EmconOverrideTypeSerde")]
pub enum EmconOverrideType {
	#[doc = r#"Specifies the emission control level to be used for this command."#]
	EmconLevel(crate::v2_5::enums::EmconLevelEnum),
	#[doc = r#"If a service does not use a normative EmconLevel, foreign keys may be used to specify the level."#]
	ForeignLevel(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	EmconOverrideType - EmconOverrideTypeSerde
	EmconLevel,
	ForeignLevel,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmconSettingTypeSerde")]
#[serde(try_from = "EmconSettingTypeSerde")]
pub enum EmconSettingType {
	#[doc = r#"Specifies the emission control level to be used for this capability."#]
	EmconLevel(crate::v2_5::enums::EmconLevelEnum),
	#[doc = r#"If a service does not use a normative EmconLevel, foreign keys may be used to specify the level."#]
	ForeignLevel(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	EmconSettingType - EmconSettingTypeSerde
	EmconLevel,
	ForeignLevel,
}

#[doc = r#"Container object for the different types of OpPoint*Enums, excluding Emergency.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmergencyReferenceOpPointCategoriesTypeSerde")]
#[serde(try_from = "EmergencyReferenceOpPointCategoriesTypeSerde")]
pub enum EmergencyReferenceOpPointCategoriesType {
	#[doc = r#"A set of commonly used point types."#]
	General(crate::v2_5::enums::OpPointGeneralEnum),
	#[doc = r#"A set of navigation or weapon hazard point types."#]
	Hazard(crate::v2_5::enums::OpPointHazardEnum),
	#[doc = r#"A set of navigation destination point types."#]
	Reference(crate::v2_5::enums::OpPointReferenceEnum),
	#[doc = r#"A set of navigation station point types."#]
	Station(crate::v2_5::enums::OpPointStationEnum),
}
choice_convert_impls! {
	EmergencyReferenceOpPointCategoriesType - EmergencyReferenceOpPointCategoriesTypeSerde
	General,
	Hazard,
	Reference,
	Station,
}

#[doc = r#"Specify an emitter by ID or by MDF_Entry number."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterEntryTypeSerde")]
#[serde(try_from = "EmitterEntryTypeSerde")]
pub enum EmitterEntryType {
	#[doc = r#"One or more emitters, defined by emitter IDs, may be the subject of the message."#]
	Emitter(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"One or more emitters, defined by an MDF_Entry key, may be the subject of the message."#]
	MdfEntry(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	EmitterEntryType - EmitterEntryTypeSerde
	Emitter,
	MdfEntry,
}

#[doc = r#"Indicates the emitter identification based on its category."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterIdentityCategoryTypeSerde")]
#[serde(try_from = "EmitterIdentityCategoryTypeSerde")]
pub enum EmitterIdentityCategoryType {
	#[doc = r#"Used for emitters identified as a Radar subsystem."#]
	Radar(crate::v2_5::types::RadarEmitterIdentityType),
	#[doc = r#"Used for emitters identified as a communications subsystem."#]
	Communications(crate::v2_5::types::CommunicationsEmitterIdentityType),
	#[doc = r#"Used for emitters identified as an Electronic Attack subsystem."#]
	Jammer(crate::v2_5::types::JammerEmitterIdentityType),
	#[doc = r#"Used for emitters identified as a Missile subsystem."#]
	Missile(crate::v2_5::types::MissileEmitterIdentityType),
}
choice_convert_impls! {
	EmitterIdentityCategoryType - EmitterIdentityCategoryTypeSerde
	Radar,
	Communications,
	Jammer,
	Missile,
}

#[doc = r#"Entity ID or Local Track ID of the emitter used to detect targets passively."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterSourceIdChoiceTypeSerde")]
#[serde(try_from = "EmitterSourceIdChoiceTypeSerde")]
pub enum EmitterSourceIdChoiceType {
	#[doc = r#"Indicates the Entity ID of the emitter used to detect targets passively."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the Measurement ID within the OMR of the emitter used to detect targets passively."#]
	OmrIndividualMeasurementId(crate::v2_5::types::MeasurementIdType),
	#[doc = r#"Indicates the unique ID of the corresponding OB emitter instance."#]
	EmitterId(crate::v2_5::types::EmitterRecordIdType),
	#[doc = r#"Indicates the unique ID of the corresponding Signal for the emitter instance."#]
	SignalId(crate::v2_5::types::SignalIdType),
}
choice_convert_impls! {
	EmitterSourceIdChoiceType - EmitterSourceIdChoiceTypeSerde
	EntityId,
	OmrIndividualMeasurementId,
	EmitterId,
	SignalId,
}

#[doc = r#"Source emitter location. Used if Waveform does not contain location."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterTargetLocationDataTypeSerde")]
#[serde(try_from = "EmitterTargetLocationDataTypeSerde")]
pub enum EmitterTargetLocationDataType {
	#[doc = r#"Indicates the Source ID of the emitter used to detect targets passively. Location can be referenced through these."#]
	EmitterSourceId(EmitterSourceIdChoiceType),
	#[doc = r#"Indicates the Field of View details associated with the source emitter."#]
	Fov(crate::v2_5::types::FovVolumeType),
	#[doc = r#"Indicates the Point Target details associated with the source emitter."#]
	PointTarget(crate::v2_5::types::PointTargetType),
}
choice_convert_impls! {
	EmitterTargetLocationDataType - EmitterTargetLocationDataTypeSerde
	EmitterSourceId,
	Fov,
	PointTarget,
}

#[doc = r#"An ellipse or rectangle shape describing 1-sigma position uncertainty."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterUncertaintyChoiceTypeSerde")]
#[serde(try_from = "EmitterUncertaintyChoiceTypeSerde")]
pub enum EmitterUncertaintyChoiceType {
	#[doc = r#"This element represents an ellipse describing the 1-sigma position uncertainty.  The ellipse is defined by the length of its semi-major and semi-minor axes.  The orientation of the ellipse defines the angle between the semi-major axis and true north."#]
	UncertaintyEllipse(crate::v2_5::types::EllipseType),
	#[doc = r#"This element represents a rectangle describing the 1-sigma position uncertainty.  The rectangle is defined by the height parallel to the orientation axis and width of the rectangle perpendicular to the orientation angle."#]
	UncertaintyRectangle(crate::v2_5::types::RectangleType),
}
choice_convert_impls! {
	EmitterUncertaintyChoiceType - EmitterUncertaintyChoiceTypeSerde
	UncertaintyEllipse,
	UncertaintyRectangle,
}

#[doc = r#"Indicates the source of or explicit values for emitter characteristics of the emitter used as a source for passive detection."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EmitterWaveformDataTypeSerde")]
#[serde(try_from = "EmitterWaveformDataTypeSerde")]
pub enum EmitterWaveformDataType {
	#[doc = r#"EntityID of the emitter used to detect targets passively."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"EmitterType of the emitter used to detect targets passively."#]
	EmitterType(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"SpecificEmitter of the emitter used to detect targets passively."#]
	SpecificEmitter(crate::v2_5::types::SpecificEmitterIdentityType),
	#[doc = r#"SignalDescription of the emitter used to detect targets passively."#]
	SignalDescription(crate::v2_5::types::SignalParametricsType),
	#[doc = r#"SignalID of the emitter used to detect targets passively."#]
	SignalId(crate::v2_5::types::SignalIdType),
	#[doc = r#"EmitterModeID of the emitter used to detect targets passively."#]
	EmitterModeId(crate::v2_5::types::EmitterModeIdType),
}
choice_convert_impls! {
	EmitterWaveformDataType - EmitterWaveformDataTypeSerde
	EntityId,
	EmitterType,
	SpecificEmitter,
	SignalDescription,
	SignalId,
	EmitterModeId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EndPointTypeSerde")]
#[serde(try_from = "EndPointTypeSerde")]
pub enum EndPointType {
	#[doc = r#"This element is used to specify a point in the route where no turning occurs.  It is generally used for points associated with Actions such as task "transitions", changes in acceleration, changes in speed, changes in altitude, etc."#]
	WayPoint(crate::v2_5::types::WayPointType),
	#[doc = r#"This element is used to specify turns in a segment where a high fidelity (i.e., high precision and/or complex curvature) description of the turn trajectory is not required. This includes standard fly-through and turn-short scenarios. It can additionally be associated with tasks/actions that require or are coincident with turns. TurnPoints will generally come in groups of at least two in linked PathSegments. All but the last endpoint in a group will include a TurnRadius."#]
	TurnPoint(crate::v2_5::types::TurnPointType),
	#[doc = r#"This element specifies a Loiter.  Currently specifying a task/action associated with a LoiterPoint when the task/action occurs only during a portion of the Loiter is not supported."#]
	LoiterPoint(crate::v2_5::types::LoiterPointType),
	#[doc = r#"This element is used to specify a Non-Uniform Rational Basis Spline (NURBS) representation of the trajectory the platform should use for this path segment. When NURBS is the selected description, the following constraints should be observed: A) A third-order curve should be sufficient to accurately describe the desired trajectory, B) The curve description should be terminated at both endpoints C) The curve should be continuous to first and second order across the path segments for the active route."#]
	NurbsPoint(crate::v2_5::types::NurbsPointType),
}
choice_convert_impls! {
	EndPointType - EndPointTypeSerde
	WayPoint,
	TurnPoint,
	LoiterPoint,
	NurbsPoint,
}

#[doc = r#"Indicates endurance in terms of the domain specific choice."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EnduranceMultiStandardTypeSerde")]
#[serde(try_from = "EnduranceMultiStandardTypeSerde")]
pub enum EnduranceMultiStandardType {
	#[doc = r#"Indicates endurance in terms of maximum ground/surface distance that can be reached given the current system state."#]
	EnduranceFootprint(crate::v2_5::types::EnduranceType),
	#[doc = r#"Logical comparator to use when comparing the current endurance remaining to the sibling EnduranceRemaining element.  The logical expression is (current endurance remaining) (LogicalOperator) (sibling EnduranceRemaining).  When the logical expression is TRUE then the parent Endurance Condition is TRUE."#]
	SatelliteEndurance(crate::v2_5::types::SatelliteEnduranceType),
}
choice_convert_impls! {
	EnduranceMultiStandardType - EnduranceMultiStandardTypeSerde
	EnduranceFootprint,
	SatelliteEndurance,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityCharacteristicTypeSerde")]
#[serde(try_from = "EntityCharacteristicTypeSerde")]
pub enum EntityCharacteristicType {
	#[doc = r#"Indicates the identity to be compared to the identity in the Entity message."#]
	Identity(crate::v2_5::types::IdentityComparisonType),
	#[doc = r#"Indicates the identity staleness to be compared to the identity staleness in the Entity message.  This choice requires a corresponding Identity TargetClause."#]
	IdentityStaleness(chrono::TimeDelta),
	#[doc = r#"Indicates the position uncertainty to be compared to the position uncertainty in the Entity message.  This element represents the positional accuracy of the target.  It is the area, in square data miles, within which it is assessed that there is a 95% probability that the target lies.  The value given in this element can be compared to either the Quality element of the target or a calculated uncertainty area for the target based on its uncertainty ellipse."#]
	PositionUncertainty(f32),
	#[doc = r#"Indicates the position staleness to be compared to the kinematic staleness in the Entity message."#]
	PositionStaleness(chrono::TimeDelta),
	#[doc = r#"Indicates a value priority/rank  to be compared to the priority/rank in PrioritizationList referencing the Entity."#]
	PrioritizationList(crate::v2_5::types::PrioritizationListValueType),
	#[doc = r#"Indicates behaviors, activities, use of capabilities, etc. to be compared to those of the battlespace object associated with the Entity."#]
	Behavior(crate::v2_5::types::BehaviorType),
}
choice_convert_impls! {
	EntityCharacteristicType - EntityCharacteristicTypeSerde
	Identity,
	IdentityStaleness,
	PositionUncertainty,
	PositionStaleness,
	PrioritizationList,
	Behavior,
}

#[doc = r#"Indicates the contributors to the fused entity.  This type allows specifying non-Entity contributors if a fusion service supports this functionality."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityContributorIdChoiceTypeSerde")]
#[serde(try_from = "EntityContributorIdChoiceTypeSerde")]
pub enum EntityContributorIdChoiceType {
	#[doc = r#"Indicates the ID of a fusion input Entity that is a contributor to this fusion output Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the ID of a fusion input OB emitter that is a contributor to this fusion output Entity."#]
	EmitterId(crate::v2_5::types::EmitterRecordIdType),
	#[doc = r#"Indicates the ID of a fusion input System that is a contributor to this fusion output Entity.  The use case would be when PPLI data is received as a J2.0, the translation of that to UCI results in a set of messages that can be referenced by a SystemID_Type.  This System can be a contributor to a Fused Entity.  The entity originator could be System (especially since PPLI is usually self-reported), in which case the Fusion.SystemID would be the same as this element, or they could be different if another platform was reporting on the System's behalf."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the ID of a fusion input Signal that is a contributor to this fusion output Entity."#]
	SignalId(crate::v2_5::types::SignalIdType),
	#[doc = r#"Indicates the ID of a fusion input OB Record that is a contributor to this fusion output Entity."#]
	RecordId(crate::v2_5::types::RecordIdType),
	#[doc = r#"Indicates the ID of a fusion input Measurement that is a contributor to this fusion output Entity."#]
	MeasurementId(crate::v2_5::types::MeasurementIdType),
	#[doc = r#"Indicates the ID of a fusion input IFF Measurement that is a contributor to this fusion output Entity."#]
	IffMeasurementId(crate::v2_5::types::IffMeasurementIdType),
	#[doc = r#"Indicates the ID of a fusion input IFF Measurement And Data that is a contributor to this fusion output Entity.  This choice is used when the IFF Reported Data  (for example reported altitude, position, velocity) is used to update position or identity information (for example reported mode codes/PIN/NO/AA, emergency status, response to Combat ID)."#]
	IffMeasurementAndDataId(crate::v2_5::types::IffMeasurementAndDataMessageIdType),
}
choice_convert_impls! {
	EntityContributorIdChoiceType - EntityContributorIdChoiceTypeSerde
	EntityId,
	EmitterId,
	SystemId,
	SignalId,
	RecordId,
	MeasurementId,
	IffMeasurementId,
	IffMeasurementAndDataId,
}

#[doc = r#"Indicates whether an ElementSet, EntityElementSetID, KinematicVector, EntityVCM_ID, or OrbitPlanID will be used to determine the ephemeris."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityEphemerisBasisChoiceTypeSerde")]
#[serde(try_from = "EntityEphemerisBasisChoiceTypeSerde")]
pub enum EntityEphemerisBasisChoiceType {
	#[doc = r#"Indicates the default or "catalog" orbital element set (also known as two line element or TLE) for the satellite."#]
	ElementSet(crate::v2_5::types::TleBaseType),
	#[doc = r#"The ID type for UCI IDs that correspond to an orbital element set (TLE)."#]
	EntityElementSetId(crate::v2_5::types::EntityOrbitalElementSetIdType),
	#[doc = r#"Indicates kinematics expressed in terms of a standard frame centered on a celestial object."#]
	KinematicVector(OrbitalKinematicsStandardFrameChoiceType),
	#[doc = r#"Propagate using a complete VCM (vector covariance message) which includes a kinematic vector, along with other special perturbation parameters."#]
	EntityVcmId(crate::v2_5::types::EntityOrbitalVcmIdType),
	#[doc = r#"Specifies the Orbit Plan ID and other Orbit Plan data that the ephemeris should be based on."#]
	OrbitPlan(crate::v2_5::types::OrbitPlanReferenceDetailsType),
}
choice_convert_impls! {
	EntityEphemerisBasisChoiceType - EntityEphemerisBasisChoiceTypeSerde
	ElementSet,
	EntityElementSetId,
	KinematicVector,
	EntityVcmId,
	OrbitPlan,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityIdentityChoiceTypeSerde")]
#[serde(try_from = "EntityIdentityChoiceTypeSerde")]
pub enum EntityIdentityChoiceType {
	#[doc = r#"Indicates the "standard identity" of the target."#]
	Standard(crate::v2_5::types::StandardIdentityType),
	#[doc = r#"Indicates the "environment identity" of the target."#]
	Environment(crate::v2_5::types::EnvironmentIdentityType),
	#[doc = r#"Indicates the "platform identity" of the target."#]
	Platform(crate::v2_5::types::PlatformIdentityType),
	#[doc = r#"Indicates the "specific identity" of the target."#]
	Specific(crate::v2_5::types::SpecificIdentityType),
	#[doc = r#"Indicates the emitter type of the target."#]
	Emitter(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"Indicates the specific emitter type instance of the target.  Specific Emitter Identification (SEI) technology passively measures emitters, and characterizes unique emitter signatures that can be used for friendly or enemy emitter identification and fingerprinting.  A SEI and this SpecificEmitter element refer to a specific physical instance of a type of emitter as given in the sibling Emitter element."#]
	SpecificEmitter(crate::v2_5::types::SpecificEmitterIdentityType),
	#[doc = r#"Indicates the specific, generally blue force, vehicle instance of the Entity."#]
	SpecificVehicle(crate::v2_5::types::VehicleIdentificationType),
	#[doc = r#"Indicates the specific, generally blue force, vehicle instance of the Entity."#]
	SpecificFacility(FacilityIdentificationType),
	#[doc = r#"Indicates the EOB identity of the target."#]
	Eob(EobIdentityType),
	#[doc = r#"Indicates a type of weapon available. This element represents the MIL-STD-6016 specific type of the store. See MIL-STD-6016 for details; this schema will not redefine the enumerations. It is left to applications using this schema to enforce compatibility with MIL-STD-6016."#]
	Weapon(crate::v2_5::types::StoreType),
}
choice_convert_impls! {
	EntityIdentityChoiceType - EntityIdentityChoiceTypeSerde
	Standard,
	Environment,
	Platform,
	Specific,
	Emitter,
	SpecificEmitter,
	SpecificVehicle,
	SpecificFacility,
	Eob,
	Weapon,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityManagementDropTypeSerde")]
#[serde(try_from = "EntityManagementDropTypeSerde")]
pub enum EntityManagementDropType {
	#[doc = r#"Specifies a specific Entity to be dropped/removed."#]
	EntityId(Vec<crate::v2_5::types::EntityIdType>),
	#[doc = r#"Specifies a policy to be used to drop multiple Entities."#]
	DropPolicy(crate::v2_5::enums::EntityDropPolicyEnum),
}
choice_convert_impls! {
	EntityManagementDropType - EntityManagementDropTypeSerde
	EntityId,
	DropPolicy,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntityManagementRequestTypeSerde")]
#[serde(try_from = "EntityManagementRequestTypeSerde")]
pub enum EntityManagementRequestType {
	#[doc = r#"Indicates a request to split one or more Entities from a multi-contributor Entity."#]
	Split(crate::v2_5::types::EntityManagementSplitType),
	#[doc = r#"Indicates a request to merge two or more Entities into a single Entity."#]
	Merge(crate::v2_5::types::EntityManagementMergeType),
	#[doc = r#"Indicates a request to drop/remove an Entity."#]
	Drop(EntityManagementDropType),
	#[doc = r#"Indicates a request to add a drop restriction to an Entity."#]
	SetDropRestriction(crate::v2_5::types::EntityManagementDropRestrictType),
	#[doc = r#"Indicates a request to remove an existing drop restriction from an Entity."#]
	ClearDropRestriction(crate::v2_5::types::EntityManagementDropRestrictType),
	#[doc = r#"Indicates a request to swap the characteristics between two Fused Entities."#]
	Swap(crate::v2_5::types::EntityManagementSwapType),
	#[doc = r#"Indicates a request to set the fusion eligibility of an Entity."#]
	SetFusionEligibility(crate::v2_5::types::EntityManagementSetFusionEligibilityType),
	#[doc = r#"Indicates a request to set the identity of an Entity."#]
	SetIdentity(crate::v2_5::types::EntityManagementSetIdentityType),
	#[doc = r#"Reset the specified identity field to the default value."#]
	ClearIff(crate::v2_5::types::EntityManagementClearIdentityType),
	#[doc = r#"Indicates a request to modify the DownLocation of an Entity."#]
	SetDownLocation(crate::v2_5::types::EntityManagementDownType),
	#[doc = r#"Indicates a request to modify the Kinematics of an Entity."#]
	SetKinematics(crate::v2_5::types::EntityManagementKinematicsType),
	#[doc = r#"Indicates a request to modify the Mode of an Entity."#]
	SetMode(crate::v2_5::types::EntityManagementSetModeType),
	#[doc = r#"Indicates a request to modify the Activities of an Entity. The new list of Activities replaces the existing list for the entity. An empty list clears all Activities for the entity."#]
	SetActivityBy(crate::v2_5::types::EntityManagementSetActivityByType),
	#[doc = r#"Indicates a request to modify the Strength of an Entity."#]
	SetStrength(crate::v2_5::types::EntityManagementSetStrengthType),
	#[doc = r#"Indicates a request to modify the PlatformStatus of an Entity."#]
	SetPlatformStatus(crate::v2_5::types::EntityManagementSetPlatformStatusType),
	#[doc = r#"Set the endurance of an entity."#]
	SetEndurance(crate::v2_5::types::EntityManagementSetEnduranceType),
	#[doc = r#"Indicates a request to modify the Link16Metadata of an Entity."#]
	SetLink16Metadata(crate::v2_5::types::EntityManagementSetLink16MetadataType),
	#[doc = r#"This one choice may be sent by fusion and read by data links. Data links are expected to drop any requests with other choices filled in, and fusion is expected to drop any requests with this choice populated."#]
	ProposeCorrelation(crate::v2_5::types::EntityManagementProposeCorrelationType),
	#[doc = r#"Set the voice control frequency of an entity."#]
	SetVoiceControl(crate::v2_5::types::EntityManagementSetVoiceControlType),
	#[doc = r#"Adds or removes capabilities from an Entity. The new list of capabilities replaces the existing list for the entity. An empty list clears all capabilities for the entity."#]
	SetCapability(crate::v2_5::types::EntityManagementSetCapabilityType),
	#[doc = r#"Set the signalSummary of an entity."#]
	SetSignalSummary(crate::v2_5::types::EntityManagementSetSignalSummaryType),
}
choice_convert_impls! {
	EntityManagementRequestType - EntityManagementRequestTypeSerde
	Split,
	Merge,
	Drop,
	SetDropRestriction,
	ClearDropRestriction,
	Swap,
	SetFusionEligibility,
	SetIdentity,
	ClearIff,
	SetDownLocation,
	SetKinematics,
	SetMode,
	SetActivityBy,
	SetStrength,
	SetPlatformStatus,
	SetEndurance,
	SetLink16Metadata,
	ProposeCorrelation,
	SetVoiceControl,
	SetCapability,
	SetSignalSummary,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EntitySourceIdentifierTypeSerde")]
#[serde(try_from = "EntitySourceIdentifierTypeSerde")]
pub enum EntitySourceIdentifierType {
	#[doc = r#"Indicates an OB Record from which this Entity originated."#]
	ObRecordId(crate::v2_5::types::RecordIdType),
	#[doc = r#"Indicates the ID of an Entity-like object in an external system (non-UCI) which this Entity was derived from. For example, this element could indicate a MIL-STD-6016 track number."#]
	ExternalIdentifier(crate::v2_5::types::EntityExternalType),
	#[doc = r#"Indicates the Entity is an output of a fusion service and provides references to a fusion input Entity that fused/contributed to it."#]
	Fusion(Vec<crate::v2_5::types::EntityFusionSourceType>),
	#[doc = r#"This element represents a non-Entity UCI object/message ID from which this Entity was derived.  Most elements of type ID_Type infer a specific ID type.  This element is different in that any UCI ID type is allowed. This will support moving target date from Tasks (possibly specified by ground forces) to Entities in order to designate DMPIs.  It will also support linkage to a generic event or ID.  Examples include; association of an entity with an operator location of interest. Locations are maintained independently between OLI and entity, indication that this entity was created as a result of completing a task, or a command which generated the entity."#]
	InternallyDerivedId(crate::v2_5::types::IdType),
	#[doc = r#"Indicates this Entity is from a Capability source and provides the unique ID of the Capability instance along with the Capability's internal reference number/ID that corresponds to this Entity."#]
	Capability(crate::v2_5::types::EntityCapabilitySourceType),
	#[doc = r#"Indicates the unique ID of the Product from which the Entity was derived."#]
	ProductMetadataId(Vec<crate::v2_5::types::ProductMetadataIdType>),
	#[doc = r#"Indicates the operator associated with this Entity."#]
	Operator(OperatorReferenceType),
}
choice_convert_impls! {
	EntitySourceIdentifierType - EntitySourceIdentifierTypeSerde
	ObRecordId,
	ExternalIdentifier,
	Fusion,
	InternallyDerivedId,
	Capability,
	ProductMetadataId,
	Operator,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EobIdentityTypeSerde")]
#[serde(try_from = "EobIdentityTypeSerde")]
pub enum EobIdentityType {
	#[doc = r#"Indicates the uniquely identifying information of an EOB Facility."#]
	Facility(crate::v2_5::types::EobFacilityIdentityType),
	#[doc = r#"Indicates the uniquely identifying information of EOB Equipment."#]
	Equipment(crate::v2_5::types::EobEquipmentIdentityType),
}
choice_convert_impls! {
	EobIdentityType - EobIdentityTypeSerde
	Facility,
	Equipment,
}

#[doc = r#"Indicates a choice between propagation parameters. Allows either the selection of USSF Astrodynamic Standards orbital model parameters or a reference to a PropagatorSettingsID."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EphemerisPropagatorChoiceReferenceTypeSerde")]
#[serde(try_from = "EphemerisPropagatorChoiceReferenceTypeSerde")]
pub enum EphemerisPropagatorChoiceReferenceType {
	#[doc = r#"The USSF Astrodynamic Standards orbital model parameters."#]
	OrbitalModelPropagator(crate::v2_5::types::EphemerisOrbitalModelType),
	#[doc = r#"An ID that references a pre-defined set of propagator settings."#]
	PropagatorSettingsId(crate::v2_5::types::PropagatorSettingsIdType),
}
choice_convert_impls! {
	EphemerisPropagatorChoiceReferenceType - EphemerisPropagatorChoiceReferenceTypeSerde
	OrbitalModelPropagator,
	PropagatorSettingsId,
}

#[doc = r#"Indicates a choice between propagation parameters. Allows either the selection of USSF Astrodynamic Standards orbital model parameters or more general propagator settings."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EphemerisPropagatorChoiceTypeSerde")]
#[serde(try_from = "EphemerisPropagatorChoiceTypeSerde")]
pub enum EphemerisPropagatorChoiceType {
	#[doc = r#"The USSF Astrodynamic Standards orbital model parameters."#]
	OrbitalModelPropagator(crate::v2_5::types::EphemerisOrbitalModelType),
	#[doc = r#"An ID that references a pre-defined set of propagator settings."#]
	PropagatorSettingsId(crate::v2_5::types::PropagatorSettingsIdType),
	#[doc = r#"The type of propagator to use."#]
	PropagatorChoice(PropagatorChoiceType),
}
choice_convert_impls! {
	EphemerisPropagatorChoiceType - EphemerisPropagatorChoiceTypeSerde
	OrbitalModelPropagator,
	PropagatorSettingsId,
	PropagatorChoice,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EsmAcquisitionTargetTypeSerde")]
#[serde(try_from = "EsmAcquisitionTargetTypeSerde")]
pub enum EsmAcquisitionTargetType {
	#[doc = r#"Indicates the unique ID of a specific Entity that is a target of ESM acquisition."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates a type of emitter that is a target of ESM acquisition."#]
	EmitterType(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"Indicates a specific emitter type ***instance*** that is a target of ESM acquisition."#]
	SpecificEmitter(crate::v2_5::types::SpecificEmitterIdentityType),
	#[doc = r#"Indicates summary signal characteristics that are a target of ESM acquisition."#]
	SignalDescription(crate::v2_5::types::SignalSummaryType),
	#[doc = r#"Indicates the unique ID of a specific Signal that is a target of ESM acquisition."#]
	SignalId(crate::v2_5::types::SignalIdType),
}
choice_convert_impls! {
	EsmAcquisitionTargetType - EsmAcquisitionTargetTypeSerde
	EntityId,
	EmitterType,
	SpecificEmitter,
	SignalDescription,
	SignalId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EsmCommandTypeSerde")]
#[serde(try_from = "EsmCommandTypeSerde")]
pub enum EsmCommandType {
	#[doc = r#"Indicates a new invocation of an ESM Capability.  Generally, if accepted, the command will result in one or more new ESM Activities being created and reported via the ESM_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::EsmCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing ESM Activity (which was previously reported via the ESM_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent ESM_Activity messages."#]
	Activity(crate::v2_5::types::EsmActivityCommandType),
}
choice_convert_impls! {
	EsmCommandType - EsmCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EsmLocationTypeSerde")]
#[serde(try_from = "EsmLocationTypeSerde")]
pub enum EsmLocationType {
	#[doc = r#"Indicates the source of or explicit values for the kinematic location of the target."#]
	TargetLocationData(EsmSubcapabilityTargetLocationDataType),
	#[doc = r#"Indicates the volume to search within."#]
	EsmAirVolume(crate::v2_5::types::AirVolumeSensorReferencedType),
}
choice_convert_impls! {
	EsmLocationType - EsmLocationTypeSerde
	TargetLocationData,
	EsmAirVolume,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EsmSubcapabilityTargetLocationDataTypeSerde")]
#[serde(try_from = "EsmSubcapabilityTargetLocationDataTypeSerde")]
pub enum EsmSubcapabilityTargetLocationDataType {
	#[doc = r#"Indicates the EntityID associated with the pulse data collection."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the dwell's Field of View details associated with the pulse data collection."#]
	DwellFov(crate::v2_5::types::NedConeType),
	#[doc = r#"Indicates the Point Target details associated with the pulse data collection."#]
	PointTarget(crate::v2_5::types::PointTargetType),
}
choice_convert_impls! {
	EsmSubcapabilityTargetLocationDataType - EsmSubcapabilityTargetLocationDataTypeSerde
	EntityId,
	DwellFov,
	PointTarget,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EsmTargetTypeSerde")]
#[serde(try_from = "EsmTargetTypeSerde")]
pub enum EsmTargetType {
	#[doc = r#"Indicates the unique ID of a specific Entity that is a target of ESM acquisition."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates a type of emitter that is a target of ESM acquisition."#]
	EmitterType(crate::v2_5::types::EmitterIdentityType),
	#[doc = r#"Indicates a specific emitter type ***instance*** that is a target of ESM acquisition."#]
	SpecificEmitter(crate::v2_5::types::SpecificEmitterIdentityType),
	#[doc = r#"Indicates summary signal characteristics that are a target of ESM acquisition."#]
	SignalDescription(crate::v2_5::types::SignalSummaryType),
	#[doc = r#"Indicates the unique ID of a specific Signal that is a target of ESM acquisition."#]
	SignalId(crate::v2_5::types::SignalIdType),
	#[doc = r#"Indicates the Emitter Priority Bin for grouping ESM emitters in the ESM profile (to set Activation and ResourceAccessPriority). These bins are used to set ResourceAccessPriority for a "binned" set of emitters without identifying them individually every time their ResourceAccessPriority needs to change. This element follows the priority weighting approach where higher values are higher priority and lower values are lower priority."#]
	EmitterPriorityBin(crate::v2_5::common::PriorityWeightType),
}
choice_convert_impls! {
	EsmTargetType - EsmTargetTypeSerde
	EntityId,
	EmitterType,
	SpecificEmitter,
	SignalDescription,
	SignalId,
	EmitterPriorityBin,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EthernetSettingsTypeSerde")]
#[serde(try_from = "EthernetSettingsTypeSerde")]
pub enum EthernetSettingsType {
	#[doc = r#"Internet Protocol version 4 (IPv4) type consisting of a Static IP Address, Preferred DNS Server and Port Number settings."#]
	Ipv4(crate::v2_5::types::Ipv4SettingsType),
	#[doc = r#"Internet Protocol version 6 (IPv6) type  consisting of a Static IP Address, Preferred DNS Server and Port Number settings."#]
	Ipv6(crate::v2_5::types::Ipv6SettingsType),
}
choice_convert_impls! {
	EthernetSettingsType - EthernetSettingsTypeSerde
	Ipv4,
	Ipv6,
}

#[doc = r#"Provides a choice of event offset types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EventOffsetChoiceTypeSerde")]
#[serde(try_from = "EventOffsetChoiceTypeSerde")]
pub enum EventOffsetChoiceType {
	#[doc = r#"The offset time from the associated OrbitalEvent."#]
	OffsetTime(chrono::TimeDelta),
	#[doc = r#"The offset angle from the associated Event."#]
	OffsetAngle(crate::v2_5::common::AngleType),
	#[doc = r#"The Azimuth, Elevation, and Range of the Offset from the Event."#]
	AzEl(crate::v2_5::types::LosInertialBType),
}
choice_convert_impls! {
	EventOffsetChoiceType - EventOffsetChoiceTypeSerde
	OffsetTime,
	OffsetAngle,
	AzEl,
}

#[doc = r#"Provides a choice of event window size definitions."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "EventWindowChoiceTypeSerde")]
#[serde(try_from = "EventWindowChoiceTypeSerde")]
pub enum EventWindowChoiceType {
	#[doc = r#"Indicates a boundary for an Event described as an angle."#]
	WindowAngle(crate::v2_5::common::AnglePositiveType),
	#[doc = r#"Indicates a boundary for an OrbitalEvent described as a time-based offset."#]
	WindowDuration(chrono::TimeDelta),
	#[doc = r#"Indicates a boundary for an Event described as a radius."#]
	WindowRadius(crate::v2_5::common::DistanceType),
}
choice_convert_impls! {
	EventWindowChoiceType - EventWindowChoiceTypeSerde
	WindowAngle,
	WindowDuration,
	WindowRadius,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ExecutionSequenceInsertionTypeChoiceTypeSerde")]
#[serde(try_from = "ExecutionSequenceInsertionTypeChoiceTypeSerde")]
pub enum ExecutionSequenceInsertionTypeChoiceType {
	#[doc = r#"Indicates an insertion at the start of the existing execution plan set."#]
	InsertAtStart(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the execution plan set after which to insert the additional execution plan sets."#]
	InsertAfterExecutionPlanSetId(crate::v2_5::types::ExecutionPlanSetIdType),
}
choice_convert_impls! {
	ExecutionSequenceInsertionTypeChoiceType - ExecutionSequenceInsertionTypeChoiceTypeSerde
	InsertAtStart,
	InsertAfterExecutionPlanSetId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ExecutionSequenceReplaceOrModifyChoiceTypeSerde")]
#[serde(try_from = "ExecutionSequenceReplaceOrModifyChoiceTypeSerde")]
pub enum ExecutionSequenceReplaceOrModifyChoiceType {
	#[doc = r#"Indicates a complete replacement of a Mission Plan's execution sequence."#]
	ReplaceExecutionSequence(crate::v2_5::types::ExecutionSequenceType),
	#[doc = r#"Indicates a modification of a Mission Plan's execution sequence."#]
	ModifyExecutionSequence(crate::v2_5::types::ExecutionSequenceModificationDetailsType),
}
choice_convert_impls! {
	ExecutionSequenceReplaceOrModifyChoiceType - ExecutionSequenceReplaceOrModifyChoiceTypeSerde
	ReplaceExecutionSequence,
	ModifyExecutionSequence,
}

#[doc = r#"Provides identification of an object associated with the Air Force Space Command."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FacilityIdentificationTypeSerde")]
#[serde(try_from = "FacilityIdentificationTypeSerde")]
pub enum FacilityIdentificationType {
	#[doc = r#"Indicates the site ID."#]
	SiteIdentifier(u16),
	#[doc = r#"Indicates the ID of the sensor."#]
	SensorIdentifier(u16),
	#[doc = r#"Indicates the code for an airfield that is the facility or where the facility is based."#]
	IcaoCode(crate::v2_5::common::IcaoAirfieldIdentifierType),
	#[doc = r#"The ID for this facility is associated with a key that exists in another system, protocol, or network."#]
	ForeignFacilityKey(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	FacilityIdentificationType - FacilityIdentificationTypeSerde
	SiteIdentifier,
	SensorIdentifier,
	IcaoCode,
	ForeignFacilityKey,
}

#[doc = r#"Encoding types for CVEnumISMCATFGIOpen Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATFGIOpen.xml CVE.(U) 
				  All currently valid GENC trigraphs except USA in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph. UNKNOWN removed since GENC has it as AX1

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATFGIOpen.xml"#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FgiSourceOpenChoiceTypeSerde")]
#[serde(try_from = "FgiSourceOpenChoiceTypeSerde")]
pub enum FgiSourceOpenChoiceType {
	#[doc = r#"CVEnumISMCATFGIOpen Values"#]
	ForeignGovernmentIdentifier(crate::v2_5::enums::FgiSourceOpenEnum),
	#[doc = r#"North Atlantic Treaty Organization Special Words"#]
	NatoSpecialWord(crate::v2_5::common::NatoSpecialWordsType),
}
choice_convert_impls! {
	FgiSourceOpenChoiceType - FgiSourceOpenChoiceTypeSerde
	ForeignGovernmentIdentifier,
	NatoSpecialWord,
}

#[doc = r#"Encoding types for CVEnumISMCATFGIProtected Version 2.1 controlled vocabulary enumerations. Derived from the CVEnumISMCATFGIProtected.xml CVE.(U) FGI, followed by GENC trigraphs (except USA and AX1) in alphabetical order by trigraph, followed by IC Markings System Register and Manual Coalition tetragraphs in alphabetical order by tetragraph.

						PERMISSIBLE VALUES

						The permissible values for this simple type are defined in the Controlled Value Enumeration:

						CVEnumISMCATFGIProtected.xml"#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FgiSourceProtectedChoiceTypeSerde")]
#[serde(try_from = "FgiSourceProtectedChoiceTypeSerde")]
pub enum FgiSourceProtectedChoiceType {
	#[doc = r#"CVEnumISMCATFGIProtected Values"#]
	ForeignGovernmentIdentifier(crate::v2_5::enums::FgiSourceProtectedEnum),
	#[doc = r#"North Atlantic Treaty Organization Special Words"#]
	NatoSpecialWord(crate::v2_5::common::NatoSpecialWordsType),
}
choice_convert_impls! {
	FgiSourceProtectedChoiceType - FgiSourceProtectedChoiceTypeSerde
	ForeignGovernmentIdentifier,
	NatoSpecialWord,
}

#[doc = r#"This element defines a filter which can be applied to any file regardless of type"#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FileFilterTypeSerde")]
#[serde(try_from = "FileFilterTypeSerde")]
pub enum FileFilterType {
	#[doc = r#"This field specifies a filter criteria for the source of a file."#]
	Source(crate::v2_5::types::SourceFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of a file.  If a zone is supplied, any intersection with the supplied zone is considered passing the filter criteria.  If a point is supplied, it indicates that the file must contain the point for the filter to apply.  Point based files with a point location filter have to match exactly to a defined point in the file."#]
	Geospatial(LocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of the source of a file."#]
	SourceGeospatial(crate::v2_5::types::SourceLocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the file format of a file."#]
	FileFormat(FileFormatType),
	#[doc = r#"This field specifies a filter criteria for the classification of a file."#]
	SecurityInformation(crate::v2_5::types::SecurityInformationType),
	#[doc = r#"This field specifies a filter criteria for a range of creation times of a file."#]
	DateTimeRange(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"This field specifies the criteria used to filter messages based on a specified QueryType expression."#]
	QueryFilter(QueryType),
	#[doc = r#"The file type that the subplan applies to."#]
	FileType(crate::v2_5::enums::FileTypeEnum),
}
choice_convert_impls! {
	FileFilterType - FileFilterTypeSerde
	Source,
	Geospatial,
	SourceGeospatial,
	FileFormat,
	SecurityInformation,
	DateTimeRange,
	QueryFilter,
	FileType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FileFormatTypeSerde")]
#[serde(try_from = "FileFormatTypeSerde")]
pub enum FileFormatType {
	#[doc = r#"Describes the digital format of a file."#]
	Mime(crate::v2_5::common::MimeType),
	#[doc = r#"Indicates a file format that is not registered as a MIME type."#]
	NonMime(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	FileFormatType - FileFormatTypeSerde
	Mime,
	NonMime,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FileReferenceTypeSerde")]
#[serde(try_from = "FileReferenceTypeSerde")]
pub enum FileReferenceType {
	#[doc = r#"This element defines the file name of the File.  This can be used if the name of a File has been advertised by the system via an out-of-band source.  This only works if the service interface receiving this message is the service interface advertising the filename via the out-of-band source."#]
	FileName(crate::v2_5::common::FileNameType),
	#[doc = r#"The File metadata ID for the File. This element is the most common method for referencing a File.  A service has to also obtain the FileLocation message to discover how to retrieve the File.  A File location can be a network location or a reference to a system.  If the location is a reference to a system, a download request must be issued to request that the system download the File and make it available via a network URI."#]
	FileMetadataId(crate::v2_5::types::FileMetadataIdType),
}
choice_convert_impls! {
	FileReferenceType - FileReferenceTypeSerde
	FileName,
	FileMetadataId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FusionSettingsRequestTypeSerde")]
#[serde(try_from = "FusionSettingsRequestTypeSerde")]
pub enum FusionSettingsRequestType {
	#[doc = r#"Defines thresholds for determining the staleness of data."#]
	SetStalenessThresholds(crate::v2_5::types::EntityStalenessThresholdsType),
	#[doc = r#"Indicates settings that control how Entity fusion behaves.  These settings apply to all "fusion eligible" Entities."#]
	SetMergeAndDropSettings(crate::v2_5::types::FusionSettingsType),
	#[doc = r#"Indicates a request that configures the contributing sources to fusion and details of how they are allowed to contribute."#]
	SetInputSources(crate::v2_5::types::FusionSourcesType),
}
choice_convert_impls! {
	FusionSettingsRequestType - FusionSettingsRequestTypeSerde
	SetStalenessThresholds,
	SetMergeAndDropSettings,
	SetInputSources,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "FuzeTriggerTypeSerde")]
#[serde(try_from = "FuzeTriggerTypeSerde")]
pub enum FuzeTriggerType {
	#[doc = r#"Weapon fuze distance setting applies to proximity and hydrostatic fuze modes.  Hydrostatic values will be negative to denote subsurface.  Proximity values will be positive to denote height of burst above ground or distance to the target depending on the fuze's sensing."#]
	FuzeDistance(crate::v2_5::common::DistanceType),
	#[doc = r#"Weapon fuze delay time setting; applies to impact and time fuze modes."#]
	FuzeDelayTime(chrono::TimeDelta),
}
choice_convert_impls! {
	FuzeTriggerType - FuzeTriggerTypeSerde
	FuzeDistance,
	FuzeDelayTime,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GatewayCommandTypeSerde")]
#[serde(try_from = "GatewayCommandTypeSerde")]
pub enum GatewayCommandType {
	#[doc = r#"Indicates a new invocation of a Gateway Capability.  Generally, if accepted, the command will result in one or more new Gateway Activities being created and reported via the Gateway Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::GatewayCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Gateway Activity (which was previously reported via the Gateway Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent GatewayActivity messages."#]
	Activity(crate::v2_5::types::GatewayActivityCommandType),
}
choice_convert_impls! {
	GatewayCommandType - GatewayCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"Contains methods for describing geographic area characteristics of a Link 16 filter."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GatewayLink16ConfigurationAreaTypeSerde")]
#[serde(try_from = "GatewayLink16ConfigurationAreaTypeSerde")]
pub enum GatewayLink16ConfigurationAreaType {
	#[doc = r#"Circular filter area."#]
	Circle(crate::v2_5::types::GatewayLink16ConfigurationCircleType),
	#[doc = r#"Nested circle filter area."#]
	Annulus(crate::v2_5::types::GatewayLink16ConfigurationAnnulusType),
	#[doc = r#"Circular sector filter area."#]
	CircularSector(crate::v2_5::types::GatewayLink16ConfigurationCircularSectorType),
	#[doc = r#"Rectangular filter area."#]
	Rectangle(crate::v2_5::types::GatewayLink16ConfigurationRectangleType),
	#[doc = r#"Line-based filter area."#]
	Line(crate::v2_5::types::GatewayLink16ConfigurationLineType),
	#[doc = r#"Polygon-based filter area."#]
	Polygon(crate::v2_5::types::GatewayLink16ConfigurationPolygonType),
	#[doc = r#"Elliptical filter area."#]
	Ellipse(crate::v2_5::types::GatewayLink16ConfigurationEllipseType),
}
choice_convert_impls! {
	GatewayLink16ConfigurationAreaType - GatewayLink16ConfigurationAreaTypeSerde
	Circle,
	Annulus,
	CircularSector,
	Rectangle,
	Line,
	Polygon,
	Ellipse,
}

#[doc = r#"Contains methods for describing movement characteristics of a Link 16 filter."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GatewayLink16ConfigurationMotionTypeSerde")]
#[serde(try_from = "GatewayLink16ConfigurationMotionTypeSerde")]
pub enum GatewayLink16ConfigurationMotionType {
	#[doc = r#"Filter is moving with independent course and speed."#]
	Moving(crate::v2_5::types::GatewayLink16ConfigurationMovementVectorType),
	#[doc = r#"Filter is moving relative to the location of the identified item."#]
	Slaved(GatewayLink16ConfigurationSlaveType),
}
choice_convert_impls! {
	GatewayLink16ConfigurationMotionType - GatewayLink16ConfigurationMotionTypeSerde
	Moving,
	Slaved,
}

#[doc = r#"Identifies a geographically-located item to which a Link 16 filter is slaved, meaning that the filter's current location should be considered to always be relative to the location of the identified item."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GatewayLink16ConfigurationSlaveTypeSerde")]
#[serde(try_from = "GatewayLink16ConfigurationSlaveTypeSerde")]
pub enum GatewayLink16ConfigurationSlaveType {
	#[doc = r#"Filter is slaved to the designated Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Filter is slaved to the designated OperatorLocationOfInterest."#]
	OperatorLocationOfInterestId(crate::v2_5::types::OperatorLocationOfInterestIdType),
	#[doc = r#"Filter is slaved to the designated OpPoint."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Filter is slaved to the designated System."#]
	SystemId(crate::v2_5::types::SystemIdType),
}
choice_convert_impls! {
	GatewayLink16ConfigurationSlaveType - GatewayLink16ConfigurationSlaveTypeSerde
	EntityId,
	OperatorLocationOfInterestId,
	OpPointId,
	SystemId,
}

#[doc = r#"Container to reference the appropriate geo-located object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GeoLocatedObjectTypeSerde")]
#[serde(try_from = "GeoLocatedObjectTypeSerde")]
pub enum GeoLocatedObjectType {
	#[doc = r#"An Entity to use as the object reference."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"A System to use as the object reference."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"An OpPoint to use as the object reference."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"An OpLine to use as the object reference."#]
	OpLineId(crate::v2_5::types::OpLineIdType),
	#[doc = r#"An OpZone to use as the object reference."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"An OpVolume to use as the object reference."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
	#[doc = r#"A DMPI to use as the object reference."#]
	DmpiId(crate::v2_5::types::DmpiIdType),
	#[doc = r#"A SignalReport to use as the object reference."#]
	SignalReportId(crate::v2_5::types::SignalReportIdType),
}
choice_convert_impls! {
	GeoLocatedObjectType - GeoLocatedObjectTypeSerde
	EntityId,
	SystemId,
	OpPointId,
	OpLineId,
	OpZoneId,
	OpVolumeId,
	DmpiId,
	SignalReportId,
}

#[doc = r#"A choice between archived objects with a defined location."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "GeoLocatedStoredObjectTypeSerde")]
#[serde(try_from = "GeoLocatedStoredObjectTypeSerde")]
pub enum GeoLocatedStoredObjectType {
	#[doc = r#"The last Entity value reported before the emergency point was created."#]
	Entity(crate::v2_5::types::EntityDataType),
	#[doc = r#"The last System value reported before the emergency point was created."#]
	System(crate::v2_5::types::SystemDataType),
	#[doc = r#"The last OpPoint value reported before the emergency point was created."#]
	OpPoint(crate::v2_5::types::EmergencyReferenceOpPointType),
	#[doc = r#"The last OpLine value reported before the emergency point was created."#]
	OpLine(crate::v2_5::types::OpLineMdt),
	#[doc = r#"The last OpZone value reported before the emergency point was created."#]
	OpZone(crate::v2_5::types::OpZoneMdt),
	#[doc = r#"The last OpVolume value reported before the emergency point was created."#]
	OpVolume(crate::v2_5::types::OpVolumeMdt),
}
choice_convert_impls! {
	GeoLocatedStoredObjectType - GeoLocatedStoredObjectTypeSerde
	Entity,
	System,
	OpPoint,
	OpLine,
	OpZone,
	OpVolume,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "HeadingHoldOrConstraintChoiceTypeSerde")]
#[serde(try_from = "HeadingHoldOrConstraintChoiceTypeSerde")]
pub enum HeadingHoldOrConstraintChoiceType {
	#[doc = r#"Indicates navigation by updated heading hold."#]
	HeadingHold(crate::v2_5::types::NavigationByVectorType),
	#[doc = r#"Indicates navigation by updated heading hold constraint, seen in the sibling Constraints element."#]
	HeadingHoldConstraint(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	HeadingHoldOrConstraintChoiceType - HeadingHoldOrConstraintChoiceTypeSerde
	HeadingHold,
	HeadingHoldConstraint,
}

#[doc = r#"Provides the container that allows for specifying ways to identify the battlespace object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IdentityKindAssetTypeSerde")]
#[serde(try_from = "IdentityKindAssetTypeSerde")]
pub enum IdentityKindAssetType {
	#[doc = r#"Indicates a specific instance of an asset; a System or Entity."#]
	ByInstance(AssetType),
	#[doc = r#"Indicates an asset based on Identity.  This element can be used to select an asset by type such as friendly, tank, bomber or a specific aircraft type.  Zero to many instances of UCI messages that include the Identity taxonomy could then be a match such as a SystemStatus or Entity."#]
	ByIdentity(crate::v2_5::types::IdentityType),
	#[doc = r#"Indicates an instance of a System expressed as a planning candidate.  A System given here, instead of by the sibling elements, has the additional context of existing *Plans to consider."#]
	ByPlan(crate::v2_5::types::ByPlanType),
}
choice_convert_impls! {
	IdentityKindAssetType - IdentityKindAssetTypeSerde
	ByInstance,
	ByIdentity,
	ByPlan,
}

#[doc = r#"Provides the container that allows for specifying ways to identify the battlespace object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IdentityKindInstanceTypeSerde")]
#[serde(try_from = "IdentityKindInstanceTypeSerde")]
pub enum IdentityKindInstanceType {
	#[doc = r#"Indicates the specific identity of the battlespace object.  This could be the specific Entity given by its ID, an Operator-generated location or some other specific target."#]
	ByInstance(TargetType),
	#[doc = r#"Indicates the "platform identity" of the asset."#]
	ByIdentity(crate::v2_5::types::IdentityType),
}
choice_convert_impls! {
	IdentityKindInstanceType - IdentityKindInstanceTypeSerde
	ByInstance,
	ByIdentity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IffActiveModesTypeSerde")]
#[serde(try_from = "IffActiveModesTypeSerde")]
pub enum IffActiveModesType {
	#[doc = r#"Indicates the IFF modes enabled for response."#]
	IffTransponderModes(crate::v2_5::types::IffActivityTransponderType),
	#[doc = r#"Indicates Interrogation modes currently active."#]
	IffInterrogationModes(crate::v2_5::types::IffModeSelectionType),
}
choice_convert_impls! {
	IffActiveModesType - IffActiveModesTypeSerde
	IffTransponderModes,
	IffInterrogationModes,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IffCommandTypeSerde")]
#[serde(try_from = "IffCommandTypeSerde")]
pub enum IffCommandType {
	#[doc = r#"Indicates a new invocation of an IFF Capability.  Generally, if accepted, the command will result in one or more new IFF_Activities being created and reported via the IFF_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::IffCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing IFF Activity (which was previously reported via the IFF_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent IFF_Activity messages."#]
	Activity(crate::v2_5::types::IffActivityCommandType),
}
choice_convert_impls! {
	IffCommandType - IffCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IffInterrogatorTransponderModesTypeSerde")]
#[serde(try_from = "IffInterrogatorTransponderModesTypeSerde")]
pub enum IffInterrogatorTransponderModesType {
	#[doc = r#"This sets up the transponder modes to be enabled by this command."#]
	TransponderModesControl(crate::v2_5::types::IffTransponderModeControlType),
	#[doc = r#"Indicates that the command is selecting the Interrogator mode of operation."#]
	InterrogatorModesEnable(crate::v2_5::types::IffInterrogatorModesEnableType),
}
choice_convert_impls! {
	IffInterrogatorTransponderModesType - IffInterrogatorTransponderModesTypeSerde
	TransponderModesControl,
	InterrogatorModesEnable,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IffKinematicsChoiceTypeSerde")]
#[serde(try_from = "IffKinematicsChoiceTypeSerde")]
pub enum IffKinematicsChoiceType {
	#[doc = r#"Measurements provided in sensor relative line of sight frame (body or inertial) expressed in Azimuth/Elevation/Slant Range.  Reference kinematics are found in the Source element."#]
	LosAzEl(crate::v2_5::types::LosMeasurementWithUncertaintyType),
	#[doc = r#"Indicates kinematics expressed according to the World Geodetic System (WGS) frame/standard."#]
	Wgs(crate::v2_5::types::KinematicsType),
	#[doc = r#"Indicates kinematics expressed according to the Earth-Centered, Earth-Fixed frame/standard."#]
	EcefKinematics(crate::v2_5::types::EcefKinematicsType),
}
choice_convert_impls! {
	IffKinematicsChoiceType - IffKinematicsChoiceTypeSerde
	LosAzEl,
	Wgs,
	EcefKinematics,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IffModeSInterrogatorAddressTypeSerde")]
#[serde(try_from = "IffModeSInterrogatorAddressTypeSerde")]
pub enum IffModeSInterrogatorAddressType {
	#[doc = r#"Interrogator ID (legacy address type): used for transponder lockout interactions"#]
	InterrogatorIdentifier(crate::v2_5::common::InterrogatorIdentifierType),
	#[doc = r#"Surveillance Identifier (modern address type): lockout interactions"#]
	SurveillanceIdentifier(crate::v2_5::common::SurveillanceIdentifierType),
}
choice_convert_impls! {
	IffModeSInterrogatorAddressType - IffModeSInterrogatorAddressTypeSerde
	InterrogatorIdentifier,
	SurveillanceIdentifier,
}

#[doc = r#"Indicates the point of impact."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ImpactPointTypeSerde")]
#[serde(try_from = "ImpactPointTypeSerde")]
pub enum ImpactPointType {
	#[doc = r#"Indicates a body location as an X,Y,Z offset relative to the body coordinate system."#]
	BodyLocation(crate::v2_5::types::OffsetLocationErrorType),
	#[doc = r#"Indicates a body face and radial offset."#]
	BodyFace(crate::v2_5::types::BodyFaceType),
}
choice_convert_impls! {
	ImpactPointType - ImpactPointTypeSerde
	BodyLocation,
	BodyFace,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "InputFileTypeSerde")]
#[serde(try_from = "InputFileTypeSerde")]
pub enum InputFileType {
	#[doc = r#"Identifies a specific File instance"#]
	FileReference(FileReferenceType),
	#[doc = r#"Specifies a File type and source.  If there is more than one filter type defined, treat the filters as a logical AND, where all criteria must be met to satisfy the filters.

This element can be used in 2 ways:

1.)  Streaming Files:  Streaming Files stream continuously and File management functions are applied to them continuously.  For example, continuous dissemination, classification, and processing of an EOIR video stream.  Streams may or may not have associated File metadata; therefore, type and source can be used to identify a specific stream as an input File to a File management function.

2.)  Discrete Files:  For discrete Files, this element can be used to make reference to all discrete Files of a specified type.  This use is typically used to reference Files that have already been created and is not intended to override a File management plan (dissemination, classification, etc.) over some indefinite time period in the future."#]
	FileFilter(Vec<FileFilterType>),
}
choice_convert_impls! {
	InputFileType - InputFileTypeSerde
	FileReference,
	FileFilter,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "InputProductOrFileChoiceTypeSerde")]
#[serde(try_from = "InputProductOrFileChoiceTypeSerde")]
pub enum InputProductOrFileChoiceType {
	#[doc = r#"This element specifies the file(s) to which the parent applies."#]
	InputFile(Vec<InputFileType>),
	#[doc = r#"This element specifies the product(s) to which the parent applies."#]
	InputProduct(Vec<InputProductType>),
}
choice_convert_impls! {
	InputProductOrFileChoiceType - InputProductOrFileChoiceTypeSerde
	InputFile,
	InputProduct,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "InputProductTypeSerde")]
#[serde(try_from = "InputProductTypeSerde")]
pub enum InputProductType {
	#[doc = r#"Identifies a specific product instance."#]
	ProductReference(ProductReferenceType),
	#[doc = r#"Specifies a product type and source.  If there is more than one filter type defined, treat the filters as a logical AND, where all criteria must be met to satisfy the filters.

This element can be used in 2 ways:

1.)  Streaming products:  Streaming products stream continuously and product management functions are applied to them continuously.  For example, continuous dissemination, classification, and processing of an EOIR video stream.  Streams may or may not have associated product metadata; therefore, type and source can be used to identify a specific stream as an input product to a product management function.

2.)  Discrete products:  For discrete products, this element can be used to make reference to all discrete products of a specified type.  This use is typically used to reference products that have already been created and is not intended to override a product management plan (download, dissemination, classification, etc.) over some indefinite time period in the future."#]
	ProductFilter(Vec<ProductFilterType>),
}
choice_convert_impls! {
	InputProductType - InputProductTypeSerde
	ProductReference,
	ProductFilter,
}

#[doc = r#"The type used to specify a distance or duration type for an interval."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IntervalChoiceTypeSerde")]
#[serde(try_from = "IntervalChoiceTypeSerde")]
pub enum IntervalChoiceType {
	#[doc = r#"Indicates the interval amount is a distance."#]
	Distance(crate::v2_5::common::DistanceType),
	#[doc = r#"Indicates the interval amount is a duration."#]
	Duration(chrono::TimeDelta),
}
choice_convert_impls! {
	IntervalChoiceType - IntervalChoiceTypeSerde
	Distance,
	Duration,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IoPortConfigurationStatusTypeSerde")]
#[serde(try_from = "IoPortConfigurationStatusTypeSerde")]
pub enum IoPortConfigurationStatusType {
	#[doc = r#"Ethernet settings for internet protocols such as IPv4 or IPv6."#]
	EthernetSettings(EthernetSettingsType),
	#[doc = r#"Communication serial port settings that may consist of port ID, current state, bit rate, character width, parity and stop bit condition."#]
	SerialPortSettings(crate::v2_5::types::CommPortSettingsType),
}
choice_convert_impls! {
	IoPortConfigurationStatusType - IoPortConfigurationStatusTypeSerde
	EthernetSettings,
	SerialPortSettings,
}

#[doc = r#"Specifies an IPv4 or IPv6 connection."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IpConnectionChoiceTypeSerde")]
#[serde(try_from = "IpConnectionChoiceTypeSerde")]
pub enum IpConnectionChoiceType {
	#[doc = r#"IPv4 connection information."#]
	Ipv4(crate::v2_5::types::Ipv4ConnectionType),
	#[doc = r#"IPv6 connection information."#]
	Ipv6(crate::v2_5::types::Ipv6ConnectionType),
}
choice_convert_impls! {
	IpConnectionChoiceType - IpConnectionChoiceTypeSerde
	Ipv4,
	Ipv6,
}

#[doc = r#"Specifies a server, client, or multicast IP connection."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IpConnectionTypeSerde")]
#[serde(try_from = "IpConnectionTypeSerde")]
pub enum IpConnectionType {
	#[doc = r#"Configure a Server IP connection."#]
	Server(IpConnectionChoiceType),
	#[doc = r#"Configure a Client IP connection."#]
	Client(IpConnectionChoiceType),
	#[doc = r#"Configure a Multicast or Broadcast IP connection."#]
	Multicast(IpConnectionChoiceType),
}
choice_convert_impls! {
	IpConnectionType - IpConnectionTypeSerde
	Server,
	Client,
	Multicast,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "IsarTargetTypeSerde")]
#[serde(try_from = "IsarTargetTypeSerde")]
pub enum IsarTargetType {
	#[doc = r#"Entity to be used to cue the activity/activities generated from this command."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Raw target location and velocity information used in lieu of an entity cue to direct activity/activities generated from this command."#]
	RawTarget(crate::v2_5::types::PointTargetType),
}
choice_convert_impls! {
	IsarTargetType - IsarTargetTypeSerde
	EntityId,
	RawTarget,
}

#[doc = r#"Indicates the kinematics expressed in one of several different kinematics standards."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "KinematicsChoiceTypeSerde")]
#[serde(try_from = "KinematicsChoiceTypeSerde")]
pub enum KinematicsChoiceType {
	#[doc = r#"Indicates kinematics expressed in one of several kinematics frames/standards."#]
	VolumeKinematics(crate::v2_5::types::OpVolumeKinematicsType),
	#[doc = r#"Indicates kinematics expressed in one of several orbital kinematics frames/standards."#]
	OrbitalKinematics(OrbitalKinematicsChoiceType),
	#[doc = r#"Describes the kinematics position and orientation in the Radial-Transverse-Normal reference frame with respect to an asset. For example the position and attitude of a sensor relative to a host satellite's body frame."#]
	LocalBodyPosition(crate::v2_5::types::RtnLocalPositionType),
}
choice_convert_impls! {
	KinematicsChoiceType - KinematicsChoiceTypeSerde
	VolumeKinematics,
	OrbitalKinematics,
	LocalBodyPosition,
}

#[doc = r#"Provides a choice of ways to express kinematics in one of several orbital kinematics frames/standards."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "KinematicsMultiStandardTypeSerde")]
#[serde(try_from = "KinematicsMultiStandardTypeSerde")]
pub enum KinematicsMultiStandardType {
	#[doc = r#"Indicates kinematics expressed in one of several orbital kinematics frames/standards."#]
	Orbital(OrbitalKinematicsChoiceType),
	#[doc = r#"Indicates kinematics expressed according to the World Geodetic System (WGS) frame/standard."#]
	Wgs(crate::v2_5::types::KinematicsType),
	#[doc = r#"Indicates the orbital solution to achieve the task must be within the defined relative plane angles min and max."#]
	DeltaOrbitalPlaneTolerance(crate::v2_5::types::AngleHalfPairType),
}
choice_convert_impls! {
	KinematicsMultiStandardType - KinematicsMultiStandardTypeSerde
	Orbital,
	Wgs,
	DeltaOrbitalPlaneTolerance,
}

#[doc = r#"Option to implicitly or explicitly provide the kinematics of an Entity or System.

This allows the option to override kinematics information for a known system/entity when there is no known kinematics information or the information is not appropriate (e.g. outdated) by the time of use."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "KinematicsOptionsTypeSerde")]
#[serde(try_from = "KinematicsOptionsTypeSerde")]
pub enum KinematicsOptionsType {
	#[doc = r#"Indicates the ID of the object for which the reference frame of the sibling elements are referenced."#]
	ReferenceAsset(AssetType),
	#[doc = r#"Indicates the kinematics of the object for which the reference frame of the sibling elements are referenced."#]
	KinematicsOverride(KinematicsMultiStandardType),
}
choice_convert_impls! {
	KinematicsOptionsType - KinematicsOptionsTypeSerde
	ReferenceAsset,
	KinematicsOverride,
}

#[doc = r#"Choice of either relative or geospatial point representing the vertex of a line."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LinePointChoiceTypeSerde")]
#[serde(try_from = "LinePointChoiceTypeSerde")]
pub enum LinePointChoiceType {
	#[doc = r#"Indicates a vertex of the line target.  Generally, services are encouraged to provide altitude and/or time data whenever it is known."#]
	Point(Vec<crate::v2_5::types::LinePoint2DType>),
	#[doc = r#"Indicates a relative vertex of the line target."#]
	RelativePoint(crate::v2_5::types::LineRelativeType),
}
choice_convert_impls! {
	LinePointChoiceType - LinePointChoiceTypeSerde
	Point,
	RelativePoint,
}

#[doc = r#"Stores the ID of an EW."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "Link16ElectronicWarfareDataStoreIdChoiceTypeSerde")]
#[serde(try_from = "Link16ElectronicWarfareDataStoreIdChoiceTypeSerde")]
pub enum Link16ElectronicWarfareDataStoreIdChoiceType {
	#[doc = r#"UCI IDs that correspond to entities."#]
	EntityId(Vec<crate::v2_5::types::EntityIdType>),
	#[doc = r#"UCI IDs that correspond to Signal Reports."#]
	SignalReportId(Vec<crate::v2_5::types::SignalReportIdType>),
}
choice_convert_impls! {
	Link16ElectronicWarfareDataStoreIdChoiceType - Link16ElectronicWarfareDataStoreIdChoiceTypeSerde
	EntityId,
	SignalReportId,
}

#[doc = r#"Stores the ID of a Friendly Target of Interest in a Link16 setting."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "Link16FriendlyTargetOfInterestDataStoreIdChoiceTypeSerde")]
#[serde(try_from = "Link16FriendlyTargetOfInterestDataStoreIdChoiceTypeSerde")]
pub enum Link16FriendlyTargetOfInterestDataStoreIdChoiceType {
	#[doc = r#"Stores the UCI IDs of entities."#]
	EntityId(Vec<crate::v2_5::types::EntityIdType>),
	#[doc = r#"Stores the UCI IDs of OpLines."#]
	OpLineId(Vec<crate::v2_5::types::OpLineIdType>),
	#[doc = r#"Stores the UCI IDs of OpPoints."#]
	OpPointId(Vec<crate::v2_5::types::OpPointIdType>),
	#[doc = r#"Stores the UCI IDs of OpZones."#]
	OpZoneId(Vec<crate::v2_5::types::OpZoneIdType>),
	#[doc = r#"Stores the UCI IDs of OpVolumes."#]
	OpVolumeId(Vec<crate::v2_5::types::OpVolumeIdType>),
	#[doc = r#"Stores the UCI IDs of subsystems."#]
	SystemId(Vec<crate::v2_5::types::SystemIdType>),
}
choice_convert_impls! {
	Link16FriendlyTargetOfInterestDataStoreIdChoiceType - Link16FriendlyTargetOfInterestDataStoreIdChoiceTypeSerde
	EntityId,
	OpLineId,
	OpPointId,
	OpZoneId,
	OpVolumeId,
	SystemId,
}

#[doc = r#"Provides information about the OpZone, OpVolume, OpLine, and OpPoint ID."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "Link16ReferencePointDataStoreIdChoiceTypeSerde")]
#[serde(try_from = "Link16ReferencePointDataStoreIdChoiceTypeSerde")]
pub enum Link16ReferencePointDataStoreIdChoiceType {
	#[doc = r#"UCI IDs that correspond to OpLines."#]
	OpLineId(Vec<crate::v2_5::types::OpLineIdType>),
	#[doc = r#"UCI IDs that correspond to OpPoints."#]
	OpPointId(Vec<crate::v2_5::types::OpPointIdType>),
	#[doc = r#"UCI IDs that correspond to OpZones."#]
	OpZoneId(Vec<crate::v2_5::types::OpZoneIdType>),
	#[doc = r#"UCI IDs that correspond to OpVolumes."#]
	OpVolumeId(Vec<crate::v2_5::types::OpVolumeIdType>),
}
choice_convert_impls! {
	Link16ReferencePointDataStoreIdChoiceType - Link16ReferencePointDataStoreIdChoiceTypeSerde
	OpLineId,
	OpPointId,
	OpZoneId,
	OpVolumeId,
}

#[doc = r#"A choice of the kinematics position and orientation in the desired reference frame with respect to an asset that is not specified explicitly."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LocalPositionBaseChoiceTypeSerde")]
#[serde(try_from = "LocalPositionBaseChoiceTypeSerde")]
pub enum LocalPositionBaseChoiceType {
	#[doc = r#"The position and attitude expressed in the Radial-Transverse-Normal, or RTN, reference frame with respect to an asset. Also known as the RIC (Radial, In-Track, Cross-Track) frame. The Radial basis vector is aligned with the position vector. The Normal basis vector is aligned with the angular momentum direction (which is the orbit normal direction). The Transverse basis vector is in the direction of N x R to complete the right-handed triad. The asset is not explicitly specified and is assumed to be associated with a parent or sibling element."#]
	Rtn(crate::v2_5::types::RtnLocalPositionBaseType),
	#[doc = r#"The position and attitude expressed in the body frame of the referenced asset. The body frame axes, X, Y, and Z, are defined by the referenced asset's orientation. The asset is not explicitly specified and is assumed to be associated with a parent or sibling element."#]
	BodyFrame(crate::v2_5::types::BodyFrameLocalPositionBaseType),
}
choice_convert_impls! {
	LocalPositionBaseChoiceType - LocalPositionBaseChoiceTypeSerde
	Rtn,
	BodyFrame,
}

#[doc = r#"Specifies that the content being filtered must be in the specified zone if the zone is marked inclusionary or outside of the zone if the zone is marked exclusionary."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LocationFilterTypeSerde")]
#[serde(try_from = "LocationFilterTypeSerde")]
pub enum LocationFilterType {
	#[doc = r#"The geospatial zone to be used in the filter."#]
	Zone(crate::v2_5::types::ZoneInclusionType),
	#[doc = r#"A specific point location (2D or 3D) to be used in the filter."#]
	Location(crate::v2_5::types::Point2DType),
}
choice_convert_impls! {
	LocationFilterType - LocationFilterTypeSerde
	Zone,
	Location,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LoiterProgressTypeSerde")]
#[serde(try_from = "LoiterProgressTypeSerde")]
pub enum LoiterProgressType {
	#[doc = r#"Indicates the time at which the loiter will end."#]
	LoiterEndTime(chrono::DateTime<chrono::Utc>),
	#[doc = r#"Indicates the number of completed orbits of the loiter.  This is the required progress choice when the corresponding MissionPlan indicates a number of orbits."#]
	CompletedOrbits(u32),
}
choice_convert_impls! {
	LoiterProgressType - LoiterProgressTypeSerde
	LoiterEndTime,
	CompletedOrbits,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LoiterTypeSerde")]
#[serde(try_from = "LoiterTypeSerde")]
pub enum LoiterType {
	#[doc = r#"The details of an orbit loiter."#]
	Orbit(crate::v2_5::types::OrbitType),
	#[doc = r#"The details of a hover loiter."#]
	Hover(crate::v2_5::types::HoverType),
}
choice_convert_impls! {
	LoiterType - LoiterTypeSerde
	Orbit,
	Hover,
}

#[doc = r#"Provides a choice of line of sight vector definitions."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LosChoiceTypeSerde")]
#[serde(try_from = "LosChoiceTypeSerde")]
pub enum LosChoiceType {
	#[doc = r#"LOS data provided in alternate spaces to preserve relevant data in order to propagate the track. All data should be provided in the NED reference frame."#]
	LosAzEl(crate::v2_5::types::LosMeasurementAndUncertaintyType),
	#[doc = r#"Indicates the line of sight vector and associated uncertainty relative to a reference point in terms of slant range and a unit vector."#]
	Los3dKinematics(crate::v2_5::types::Los3DKinematicsType),
}
choice_convert_impls! {
	LosChoiceType - LosChoiceTypeSerde
	LosAzEl,
	Los3dKinematics,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LosDTypeSerde")]
#[serde(try_from = "LosDTypeSerde")]
pub enum LosDType {
	#[doc = r#"Defines a line of sight ray in a world inertial or body frame of reference."#]
	Los(crate::v2_5::types::LosVariableBType),
	#[doc = r#"Indicates the rates at which the LOS should move."#]
	LosRates(crate::v2_5::types::LosRatesType),
}
choice_convert_impls! {
	LosDType - LosDTypeSerde
	Los,
	LosRates,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "LostLinkSourceIdChoiceTypeSerde")]
#[serde(try_from = "LostLinkSourceIdChoiceTypeSerde")]
pub enum LostLinkSourceIdChoiceType {
	#[doc = r#"Indicates the unique ID of the System whose link failed."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"This identifies the Capability ID of the communication device whose link failed."#]
	CapabilityId(crate::v2_5::types::CapabilityIdType),
}
choice_convert_impls! {
	LostLinkSourceIdChoiceType - LostLinkSourceIdChoiceTypeSerde
	SystemId,
	CapabilityId,
}

#[doc = r#"Indicates choices for sensor collection maneuver constraints."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ManeuverConstraintsChoiceTypeSerde")]
#[serde(try_from = "ManeuverConstraintsChoiceTypeSerde")]
pub enum ManeuverConstraintsChoiceType {
	#[doc = r#"BasicManeuverConstraints are used for situations when looking for a maneuver, but there is nothing to drive expectations on maneuver size or direction.  Max delta-V is required to bound search space and Min to bound the sensor and processing resolution for maneuver detection."#]
	BasicManeuverConstraints(crate::v2_5::types::BasicManeuverConstraintsType),
	#[doc = r#"PredictedManeuverConstraints is used when there is information on the nominal parameters of an expected maneuver (such as a vector of delta-V values, maneuver duration, and maneuver start time). Since the maneuver has not happened, these are nominal and the actual maneuver may vary in start time, duration, and velocity parameters."#]
	PredictedManeuverConstraints(crate::v2_5::types::OrbitalManeuverDetailsType),
}
choice_convert_impls! {
	ManeuverConstraintsChoiceType - ManeuverConstraintsChoiceTypeSerde
	BasicManeuverConstraints,
	PredictedManeuverConstraints,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MeasurementKinematicsChoiceTypeSerde")]
#[serde(try_from = "MeasurementKinematicsChoiceTypeSerde")]
pub enum MeasurementKinematicsChoiceType {
	#[doc = r#"Measurements provided in sensor relative line of sight frame (body or inertial) expressed in Azimuth/Elevation/Slant Range.  Reference kinematics are found in the Source element."#]
	LosAzEl(crate::v2_5::types::LosMeasurementWithUncertaintyType),
	#[doc = r#"Measurements provided in the equatorial coordinate system expressed as Right Ascension/Declination/Range."#]
	LosEquatorial(crate::v2_5::types::EquatorialKinematicsType),
	#[doc = r#"Indicates kinematics expressed in one of several orbital kinematics frames/standards."#]
	Orbital(OrbitalKinematicsChoiceType),
	#[doc = r#"Indicates kinematics expressed according to the World Geodetic System (WGS) frame/standard."#]
	Wgs(crate::v2_5::types::KinematicsType),
	#[doc = r#"Indicates kinematics expressed according to the Earth-Centered, Earth-Fixed frame/standard."#]
	EcefKinematics(crate::v2_5::types::EcefKinematicsType),
}
choice_convert_impls! {
	MeasurementKinematicsChoiceType - MeasurementKinematicsChoiceTypeSerde
	LosAzEl,
	LosEquatorial,
	Orbital,
	Wgs,
	EcefKinematics,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionEnvironmentConstraintTypeSerde")]
#[serde(try_from = "MissionEnvironmentConstraintTypeSerde")]
pub enum MissionEnvironmentConstraintType {
	#[doc = r#"Indicates an environment constraint on an Entity, specified by the child EntityID field, that will be overridden."#]
	ConstrainedEntity(crate::v2_5::types::ConstrainedEntityType),
	#[doc = r#"Indicates an environment constraint on an Identity, that is specified by the child Identifier field, that will be overridden."#]
	ConstrainedIdentity(crate::v2_5::types::ConstrainedIdentityType),
	#[doc = r#"Indicates an environment constraint on a System, specified by the SystemID child field, that will be overridden."#]
	System(crate::v2_5::types::SystemStatusMdt),
	#[doc = r#"Indicates an environment constraint on an OpPoint, specified by the child OpPointID field, that will be overridden."#]
	ConstrainedOpPoint(crate::v2_5::types::ConstrainedOpPointType),
	#[doc = r#"Indicates an environment constraint on an OpLine, specified by the child OpLineID field, that will be overridden."#]
	ConstrainedOpLine(crate::v2_5::types::ConstrainedOpLineType),
	#[doc = r#"Indicates an environment constraint on an OpZone, specified by the child OpZoneID field, that will be overridden."#]
	ConstrainedOpZone(crate::v2_5::types::ConstrainedOpZoneType),
	#[doc = r#"Indicates an environment constraint on an OpVolume, specified by the child OpVolumeID field, that will be overridden."#]
	ConstrainedOpVolume(crate::v2_5::types::ConstrainedOpVolumeType),
	#[doc = r#"Indicates an environment constraint on the RiskLevel, specified by the child RiskLevel field, that will be overridden."#]
	RiskAdjustment(crate::v2_5::types::RequirementRiskAdjustmentType),
	#[doc = r#"Indicates an environment constraint on a service configuration parameter, specified by the ParameterID child field, that will be overridden."#]
	Parameter(crate::v2_5::types::ParameterAssertType),
}
choice_convert_impls! {
	MissionEnvironmentConstraintType - MissionEnvironmentConstraintTypeSerde
	ConstrainedEntity,
	ConstrainedIdentity,
	System,
	ConstrainedOpPoint,
	ConstrainedOpLine,
	ConstrainedOpZone,
	ConstrainedOpVolume,
	RiskAdjustment,
	Parameter,
}

#[doc = r#"Defines the class of object for which Mission Environment Object parameters support individual settings on specific instances."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionEnvironmentObjectClassTypeSerde")]
#[serde(try_from = "MissionEnvironmentObjectClassTypeSerde")]
pub enum MissionEnvironmentObjectClassType {
	#[doc = r#"The Mission Environment Object Values associated with an Entity Class of object."#]
	Entity(crate::v2_5::types::MissionEnvironmentObjectValueEntityType),
	#[doc = r#"The Mission Environment Object Values associated with a System Class of object."#]
	System(crate::v2_5::types::MissionEnvironmentObjectValueSystemType),
	#[doc = r#"The Mission Environment Object Values associated with an OpPoint Class of object."#]
	OpPoint(crate::v2_5::types::MissionEnvironmentObjectValueOpPointType),
	#[doc = r#"The Mission Environment Object Values associated with an OpLine Class of object."#]
	OpLine(crate::v2_5::types::MissionEnvironmentObjectValueOpLineType),
	#[doc = r#"The Mission Environment Object Values associated with an OpZone Class of object."#]
	OpZone(crate::v2_5::types::MissionEnvironmentObjectValueOpZoneType),
	#[doc = r#"The Mission Environment Object Values associated with an OpVolume Class of object."#]
	OpVolume(crate::v2_5::types::MissionEnvironmentObjectValueOpVolumeType),
	#[doc = r#"The Mission Environment Object Values associated with a Task Class of object."#]
	Task(crate::v2_5::types::MissionEnvironmentObjectValueTaskType),
	#[doc = r#"The Mission Environment Object Values associated with an Effect Class of object."#]
	Effect(crate::v2_5::types::MissionEnvironmentObjectValueEffectType),
	#[doc = r#"The Mission Environment Object Values associated with an Action Class of object."#]
	Action(crate::v2_5::types::MissionEnvironmentObjectValueActionType),
	#[doc = r#"The Mission Environment Object Values associated with a Response Class of object."#]
	Response(crate::v2_5::types::MissionEnvironmentObjectValueResponseType),
}
choice_convert_impls! {
	MissionEnvironmentObjectClassType - MissionEnvironmentObjectClassTypeSerde
	Entity,
	System,
	OpPoint,
	OpLine,
	OpZone,
	OpVolume,
	Task,
	Effect,
	Action,
	Response,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionPlanActivationDetailsTypeSerde")]
#[serde(try_from = "MissionPlanActivationDetailsTypeSerde")]
pub enum MissionPlanActivationDetailsType {
	#[doc = r#"Indicates simultaneous activation of all sub-*Plans (RoutePlan, RouteActivityPlan for example) of a MissionPlan into the same activation state."#]
	ByMissionPlan(crate::v2_5::types::MissionPlanActivationType),
	#[doc = r#"Indicates activation by sub-*Plan (RoutePlan or OrbitPlan for example) of the MissionPlan, with potentially different states for each."#]
	BySubPlan(crate::v2_5::types::MissionPlanSubplanActivationType),
}
choice_convert_impls! {
	MissionPlanActivationDetailsType - MissionPlanActivationDetailsTypeSerde
	ByMissionPlan,
	BySubPlan,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "MissionPlanCommandIdChoiceTypeSerde")]
pub enum MissionPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the MissionPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the MissionPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	MissionPlanCommandIdChoiceType - MissionPlanCommandIdChoiceTypeSerde
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionPlanningAutonomyResponseChoiceTypeSerde")]
#[serde(try_from = "MissionPlanningAutonomyResponseChoiceTypeSerde")]
pub enum MissionPlanningAutonomyResponseChoiceType {
	#[doc = r#"This element indicates the allowed autonomous mission planning for the ancestor Trigger.  It also indicates the expected MissionContingencyAlert for the ancestor Trigger; whenever autonomous mission planning is allowed and triggered, a MissionContingencyAlert is expected."#]
	AutonomousPlanningAction(Vec<crate::v2_5::types::PlanningAllowedEscalationType>),
	#[doc = r#"This element indicates that autonomous mission planning isn't allowed for the ancestor Trigger but a MissionContingencyAlert is expected."#]
	AlertOnly(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	MissionPlanningAutonomyResponseChoiceType - MissionPlanningAutonomyResponseChoiceTypeSerde
	AutonomousPlanningAction,
	AlertOnly,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MissionPlanningByResultAutonomousActionTypeSerde")]
#[serde(try_from = "MissionPlanningByResultAutonomousActionTypeSerde")]
pub enum MissionPlanningByResultAutonomousActionType {
	#[doc = r#"This element defines the mission planning type to be triggered.  If multiple instances are given, each should be of a different planning type as indicated by the child element."#]
	PlanningAllowed(Vec<crate::v2_5::types::PlanningAllowedType>),
	#[doc = r#"This element indicates that autonomous mission planning isn't allowed for the ancestor Trigger but a MissionContingencyAlert is expected."#]
	AlertOnly(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	MissionPlanningByResultAutonomousActionType - MissionPlanningByResultAutonomousActionTypeSerde
	PlanningAllowed,
	AlertOnly,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MtiTargetClassTypeSerde")]
#[serde(try_from = "MtiTargetClassTypeSerde")]
pub enum MtiTargetClassType {
	#[doc = r#"Indicates the class of the Moving Target Indicator (MTI) cross-section. Specific quantitative cross section values associated with these target classes are defined outside of this schema."#]
	MtiTargetClass(crate::v2_5::enums::MtiTargetClassEnum),
	#[doc = r#"Indicates the foreign class of the target for which the estimate applies."#]
	ForeignClass(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	MtiTargetClassType - MtiTargetClassTypeSerde
	MtiTargetClass,
	ForeignClass,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "MustFlyLocationTypeSerde")]
#[serde(try_from = "MustFlyLocationTypeSerde")]
pub enum MustFlyLocationType {
	#[doc = r#"Indicates the must-fly location is an Entity,  This is typically an overflight location with the desired altitude determined by the service design."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the must-fly location is an OpPoint."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Indicates the must-fly location is an OpLine."#]
	OpLineId(crate::v2_5::types::OpLineIdType),
	#[doc = r#"Indicates the must-fly location is an OpZone."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"Indicates the must-fly location is an OpVolume."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
	#[doc = r#"Indicates the must-fly location is a geospatial point."#]
	Point(crate::v2_5::types::Point3DType),
	#[doc = r#"Use of the Zone Target implies that a vehicle must enter the zone."#]
	ZoneTarget(crate::v2_5::types::ZoneExternalType),
	#[doc = r#"Use of the Line Target implies that a vehicle must fly through a corridor."#]
	LineTarget(crate::v2_5::types::LineTargetType),
	#[doc = r#"Use of the Volume Target implies that a vehicle must enter the volume."#]
	VolumeTarget(OpVolumeType),
}
choice_convert_impls! {
	MustFlyLocationType - MustFlyLocationTypeSerde
	EntityId,
	OpPointId,
	OpLineId,
	OpZoneId,
	OpVolumeId,
	Point,
	ZoneTarget,
	LineTarget,
	VolumeTarget,
}

#[doc = r#"The NameValuePairValue is used to report the value of a single status attribute.  This type is used to provide status for unique attributes that cannot be reported with other types or structures."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NameValuePairValueTypeSerde")]
#[serde(try_from = "NameValuePairValueTypeSerde")]
pub enum NameValuePairValueType {
	#[doc = r#"A boolean value."#]
	BooleanValue(bool),
	#[doc = r#"A byte value."#]
	ByteValue(i8),
	#[doc = r#"An unsigned byte value."#]
	UnsignedByteValue(u8),
	#[doc = r#"A short value."#]
	ShortValue(i16),
	#[doc = r#"An unsigned short value."#]
	UnsignedShortValue(u16),
	#[doc = r#"An int value."#]
	IntValue(i32),
	#[doc = r#"An unsigned integer value."#]
	UnsignedIntValue(u32),
	#[doc = r#"A long value."#]
	LongValue(i64),
	#[doc = r#"A float value."#]
	FloatValue(f32),
	#[doc = r#"A double value."#]
	DoubleValue(f64),
	#[doc = r#"A string value. A string value can be used to represent all non-primitive data types."#]
	StringValue(crate::v2_5::common::VisibleString256Type),
}
choice_convert_impls! {
	NameValuePairValueType - NameValuePairValueTypeSerde
	BooleanValue,
	ByteValue,
	UnsignedByteValue,
	ShortValue,
	UnsignedShortValue,
	IntValue,
	UnsignedIntValue,
	LongValue,
	FloatValue,
	DoubleValue,
	StringValue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NavigationByVectorChoiceTypeSerde")]
#[serde(try_from = "NavigationByVectorChoiceTypeSerde")]
pub enum NavigationByVectorChoiceType {
	#[doc = r#"Indicates the vector is describing the true navigation course of the platfom."#]
	VectorByCourse(crate::v2_5::common::AngleType),
	#[doc = r#"Indicates the vector is describing the heading of the platform with respect to True North, not necessarily the actual direction of travel."#]
	VectorByHeading(crate::v2_5::common::AngleType),
}
choice_convert_impls! {
	NavigationByVectorChoiceType - NavigationByVectorChoiceTypeSerde
	VectorByCourse,
	VectorByHeading,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NavigationByVectorPairChoiceTypeSerde")]
#[serde(try_from = "NavigationByVectorPairChoiceTypeSerde")]
pub enum NavigationByVectorPairChoiceType {
	#[doc = r#"Indicates the vector is describing the true navigation course of the platfom."#]
	VectorByCourse(crate::v2_5::common::AngleType),
	#[doc = r#"Indicates the range of vectors is describing the true navigation course of the platfom."#]
	VectorByCourseRange(crate::v2_5::types::AnglePairType),
	#[doc = r#"Indicates the vector is describing the heading of the platform with respect to True North, not necessarily the actual direction of travel"#]
	VectorByHeading(crate::v2_5::common::AngleType),
	#[doc = r#"Indicates the range of vectors is describing the heading of the platform with respect to True North, not necessarily the actual direction of travel"#]
	VectorByHeadingRange(crate::v2_5::types::AnglePairType),
}
choice_convert_impls! {
	NavigationByVectorPairChoiceType - NavigationByVectorPairChoiceTypeSerde
	VectorByCourse,
	VectorByCourseRange,
	VectorByHeading,
	VectorByHeadingRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NavigationCapabilityOptionsChoiceTypeSerde")]
#[serde(try_from = "NavigationCapabilityOptionsChoiceTypeSerde")]
pub enum NavigationCapabilityOptionsChoiceType {
	#[doc = r#"Available options for MissionPlanNavigation. List size for this element is based on "Select All That Apply" condition."#]
	MissionPlanNavigation(Vec<crate::v2_5::enums::PlannedNavigationCapabilityOptionEnum>),
	#[doc = r#"Available options for FixedPositionNavigation. List size for this element is based on "Select All That Apply" condition."#]
	FixedPositionNavigation(Vec<crate::v2_5::enums::FixedPositionNavigationCapabilityOptionEnum>),
	#[doc = r#"Available options for SlavedNavigation. List size for this element is based on "Select All That Apply" condition."#]
	SlavedNavigation(Vec<crate::v2_5::enums::SlavedNavigationCapabilityOptionEnum>),
	#[doc = r#"Available options for AutopilotNavigation. List size for this element is based on "Select All That Apply" condition. ."#]
	AutoPilotNavigation(Vec<crate::v2_5::enums::AutopilotNavigationCapabilityOptionEnum>),
	#[doc = r#"Available options for OwnshipNavigation. List size for this element is based on "Select All That Apply" condition."#]
	OwnshipNavigation(Vec<crate::v2_5::enums::OwnshipNavigationCapabilityOptionEnum>),
	#[doc = r#"Available options for OwnshipNavigation. List size for this element is based on "Select All That Apply" condition."#]
	LoiterNavigation(Vec<crate::v2_5::enums::LoiterNavigationCapabilityOptionEnum>),
}
choice_convert_impls! {
	NavigationCapabilityOptionsChoiceType - NavigationCapabilityOptionsChoiceTypeSerde
	MissionPlanNavigation,
	FixedPositionNavigation,
	SlavedNavigation,
	AutoPilotNavigation,
	OwnshipNavigation,
	LoiterNavigation,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NavigationChoiceTypeSerde")]
#[serde(try_from = "NavigationChoiceTypeSerde")]
pub enum NavigationChoiceType {
	#[doc = r#"Indicates navigation by connecting to and following a kinematic plan."#]
	MissionPlanNavigation(crate::v2_5::types::RoutePlanReferenceType),
	#[doc = r#"Indicates navigation to a static location. Example use of this choice is to command a platform to "fly to this point.""#]
	FixedPositionNavigation(TargetType),
	#[doc = r#"Indicates navigation to a location/object and to maintain commanded navigation even if the object becomes kinetic. Additionally, this choice indicates navigation through means of services/capabilities. For both uses of this choice, an offset value can be applied.  Example use of this choice is to command a platform to "navigate to this entity and maintain a certain distance from it once arrived.""#]
	SlavedNavigation(SlavedNavigationType),
	#[doc = r#"Indicates commandable autopilot navigation options."#]
	AutoPilotNavigation(crate::v2_5::types::AutoPilotNavigationType),
	#[doc = r#"Indicates commandable autonomous navigation options."#]
	OwnshipNavigation(OwnshipNavigationType),
	#[doc = r#"Indicates commandable loiter navigation."#]
	LoiterNavigation(LoiterType),
}
choice_convert_impls! {
	NavigationChoiceType - NavigationChoiceTypeSerde
	MissionPlanNavigation,
	FixedPositionNavigation,
	SlavedNavigation,
	AutoPilotNavigation,
	OwnshipNavigation,
	LoiterNavigation,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NavigationCommandTypeSerde")]
#[serde(try_from = "NavigationCommandTypeSerde")]
pub enum NavigationCommandType {
	#[doc = r#"Indicates a new invocation of a Navigation Capability.  Generally, if accepted, the command will result in one or more new Navigation Activities being created and reported via the NavigationActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::NavigationCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Flight Activity (which was previously reported via the FlightActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent FlightActivity messages."#]
	Activity(crate::v2_5::types::NavigationActivityCommandType),
}
choice_convert_impls! {
	NavigationCommandType - NavigationCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"Indicates the network endpoint (IP address) and its related network information."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NetworkEndpointTypeSerde")]
#[serde(try_from = "NetworkEndpointTypeSerde")]
pub enum NetworkEndpointType {
	#[doc = r#"Indicates an IPv4 network endpoint and its related network information."#]
	Ipv4Endpoint(crate::v2_5::types::Ipv4EndpointType),
	#[doc = r#"Indicates an IPv6 network endpoint and its related network information."#]
	Ipv6Endpoint(crate::v2_5::types::Ipv6EndpointType),
}
choice_convert_impls! {
	NetworkEndpointType - NetworkEndpointTypeSerde
	Ipv4Endpoint,
	Ipv6Endpoint,
}

#[doc = r#"NITF ImageSubheader Image Identifier 2, defined herein for either non-IPON-compliant NITF producers or IPON-compliant NITF producers, mutually exclusively."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "NitfIid2ChoiceTypeSerde")]
#[serde(try_from = "NitfIid2ChoiceTypeSerde")]
pub enum NitfIid2ChoiceType {
	#[doc = r#"NITF ImageSubheader Image Identifier 2 for non-IPON-compliant NITF product producers. Mutually exclusive from the IPON_IID2 element. This field can contain the identification of additional information about the image. Type is ECS-A, default is all spaces."#]
	NonIponIid2(crate::v2_5::common::VisibleStringLength80Type),
	#[doc = r#"NITF ImageSubheader Image Identifier 2 aka Tactical Image ID for IPON-compliant NITF product producers. Mutually exclusive from the NonIPON_IID2 element. This field contains the identification of additional information about the image, all mandatory for IPON-compliant NITF producers. 80-character field with first 40 chars mandated by the IPON and the last 40 set to all spaces (by the platform). Base types are BCS-A, but only the few fields below are defined by the tasking agency, therefore extant in UCI; the rest must be generated by the platform itself. Ref STDI-0005 (IPON)."#]
	IponIid2(crate::v2_5::types::NitfIponIid2Type),
}
choice_convert_impls! {
	NitfIid2ChoiceType - NitfIid2ChoiceTypeSerde
	NonIponIid2,
	IponIid2,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ObjectKinematicsChoiceTypeSerde")]
#[serde(try_from = "ObjectKinematicsChoiceTypeSerde")]
pub enum ObjectKinematicsChoiceType {
	#[doc = r#"Indicates the inertial state of the object."#]
	InertialState(Vec<crate::v2_5::types::InertialStateType>),
	#[doc = r#"Indicates one of the four types of kinematics options to describe the object's motion."#]
	OrbitKinematics(crate::v2_5::types::OrbitKinematicsType),
}
choice_convert_impls! {
	ObjectKinematicsChoiceType - ObjectKinematicsChoiceTypeSerde
	InertialState,
	OrbitKinematics,
}

#[doc = r#"Allows for a sibling operational constraint to be weighted by a discrete value or range threshold."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpConstraintWeightingValueTypeSerde")]
#[serde(try_from = "OpConstraintWeightingValueTypeSerde")]
pub enum OpConstraintWeightingValueType {
	#[doc = r#"Defines a specific percentage to which the operational constraint should be taken into account."#]
	Discrete(crate::v2_5::common::PercentType),
	#[doc = r#"Defines a threshold range to which the operational constraint should be taken into account."#]
	Range(crate::v2_5::types::PercentRangeType),
	#[doc = r#"Defines high, medium, low values to which the operational constraint should be taken into account."#]
	Likelihood(crate::v2_5::enums::LikelihoodEnum),
}
choice_convert_impls! {
	OpConstraintWeightingValueType - OpConstraintWeightingValueTypeSerde
	Discrete,
	Range,
	Likelihood,
}

#[doc = r#"A list of unique ID indicating the op type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpIdChoiceTypeSerde")]
#[serde(try_from = "OpIdChoiceTypeSerde")]
pub enum OpIdChoiceType {
	#[doc = r#"Indicates the unique identifier of the OpLine."#]
	OpLineId(crate::v2_5::types::OpLineIdType),
	#[doc = r#"Indicates the unique identifier of the OpPoint."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Indicates the unique identifier of the OpZone."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"Indicates the unique identifier of the OpVolume."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
	#[doc = r#"Indicates the unique identifier of the OpRouting."#]
	OpRoutingId(crate::v2_5::types::OpRoutingIdType),
}
choice_convert_impls! {
	OpIdChoiceType - OpIdChoiceTypeSerde
	OpLineId,
	OpPointId,
	OpZoneId,
	OpVolumeId,
	OpRoutingId,
}

#[doc = r#"Container object for the different types of OpPoint*Enums.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpPointCategoriesTypeSerde")]
#[serde(try_from = "OpPointCategoriesTypeSerde")]
pub enum OpPointCategoriesType {
	#[doc = r#"A set of commonly used point types."#]
	General(crate::v2_5::enums::OpPointGeneralEnum),
	#[doc = r#"A set of navigation or weapon hazard point types."#]
	Hazard(crate::v2_5::enums::OpPointHazardEnum),
	#[doc = r#"A set of navigation destination point types."#]
	Reference(crate::v2_5::enums::OpPointReferenceEnum),
	#[doc = r#"A set of navigation station point types."#]
	Station(crate::v2_5::enums::OpPointStationEnum),
	#[doc = r#"Emergency Point and associated reference types."#]
	Emergency(crate::v2_5::enums::OpPointEmergencyEnum),
}
choice_convert_impls! {
	OpPointCategoriesType - OpPointCategoriesTypeSerde
	General,
	Hazard,
	Reference,
	Station,
	Emergency,
}

#[doc = r#"Container object for the different types of OpPoint*Enums.  A separate enum applies to each of the choice types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpPointCategoriesUniqueDataTypeSerde")]
#[serde(try_from = "OpPointCategoriesUniqueDataTypeSerde")]
pub enum OpPointCategoriesUniqueDataType {
	#[doc = r#"Emergency Point and associated reference types."#]
	Emergency(crate::v2_5::types::EmergencyReferencePointType),
	#[doc = r#"Hazard Point and associated reference types."#]
	Hazard(crate::v2_5::types::Link16HazardType),
}
choice_convert_impls! {
	OpPointCategoriesUniqueDataType - OpPointCategoriesUniqueDataTypeSerde
	Emergency,
	Hazard,
}

#[doc = r#"Choice of either relative or geospatial position of the OpPoint."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpPointChoiceTypeSerde")]
#[serde(try_from = "OpPointChoiceTypeSerde")]
pub enum OpPointChoiceType {
	#[doc = r#"This element indicates the geospatial position of the OpPoint.  Time data is understood to be time of last position observation/measurement."#]
	Point(crate::v2_5::types::OpPointPositionType),
	#[doc = r#"The Relative position of the OpPoint.  The reference frame this is relative to is specified in a ReferenceFrames message."#]
	RelativePoint(crate::v2_5::types::Point2DRelativeType),
}
choice_convert_impls! {
	OpPointChoiceType - OpPointChoiceTypeSerde
	Point,
	RelativePoint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpPointReferenceTypeSerde")]
#[serde(try_from = "OpPointReferenceTypeSerde")]
pub enum OpPointReferenceType {
	#[doc = r#"The state of an object when an emergency is reported. This data is stored  to keep data about an object with an emergency after the object is dropped in the event that the lifetime of the emergency extends past the lifetime of the object."#]
	StoredObject(GeoLocatedStoredObjectType),
	#[doc = r#"The state of an object when an emergency is reported. This data is referenced when the lifetime of the object will persist for the duration of the emergency being reported."#]
	StoredObjectRef(GeoLocatedObjectType),
	#[doc = r#"Object used when not known in UCI."#]
	DataLinkObject(crate::v2_5::types::DataLinkIdentifierPet),
}
choice_convert_impls! {
	OpPointReferenceType - OpPointReferenceTypeSerde
	StoredObject,
	StoredObjectRef,
	DataLinkObject,
}

#[doc = r#"An operational volume comprises a three dimensional region of space."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpVolumeTypeSerde")]
#[serde(try_from = "OpVolumeTypeSerde")]
pub enum OpVolumeType {
	#[doc = r#"This element describes a 3-dimensional geometric volume (e.g. sphere, cone, etc.) using kinematics to describe the zone's position in space."#]
	GeometricVolume(crate::v2_5::types::GeometricVolumeType),
	#[doc = r#"Describes a 3-dimensional WGS-84 ellipsoid. For example, geocentric volumes can be used to include space objects in keep-in zones or to exclude space objects in keep-out zones."#]
	GeocentricVolume(crate::v2_5::types::GeocentricVolumeType),
	#[doc = r#"Indicates an orbital regime which are mostly altitude-based in which the space object will reside including optional indication of the class of orbits as secondary descriptions."#]
	OrbitRegime(crate::v2_5::types::OrbitRegimeType),
	#[doc = r#"Indicates an orbit altitude range that describes a region in space."#]
	OrbitAltitude(crate::v2_5::enums::OrbitAltitudeEnum),
	#[doc = r#"This element includes other qualitative types of regions in space such as space weather related zones."#]
	Qualitative(crate::v2_5::enums::OrbitQualitativeEnum),
}
choice_convert_impls! {
	OpVolumeType - OpVolumeTypeSerde
	GeometricVolume,
	GeocentricVolume,
	OrbitRegime,
	OrbitAltitude,
	Qualitative,
}

#[doc = r#"Container for parameters that are unique to a specific enumeration in OpZoneCategoryEnum.  For example, KeepIn enumeration can have amplifying information such as entry and exit restrictions of the zone."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OpZoneCategoryTypeSerde")]
#[serde(try_from = "OpZoneCategoryTypeSerde")]
pub enum OpZoneCategoryType {
	#[doc = r#"Defines boundaries which applicable Systems can only enter and exit through defined edges."#]
	ConstrainedEntryExit(crate::v2_5::types::ConstrainedEntryExitType),
	#[doc = r#"Indicates that the OpZone can be a zone filter type."#]
	FilterArea(Vec<crate::v2_5::types::OpZoneFilterAreaPet>),
	#[doc = r#"Indicates that the OpZone is a jamming control zone."#]
	Jamming(crate::v2_5::types::OpZoneJammingType),
	#[doc = r#"Defines boundaries to which applicable Systems must stay inside."#]
	KeepIn(crate::v2_5::types::IngressEgressType),
	#[doc = r#"Data defining a missile type, related track, and source of launch position."#]
	MissileLaunchPoint(crate::v2_5::types::OpZoneMissileDataType),
	#[doc = r#"Defines areas where strike impact is restricted.  Does not restrict the launch of weapons."#]
	NoFire(crate::v2_5::types::OpZoneNoFireType),
	#[doc = r#"Defines area where flight is restricted.  Equivalent to MIL-STD-6016 restricted zone."#]
	NoFly(crate::v2_5::types::OpZoneNoFlyType),
	#[doc = r#"Defines the OpZone boundary as an Order of Battle."#]
	OrderOfBattle(crate::v2_5::enums::OrderOfBattleEnum),
	#[doc = r#"Defines vehicle configuration parameters that should change based on the planned location of a vehicle."#]
	VehicleConfiguration(crate::v2_5::types::VehicleCommandDataType),
	#[doc = r#"Set of restricted weapons that cannot be used against a target type and or in a zone."#]
	WeaponRestriction(crate::v2_5::types::OpZoneWeaponRestrictionType),
	#[doc = r#"Defines area of weather conditions with potential of mission impact."#]
	WeatherConditions(crate::v2_5::types::OpZoneWeatherType),
}
choice_convert_impls! {
	OpZoneCategoryType - OpZoneCategoryTypeSerde
	ConstrainedEntryExit,
	FilterArea,
	Jamming,
	KeepIn,
	MissileLaunchPoint,
	NoFire,
	NoFly,
	OrderOfBattle,
	VehicleConfiguration,
	WeaponRestriction,
	WeatherConditions,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OperatorNotificationActionTypeSerde")]
#[serde(try_from = "OperatorNotificationActionTypeSerde")]
pub enum OperatorNotificationActionType {
	#[doc = r#"List of actions the originator of the alert is going to execute. No operator response is expected."#]
	AutonomousActions(Vec<crate::v2_5::types::OperatorActionAutonomousType>),
	#[doc = r#"List of actions the originator of the alert can take. The originator expects to receive one of these actions in a subsequent OperatorActionCommand."#]
	ControlledActions(crate::v2_5::types::OperatorActionControlledType),
}
choice_convert_impls! {
	OperatorNotificationActionType - OperatorNotificationActionTypeSerde
	AutonomousActions,
	ControlledActions,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OperatorReferenceTypeSerde")]
#[serde(try_from = "OperatorReferenceTypeSerde")]
pub enum OperatorReferenceType {
	#[doc = r#"Indicates the ID of a specific Operator."#]
	OperatorId(crate::v2_5::types::OperatorIdType),
	#[doc = r#"Indicates any Operator performing the associated Role."#]
	OperatorRoleId(crate::v2_5::types::OperatorRoleIdType),
	#[doc = r#"Indicates any Operator associated with the specified station. This could be a console, workstation or some other location."#]
	OperatorStation(crate::v2_5::types::OperatorStationIdentifierType),
	#[doc = r#"Indicates any Operator associated with the specified System or Service."#]
	OperatorSystem(crate::v2_5::types::SystemServiceType),
}
choice_convert_impls! {
	OperatorReferenceType - OperatorReferenceTypeSerde
	OperatorId,
	OperatorRoleId,
	OperatorStation,
	OperatorSystem,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitActivityPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "OrbitActivityPlanCommandIdChoiceTypeSerde")]
pub enum OrbitActivityPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the OrbitActivityPlanCommand associated with the OrbitActivityPlan."#]
	OrbitActivityPlanCommandId(crate::v2_5::types::OrbitActivityPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the OrbitActivityPlanValidationCommand associated with the OrbitActivityPlan."#]
	OrbitActivityPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the OrbitActivityPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the OrbitActivityPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	OrbitActivityPlanCommandIdChoiceType - OrbitActivityPlanCommandIdChoiceTypeSerde
	OrbitActivityPlanCommandId,
	OrbitActivityPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"This type provides the details of an on-orbit event which results in physical damage, whether in whole or in part, of an object or multiple objects."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitBreakupEventDetailsTypeSerde")]
#[serde(try_from = "OrbitBreakupEventDetailsTypeSerde")]
pub enum OrbitBreakupEventDetailsType {
	#[doc = r#"Indicates the unique ID of a previously identified orbital conjunction that is the basis for this debris estimate request."#]
	OrbitalConjunctionId(crate::v2_5::types::OrbitalConjunctionIdType),
	#[doc = r#"Indicates details of an identified orbital collision that is the basis for this orbital debris estimate."#]
	Collision(crate::v2_5::types::OrbitCollisionType),
	#[doc = r#"Characterizes a space based object breaking into pieces as a result of an internal explosion due to unvented fuel, overheated batteries or other causes."#]
	SingleObjectBreakup(crate::v2_5::types::OrbitObjectBreakupType),
}
choice_convert_impls! {
	OrbitBreakupEventDetailsType - OrbitBreakupEventDetailsTypeSerde
	OrbitalConjunctionId,
	Collision,
	SingleObjectBreakup,
}

#[doc = r#"Indicates a number of specific maneuvers to reach a new orbit."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitChangeChoiceTypeSerde")]
#[serde(try_from = "OrbitChangeChoiceTypeSerde")]
pub enum OrbitChangeChoiceType {
	#[doc = r#"Indicates a specific new orbit, described in classic orbital elements (COE), to change to.  The maneuver to reach the new orbit isn't specified; the orbit specified here is the steady-state post maneuver orbit."#]
	SpecificOrbit(crate::v2_5::types::CoeOrbitType),
	#[doc = r#"Indicates a specific orbital position to move to, expressed as a spherical volume.  The maneuver to reach the volume and the resulting orbit after reaching the volume aren't specified."#]
	SpecificPosition(crate::v2_5::types::OrbitalVolumeType),
	#[doc = r#"Indicates a specific resident space object (RSO) to rendezvous with.  Rendezvous doesn't imply proximity operations, docking, etc.  The maneuver to achieve rendezvous and the resulting orbit after achieving it aren't specified."#]
	Rendezvous(crate::v2_5::types::RsoApproachType),
	#[doc = r#"Indicates a specific resident space object (RSO) to initiate proximity operations around.  Specific objectives, specific desired maneuvers, etc. after entering proximity operations aren't specified.  The maneuver/rendezvous necessary to initiate proximity operations isn't specified.

Proximity operations means that two "neighboring" vehicles maneuver to affect their relative states.  It requires a precise match of orbital velocities and position vectors allowing them to remain at a constant distance through orbital station-keeping."#]
	ProximityOperations(crate::v2_5::types::ProximityOperationsType),
	#[doc = r#"Indicates the orbital line of longitude to change to.  This choice is only applicable for geo-stationary and some geo-synchronous orbits.    The maneuver to reach the longitude and the resulting orbit after reaching it aren't specified."#]
	Longitude(crate::v2_5::common::AngleType),
	#[doc = r#"Indicates the orbital semi-major axis to change to.   For a circular orbit, this is equivalent to orbital altitude.   The maneuver to reach the new semi-major axis and the resulting orbit after reaching it aren't specified."#]
	SemimajorAxis(crate::v2_5::common::DistanceType),
	#[doc = r#"Indicates the orbital inclination or tilt  to change to.  The maneuver to reach the inclination and the resulting orbit after reaching it aren't specified."#]
	Inclination(crate::v2_5::common::AngleHalfPositiveType),
	#[doc = r#"Indicates the orbital shape or eccentricity to change to.    The maneuver to reach the eccentricity and the resulting orbit after reaching it aren't specified."#]
	Eccentricity(f64),
	#[doc = r#"Indicates a specific new orbit, described in racetrack parameters, to change to. This choice is only applicable for geo-stationary and some geo-synchronous orbits. The manuerver to reach the racetrack orbit isn't specified."#]
	RaceTrack(crate::v2_5::types::RaceTrackOrbitType),
}
choice_convert_impls! {
	OrbitChangeChoiceType - OrbitChangeChoiceTypeSerde
	SpecificOrbit,
	SpecificPosition,
	Rendezvous,
	ProximityOperations,
	Longitude,
	SemimajorAxis,
	Inclination,
	Eccentricity,
	RaceTrack,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitDurationTypeSerde")]
#[serde(try_from = "OrbitDurationTypeSerde")]
pub enum OrbitDurationType {
	#[doc = r#"Required amount of time to be spent in the loiter pattern."#]
	Time(chrono::TimeDelta),
	#[doc = r#"Required number of laps to be spent in the loiter patter."#]
	NumberOfOrbits(u32),
}
choice_convert_impls! {
	OrbitDurationType - OrbitDurationTypeSerde
	Time,
	NumberOfOrbits,
}

#[doc = r#"Defines choice for replacement or modification of an orbit kinematics sequence."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitKinematicsSequenceReplaceOrModifyChoiceTypeSerde")]
#[serde(try_from = "OrbitKinematicsSequenceReplaceOrModifyChoiceTypeSerde")]
pub enum OrbitKinematicsSequenceReplaceOrModifyChoiceType {
	#[doc = r#"Indicates a complete replacement of an orbit kinematics sequence."#]
	ReplaceOrbitKinematicsSequence(crate::v2_5::types::OrbitKinematicsSequenceType),
	#[doc = r#"Indicates a modification of an orbit kinematics sequence."#]
	ModifyOrbitKinematicsSequence(crate::v2_5::types::OrbitKinematicsSequenceModificationDetailsType),
}
choice_convert_impls! {
	OrbitKinematicsSequenceReplaceOrModifyChoiceType - OrbitKinematicsSequenceReplaceOrModifyChoiceTypeSerde
	ReplaceOrbitKinematicsSequence,
	ModifyOrbitKinematicsSequence,
}

#[doc = r#"Defines the source from which to get Orbit Kinematics."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitKinematicsSourceTypeSerde")]
#[serde(try_from = "OrbitKinematicsSourceTypeSerde")]
pub enum OrbitKinematicsSourceType {
	#[doc = r#"Via the OrbitPlanID of a particular OrbitPlan."#]
	ByPlanId(crate::v2_5::types::OrbitPlanIdType),
	#[doc = r#"Via a specific Element Set."#]
	ByElementSet(OrbitalElementSetSourceType),
	#[doc = r#"Via a specific Ephemeris Source."#]
	ByEphemeris(OrbitalEphemerisSourceType),
}
choice_convert_impls! {
	OrbitKinematicsSourceType - OrbitKinematicsSourceTypeSerde
	ByPlanId,
	ByElementSet,
	ByEphemeris,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "OrbitPlanCommandIdChoiceTypeSerde")]
pub enum OrbitPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the OrbitPlanCommand which the OrbitPlan originated from."#]
	OrbitPlanCommandId(crate::v2_5::types::OrbitPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the OrbitPlanValidationCommand which the OrbitPlan originated from."#]
	OrbitPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand which the MissionPlan originated from."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand which the MissionPlan originated from."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	OrbitPlanCommandIdChoiceType - OrbitPlanCommandIdChoiceTypeSerde
	OrbitPlanCommandId,
	OrbitPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"Defines choice for location to insert orbit kinematics sequence."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitSequenceInsertionPositionChoiceTypeSerde")]
#[serde(try_from = "OrbitSequenceInsertionPositionChoiceTypeSerde")]
pub enum OrbitSequenceInsertionPositionChoiceType {
	#[doc = r#"Indicates an insertion at the start of the orbit kinematics sequences."#]
	InsertAtStart(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the orbit kinematics sequence after which to insert the additional orbit kinematics sequences."#]
	InsertAfterOrbitKinematicsSequenceId(crate::v2_5::types::OrbitKinematicsSequenceIdType),
}
choice_convert_impls! {
	OrbitSequenceInsertionPositionChoiceType - OrbitSequenceInsertionPositionChoiceTypeSerde
	InsertAtStart,
	InsertAfterOrbitKinematicsSequenceId,
}

#[doc = r#"Indicates the specific details of the object that is to be analyzed for close approaches."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalAnalysisObjectTypeSerde")]
#[serde(try_from = "OrbitalAnalysisObjectTypeSerde")]
pub enum OrbitalAnalysisObjectType {
	#[doc = r#"Indicates the orbital object that is the subject of the analysis."#]
	SpecificObject(crate::v2_5::types::OrbitalObjectKinematicsSourceType),
	#[doc = r#"Indicates the ID of the Order Of Battle that contains the record associated with the specific object that is the subject of the analysis."#]
	OrderOfBattleId(crate::v2_5::types::OrderOfBattleIdType),
	#[doc = r#"Indicates the unique ID of an orbital debris cloud that is the subject of the analysis."#]
	DebrisCloudId(crate::v2_5::types::OrbitalDebrisCloudIdType),
}
choice_convert_impls! {
	OrbitalAnalysisObjectType - OrbitalAnalysisObjectTypeSerde
	SpecificObject,
	OrderOfBattleId,
	DebrisCloudId,
}

#[doc = r#"Indicates the collection of Orbital Debris estimate information."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalDebrisEstimateTypeSerde")]
#[serde(try_from = "OrbitalDebrisEstimateTypeSerde")]
pub enum OrbitalDebrisEstimateType {
	#[doc = r#"Indicates the ID of the file associated with the specific debris estimate information."#]
	FileMetadataId(crate::v2_5::types::FileMetadataIdType),
	#[doc = r#"Details the orbital debris cloud included in an orbital debris estimate. For single-object breakup events, there should only be one debris cloud. For collisions that involve 2 or more object, there will be a debris cloud for each object involved in the collision."#]
	DebrisCloud(Vec<crate::v2_5::types::OrbitalDebrisCloudType>),
}
choice_convert_impls! {
	OrbitalDebrisEstimateType - OrbitalDebrisEstimateTypeSerde
	FileMetadataId,
	DebrisCloud,
}

#[doc = r#"Indicates the source of the element set kinematics data."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalElementSetSourceTypeSerde")]
#[serde(try_from = "OrbitalElementSetSourceTypeSerde")]
pub enum OrbitalElementSetSourceType {
	#[doc = r#"The unique identifier that correspond to a System's orbital element set (TLE)."#]
	SystemElementSetId(crate::v2_5::types::SystemOrbitalElementSetIdType),
	#[doc = r#"The unique identifier that correspond to an Entity's orbital element set (TLE)."#]
	EntityElementSetId(crate::v2_5::types::EntityOrbitalElementSetIdType),
	#[doc = r#"Indicates the default or "catalog" orbital element set (also known as two line element or TLE) for the satellite."#]
	ExplicitElementSet(crate::v2_5::types::TleBaseType),
}
choice_convert_impls! {
	OrbitalElementSetSourceType - OrbitalElementSetSourceTypeSerde
	SystemElementSetId,
	EntityElementSetId,
	ExplicitElementSet,
}

#[doc = r#"Indicates the ephemeris expressed in one of several orbital kinematics standards."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalEphemerisChoiceTypeSerde")]
#[serde(try_from = "OrbitalEphemerisChoiceTypeSerde")]
pub enum OrbitalEphemerisChoiceType {
	#[doc = r#"Indicates ephemeris expressed in terms of a standard frame centered on a celestial object."#]
	StandardFrame(OrbitalKinematicsStandardEphemerisType),
	#[doc = r#"Indicates ephemeris expressed relative to a reference object that is in orbit around a celestial object."#]
	OrbitingObjectRelative(crate::v2_5::types::OrbitalKinematicsRelativeEphemerisType),
}
choice_convert_impls! {
	OrbitalEphemerisChoiceType - OrbitalEphemerisChoiceTypeSerde
	StandardFrame,
	OrbitingObjectRelative,
}

#[doc = r#"Indicates the source of the ephemeris kinematics data."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalEphemerisSourceTypeSerde")]
#[serde(try_from = "OrbitalEphemerisSourceTypeSerde")]
pub enum OrbitalEphemerisSourceType {
	#[doc = r#"This element references the ephemeris ID."#]
	SystemEphemerisId(crate::v2_5::types::SystemOrbitalEphemerisIdType),
	#[doc = r#"The ID type for UCI IDs that correspond to an ephemeris."#]
	EntityEphemerisId(crate::v2_5::types::EntityOrbitalEphemerisIdType),
	#[doc = r#"Indicates ephemeris expressed in terms of a standard frame centered on a celestial object."#]
	OrbitalEphemeris(OrbitalEphemerisChoiceType),
}
choice_convert_impls! {
	OrbitalEphemerisSourceType - OrbitalEphemerisSourceTypeSerde
	SystemEphemerisId,
	EntityEphemerisId,
	OrbitalEphemeris,
}

#[doc = r#"Indicates the kinematics expressed in one of several orbital kinematics standards."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalKinematicsChoiceTypeSerde")]
#[serde(try_from = "OrbitalKinematicsChoiceTypeSerde")]
pub enum OrbitalKinematicsChoiceType {
	#[doc = r#"Indicates kinematics expressed in terms of a standard frame centered on a celestial object."#]
	StandardFrame(OrbitalKinematicsStandardFrameChoiceType),
	#[doc = r#"Indicates kinematics expressed relative to a reference object that is in orbit around a celestial object."#]
	OrbitingObjectRelative(crate::v2_5::types::OrbitalKinematicsObjectRelativeType),
}
choice_convert_impls! {
	OrbitalKinematicsChoiceType - OrbitalKinematicsChoiceTypeSerde
	StandardFrame,
	OrbitingObjectRelative,
}

#[doc = r#"Provides the choice of orbital kinematics reference frames."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalKinematicsFrameChoiceTypeSerde")]
#[serde(try_from = "OrbitalKinematicsFrameChoiceTypeSerde")]
pub enum OrbitalKinematicsFrameChoiceType {
	#[doc = r#"Indicates the orbital element set for the satellite."#]
	OrbitalElements(crate::v2_5::types::TleWithParametersType),
	#[doc = r#"Indicates kinematic vector or vectors in the Earth-Centered Inertial (ECI) J2000 (ECI-J2K) Reference System."#]
	J2k(crate::v2_5::types::J2kKinematicsType),
	#[doc = r#"Indicates kinematic vector or vectors in the Geocentric Celestial Reference System (GCRS)."#]
	Gcrs(crate::v2_5::types::GcrsKinematicsType),
	#[doc = r#"Indicates kinematic vector or vectors in the Barycentric Celestial Reference System (BCRS)."#]
	Bcrs(crate::v2_5::types::BcrsKinematicsType),
}
choice_convert_impls! {
	OrbitalKinematicsFrameChoiceType - OrbitalKinematicsFrameChoiceTypeSerde
	OrbitalElements,
	J2k,
	Gcrs,
	Bcrs,
}

#[doc = r#"Indicates ephemeris expressed in terms of a standard reference frame centered on a celestial object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalKinematicsStandardEphemerisTypeSerde")]
#[serde(try_from = "OrbitalKinematicsStandardEphemerisTypeSerde")]
pub enum OrbitalKinematicsStandardEphemerisType {
	#[doc = r#"Indicates kinematic vector(s) in the Earth-Centered Inertial (ECI) J2000 (ECI-J2K) reference system."#]
	J2kStateVector(Vec<crate::v2_5::types::J2kKinematicsType>),
	#[doc = r#"Indicates kinematic vector(s) in the Geocentric Celestial Reference System (GCRS)."#]
	GcrsStateVector(Vec<crate::v2_5::types::GcrsKinematicsType>),
	#[doc = r#"Indicates kinematic vector(s) in the barycentric celestial reference system (BCRS)."#]
	BcrsStateVector(Vec<crate::v2_5::types::BcrsKinematicsType>),
}
choice_convert_impls! {
	OrbitalKinematicsStandardEphemerisType - OrbitalKinematicsStandardEphemerisTypeSerde
	J2kStateVector,
	GcrsStateVector,
	BcrsStateVector,
}

#[doc = r#"Provides the choice of kinematics in terms of a standard coordinate reference frame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalKinematicsStandardFrameChoiceTypeSerde")]
#[serde(try_from = "OrbitalKinematicsStandardFrameChoiceTypeSerde")]
pub enum OrbitalKinematicsStandardFrameChoiceType {
	#[doc = r#"Indicates kinematic vector or vectors in the Earth-Centered Inertial (ECI) J2000 (ECI-J2K) Reference System."#]
	J2k(crate::v2_5::types::J2kKinematicsType),
	#[doc = r#"Indicates kinematic vector or vectors in the Geocentric Celestial Reference System (GCRS)."#]
	Gcrs(crate::v2_5::types::GcrsKinematicsType),
	#[doc = r#"Indicates kinematic vector or vectors in the Barycentric Celestial Reference System (BCRS)."#]
	Bcrs(crate::v2_5::types::BcrsKinematicsType),
}
choice_convert_impls! {
	OrbitalKinematicsStandardFrameChoiceType - OrbitalKinematicsStandardFrameChoiceTypeSerde
	J2k,
	Gcrs,
	Bcrs,
}

#[doc = r#"Indicates the choice between two-line element kinematic data or state vector (ECI J2K) kinematic data."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalKinematicsTleSvTypeSerde")]
#[serde(try_from = "OrbitalKinematicsTleSvTypeSerde")]
pub enum OrbitalKinematicsTleSvType {
	#[doc = r#"Indicates the object's kinematic state vector or vectors in the Earth-Centered Inertial (ECI) J2000 (ECI-J2K) reference system."#]
	StateVector(crate::v2_5::types::J2kKinematicsType),
	#[doc = r#"Indicates the orbital element set (also known as two line element or TLE) of the object."#]
	Tle(crate::v2_5::types::TleBaseType),
}
choice_convert_impls! {
	OrbitalKinematicsTleSvType - OrbitalKinematicsTleSvTypeSerde
	StateVector,
	Tle,
}

#[doc = r#"Indicates orbital surveillance instructions and information to the sensor to enable appropriate sensor set-up and data collection to meet the orbital surveillance collection need."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceCollectionRequirementsTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceCollectionRequirementsTypeSerde")]
pub enum OrbitalSurveillanceCollectionRequirementsType {
	#[doc = r#"Indicates collection requirements to perform a metric collection sensor task."#]
	MetricCollection(crate::v2_5::types::MetricCollectionType),
	#[doc = r#"Indicates collection requirements to perform a search sensor task, to include the minimum and maximum speed of the target."#]
	Search(crate::v2_5::types::SpeedRangeType),
	#[doc = r#"Indicates collection requirements to perform an orbit determination sensor task."#]
	OrbitDetermination(crate::v2_5::types::OrbitAccuracyType),
	#[doc = r#"Indicates collection requirements to perform a sensor characterization task."#]
	Characterization(SensorCharacterizationChoiceType),
	#[doc = r#"Indicates collection requirements to perform a multi-object sensor task."#]
	MultiObject(crate::v2_5::types::MultiObjectType),
	#[doc = r#"Indicates collection requirements to perform a maneuver detection sensor task."#]
	ManeuverDetection(crate::v2_5::types::ManeuverDetectionType),
	#[doc = r#"Indicates collection requirements to perform a deployment detection sensor task."#]
	DeploymentDetection(crate::v2_5::types::DeploymentDetectionType),
}
choice_convert_impls! {
	OrbitalSurveillanceCollectionRequirementsType - OrbitalSurveillanceCollectionRequirementsTypeSerde
	MetricCollection,
	Search,
	OrbitDetermination,
	Characterization,
	MultiObject,
	ManeuverDetection,
	DeploymentDetection,
}

#[doc = r#"Specifies span of time for individual collection based on duration or rotational periods of target."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceSensorMinimumCollectionRequirementsTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceSensorMinimumCollectionRequirementsTypeSerde")]
pub enum OrbitalSurveillanceSensorMinimumCollectionRequirementsType {
	#[doc = r#"Collection duration (in number of rotations)."#]
	TargetRotationalPeriods(i32),
	#[doc = r#"Collection duration (in seconds)."#]
	Time(chrono::TimeDelta),
}
choice_convert_impls! {
	OrbitalSurveillanceSensorMinimumCollectionRequirementsType - OrbitalSurveillanceSensorMinimumCollectionRequirementsTypeSerde
	TargetRotationalPeriods,
	Time,
}

#[doc = r#"Indicates the expected size of the smallest target for the task (or threshold for search) in either physical area  or apparent size appropriate to the phenomenology (e.g., radar cross section)."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceSensorMinimumSizeTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceSensorMinimumSizeTypeSerde")]
pub enum OrbitalSurveillanceSensorMinimumSizeType {
	#[doc = r#"See Base Description."#]
	RadarCrossSection(crate::v2_5::types::PercentileRcstype),
	#[doc = r#"See Base Description."#]
	VisualMagnitude(crate::v2_5::common::VisualMagnitudeType),
	#[doc = r#"Area measured in m^2."#]
	Area(crate::v2_5::common::AreaType),
	#[doc = r#"Intensity measured in watts per steradian."#]
	Intensity(f64),
}
choice_convert_impls! {
	OrbitalSurveillanceSensorMinimumSizeType - OrbitalSurveillanceSensorMinimumSizeTypeSerde
	RadarCrossSection,
	VisualMagnitude,
	Area,
	Intensity,
}

#[doc = r#"Indicates the target of the Orbital Surveillance Sensor Task."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceSensorTargetTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceSensorTargetTypeSerde")]
pub enum OrbitalSurveillanceSensorTargetType {
	#[doc = r#"Indicates that the target of the Orbital Surveillance Sensor Task is defined by a sensor point list."#]
	PointList(SensorPointListType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Sensor Task is defined by an element set cloud. This method is used currently to set up searches when limited information on a recently found object leads to non-trivial uncertainty in the estimate of the orbital parameters (target has a high likelihood of not being in the acquisition basket of a sensor receiving an orbital state estimate in a hand-off).  A tested approach for defining a search to be used by a sensor tasked for follow-up is to create a set of candidate orbital states that span the uncertainty in such a way that a sensor that attempts to acquire on each element set during a defined time window will have a high likelihood of acquiring the object."#]
	ElementSetCloud(crate::v2_5::types::ElementSetCloudType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Sensor Task is a specified object or objects or relative to a specified object."#]
	ObjectBased(crate::v2_5::types::OrbitalSurveillanceObjectsType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Sensor Task is defined by orbit kinematic data."#]
	LocationBased(crate::v2_5::types::OrbitalSurveillanceLocationTargetType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Sensor is defined by a volume specified with the sensor at the origin."#]
	SensorCentricVolume(crate::v2_5::types::SourceCoverageType),
}
choice_convert_impls! {
	OrbitalSurveillanceSensorTargetType - OrbitalSurveillanceSensorTargetTypeSerde
	PointList,
	ElementSetCloud,
	ObjectBased,
	LocationBased,
	SensorCentricVolume,
}

#[doc = r#"Indicates a SubCapability of the Orbital Surveillance Capability, the second tier in the taxonomy of Orbital Surveillance.  For Orbital Surveillance, the second tier is the desired outcome of the collection.  See enumeration annotations for further details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceSubCapabilityDetailsChoiceTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceSubCapabilityDetailsChoiceTypeSerde")]
pub enum OrbitalSurveillanceSubCapabilityDetailsChoiceType {
	#[doc = r#"Indicates a search task with an optional target speed range."#]
	Search(crate::v2_5::types::SpeedRangeType),
	#[doc = r#"Indicates a task to achieve/maintain specified orbit accuracy."#]
	OrbitAccuracy(crate::v2_5::types::OrbitAccuracyType),
	#[doc = r#"Indicates a Task to characterize an object."#]
	Characterization(CharacterizationObjectiveType),
	#[doc = r#"Indicates a task involving multiple objects."#]
	MultiObject(crate::v2_5::types::MultiObjectType),
	#[doc = r#"Indicates a task to detect maneuvers (may require more than one sensor) and near real-time processing."#]
	ManeuverDetection(crate::v2_5::types::ManeuverDetectionType),
	#[doc = r#"Indicates a task that monitors primary target to detect deployment of secondary object or objects."#]
	DeploymentDetection(crate::v2_5::types::DeploymentDetectionType),
}
choice_convert_impls! {
	OrbitalSurveillanceSubCapabilityDetailsChoiceType - OrbitalSurveillanceSubCapabilityDetailsChoiceTypeSerde
	Search,
	OrbitAccuracy,
	Characterization,
	MultiObject,
	ManeuverDetection,
	DeploymentDetection,
}

#[doc = r#"Indicates the target of the Orbital Surveillance Task as either object based, location based, or zone based."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrbitalSurveillanceTargetTypeSerde")]
#[serde(try_from = "OrbitalSurveillanceTargetTypeSerde")]
pub enum OrbitalSurveillanceTargetType {
	#[doc = r#"Indicates that the target of the Orbital Surveillance Task is a specified object or objects or relative to a specified object."#]
	ObjectBased(crate::v2_5::types::OrbitalSurveillanceObjectsType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Task is defined by orbit kinematic data."#]
	LocationBased(crate::v2_5::types::OrbitalSurveillanceLocationTargetType),
	#[doc = r#"Indicates that the target of the Orbital Surveillance Task is defined by a geocentric zone."#]
	ZoneBased(crate::v2_5::types::OrbitalSurveillanceZoneTargetType),
}
choice_convert_impls! {
	OrbitalSurveillanceTargetType - OrbitalSurveillanceTargetTypeSerde
	ObjectBased,
	LocationBased,
	ZoneBased,
}

#[doc = r#"Contains the information for the source of the OrderOfBattle."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OrderOfBattleSourceIdChoiceTypeSerde")]
#[serde(try_from = "OrderOfBattleSourceIdChoiceTypeSerde")]
pub enum OrderOfBattleSourceIdChoiceType {
	#[doc = r#"Indicates the SystemID of the System whose "reference catalog" records should-be/were used to create the OrderOfBattle record."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the unique ID of an existing "parent" OrderOfBattle record to use as the basis to create this record."#]
	OrderOfBattleId(crate::v2_5::types::OrderOfBattleIdType),
}
choice_convert_impls! {
	OrderOfBattleSourceIdChoiceType - OrderOfBattleSourceIdChoiceTypeSerde
	SystemId,
	OrderOfBattleId,
}

#[doc = r#"Encoding types for CVEnumISMCATOwnerProducer Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATOwnerProducer.xml CVE.(U) 
				  FGI, followed by all currently valid GENC trigraphs in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph.

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATOwnerProducer.xml"#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OwnerProducerChoiceTypeSerde")]
#[serde(try_from = "OwnerProducerChoiceTypeSerde")]
pub enum OwnerProducerChoiceType {
	#[doc = r#"CVEnumISMCATOwnerProducer Values"#]
	GovernmentIdentifier(crate::v2_5::enums::OwnerProducerEnum),
	#[doc = r#"North Atlantic Treaty Organization Special Words"#]
	NatoSpecialWord(crate::v2_5::common::NatoSpecialWordsType),
}
choice_convert_impls! {
	OwnerProducerChoiceType - OwnerProducerChoiceTypeSerde
	GovernmentIdentifier,
	NatoSpecialWord,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "OwnshipNavigationTypeSerde")]
#[serde(try_from = "OwnshipNavigationTypeSerde")]
pub enum OwnshipNavigationType {
	#[doc = r#"Indicates dynamic navigation without a pre-determined kinematic point, line, zone, volume or plan objective."#]
	Dynamic(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates swarm navigation."#]
	Swarm(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates team navigation."#]
	Team(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates navigation to a location defined by means other than what's listed above."#]
	Other(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	OwnshipNavigationType - OwnshipNavigationTypeSerde
	Dynamic,
	Swarm,
	Team,
	Other,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ParameterValueTypeSerde")]
#[serde(try_from = "ParameterValueTypeSerde")]
pub enum ParameterValueType {
	#[doc = r#"Current value of the configuration parameter."#]
	Value(crate::v2_5::common::VisibleString256Type),
	#[doc = r#"If this is selected, the parameter called out should be returned to its default value."#]
	ReturnToDefault(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	ParameterValueType - ParameterValueTypeSerde
	Value,
	ReturnToDefault,
}

#[doc = r#"Specify a section of a path, by time or by segments."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PathConstraintsTypeSerde")]
#[serde(try_from = "PathConstraintsTypeSerde")]
pub enum PathConstraintsType {
	#[doc = r#"Specify a segment window within a path. The absence of a begin segment indicates the window begins at the first segment within the path. The absence of an end segment indicates the window extends to the end of the path."#]
	SegmentWindow(crate::v2_5::types::SegmentRangeType),
	#[doc = r#"Specify a time window within a path. The absence of a begin time indicates the window begins at the beginning of the path. The absence of an end time indicates the window extends to the end of the path."#]
	TimeWindow(crate::v2_5::types::TimeWindowType),
}
choice_convert_impls! {
	PathConstraintsType - PathConstraintsTypeSerde
	SegmentWindow,
	TimeWindow,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PathSegmentSpeedChoiceTypeSerde")]
#[serde(try_from = "PathSegmentSpeedChoiceTypeSerde")]
pub enum PathSegmentSpeedChoiceType {
	#[doc = r#"Indicates the speed that the vehicle shall traverse the path segment."#]
	SpeedValue(crate::v2_5::types::PathSegmentSpeedValueType),
	#[doc = r#"Indicates the unitless Mach number realized at a particular speed during flight."#]
	MachValue(crate::v2_5::common::MachType),
}
choice_convert_impls! {
	PathSegmentSpeedChoiceType - PathSegmentSpeedChoiceTypeSerde
	SpeedValue,
	MachValue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PitchHoldOrConstraintChoiceTypeSerde")]
#[serde(try_from = "PitchHoldOrConstraintChoiceTypeSerde")]
pub enum PitchHoldOrConstraintChoiceType {
	#[doc = r#"Indicates navigation by pitch hold."#]
	PitchHold(crate::v2_5::common::AngleHalfType),
	#[doc = r#"Indicates navigation by updated pitch hold constraint, seen in the sibling Constraints element."#]
	PitchHoldConstraint(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	PitchHoldOrConstraintChoiceType - PitchHoldOrConstraintChoiceTypeSerde
	PitchHold,
	PitchHoldConstraint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanActivationAutonomyTypeSerde")]
#[serde(try_from = "PlanActivationAutonomyTypeSerde")]
pub enum PlanActivationAutonomyType {
	#[doc = r#"Indicates autonomous, simultaneous activation of all sub-*Plans (TaskPlan, OrbitPlan and OrbitActivityPlan for example) of a MissionPlan into the same activation state."#]
	ByMissionPlan(crate::v2_5::types::MissionPlanActivationSettingType),
	#[doc = r#"Indicates autonomous activation by sub-*Plan (RoutePlan and ActivityPlan for example) of the MissionPlan, with potentially different states for each."#]
	BySubPlan(Vec<crate::v2_5::types::SubPlanActivationSettingType>),
}
choice_convert_impls! {
	PlanActivationAutonomyType - PlanActivationAutonomyTypeSerde
	ByMissionPlan,
	BySubPlan,
}

#[doc = r#"Indicates the System or Systems which have a direct relationship with the developed plan. Complex, dispersed, and/or hierarchical C2 systems require planning and plans at multiple levels with varying purpose and detail. Therefore, plan messages must give an indication of their level, purpose and detail.  For instance, planning services at the operational level of warfare may have different details and control authorities compared to planning services at the tactical level of warfare. The PlanApplicabilityType helps clarify which systems have which roles and responsibilities with respect to the plan. See child elements for further details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanApplicabilityChoiceTypeSerde")]
#[serde(try_from = "PlanApplicabilityChoiceTypeSerde")]
pub enum PlanApplicabilityChoiceType {
	#[doc = r#"Indicates the Package and Systems which have a direct relationship with the Plan.  See annotations for the underlying type and its child elements for details."#]
	PlanPackage(crate::v2_5::types::PlanPackageApplicabilityType),
	#[doc = r#"Indicates the System or Systems which have a direct relationship with the Plan.  See annotations for the underlying type and its child elements for details."#]
	System(crate::v2_5::types::PlanApplicabilityType),
}
choice_convert_impls! {
	PlanApplicabilityChoiceType - PlanApplicabilityChoiceTypeSerde
	PlanPackage,
	System,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanReferenceIdChoiceTypeSerde")]
#[serde(try_from = "PlanReferenceIdChoiceTypeSerde")]
pub enum PlanReferenceIdChoiceType {
	#[doc = r#"Indicates a reference to a MissionPlan."#]
	MissionPlanId(crate::v2_5::types::MissionPlanIdType),
	#[doc = r#"Indicates a reference to a TaskPlan."#]
	TaskPlanId(crate::v2_5::types::TaskPlanIdType),
	#[doc = r#"Indicates a reference to an OrbitPlan."#]
	OrbitPlanId(crate::v2_5::types::OrbitPlanIdType),
	#[doc = r#"Indicates a reference to an OrbitActivityPlan."#]
	OrbitActivityPlanId(crate::v2_5::types::OrbitActivityPlanIdType),
	#[doc = r#"Indicates a reference to a RoutePlan."#]
	RoutePlanId(crate::v2_5::types::RoutePlanIdType),
	#[doc = r#"Indicates a reference to a RouteActivityPlan."#]
	RouteActivityPlanId(crate::v2_5::types::RouteActivityPlanIdType),
	#[doc = r#"Indicates a reference to a Comms allocation."#]
	CommSupportPlanId(crate::v2_5::types::CommSupportPlanIdType),
	#[doc = r#"Indicates a reference to an ActivityPlan."#]
	ActivityPlanId(crate::v2_5::types::ActivityPlanIdType),
	#[doc = r#"Indicates a reference to an EffectPlan."#]
	EffectPlanId(crate::v2_5::types::EffectPlanIdType),
	#[doc = r#"Indicates a reference to an ActionPlan."#]
	ActionPlanId(crate::v2_5::types::ActionPlanIdType),
	#[doc = r#"Indicates a reference to a ResponsePlan."#]
	ResponsePlanId(crate::v2_5::types::ResponsePlanIdType),
	#[doc = r#"Indicates a reference to a SupportPlan."#]
	SupportPlanId(crate::v2_5::types::SupportPlanIdType),
}
choice_convert_impls! {
	PlanReferenceIdChoiceType - PlanReferenceIdChoiceTypeSerde
	MissionPlanId,
	TaskPlanId,
	OrbitPlanId,
	OrbitActivityPlanId,
	RoutePlanId,
	RouteActivityPlanId,
	CommSupportPlanId,
	ActivityPlanId,
	EffectPlanId,
	ActionPlanId,
	ResponsePlanId,
	SupportPlanId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanWindowModificationTypeChoiceTypeSerde")]
#[serde(try_from = "PlanWindowModificationTypeChoiceTypeSerde")]
pub enum PlanWindowModificationTypeChoiceType {
	#[doc = r#"The new time window within which this Plan, or set of Plans, should apply."#]
	Window(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"Indicates the time offset that the existing Window for a Plan, or set of Plans, should be shifted by."#]
	TimeOffset(chrono::TimeDelta),
}
choice_convert_impls! {
	PlanWindowModificationTypeChoiceType - PlanWindowModificationTypeChoiceTypeSerde
	Window,
	TimeOffset,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanningByCaseTriggerTypeSerde")]
#[serde(try_from = "PlanningByCaseTriggerTypeSerde")]
pub enum PlanningByCaseTriggerType {
	#[doc = r#"This trigger refers to a new Capability or SubCapability which becomes available or a previously failed Capability which has been restored."#]
	CapabilityAdded(crate::v2_5::types::CapabilityTaxonomyType),
	#[doc = r#"This trigger refers to a failure of a Capability and/or SubCapability needed to perform a Requirement."#]
	CapabilityFailure(crate::v2_5::types::CapabilityTaxonomyType),
	#[doc = r#"This trigger refers to lost comms."#]
	CommsLost(crate::v2_5::types::CommsLostTriggerDataType),
	#[doc = r#"This trigger refers to a situation where the designated DMPIs exceed those initially reserved for the Task, if this creates a shortage for other assigned Tasks."#]
	DmpiOverDesignation(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to a situation where the designated DMPIs are less than those initially reserved for the task, and if weapons not designated can be used to perform previous unallocated tasks."#]
	DmpiUnderDesignation(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to a low remaining endurance condition such as low fuel or battery power. This trigger can further be specialized by the definition of the Endurance thresholds to define the trigger."#]
	EnduranceLow(crate::v2_5::types::EnduranceType),
	#[doc = r#"This trigger refers to a vehicle that has deviated from the planned route to such a degree that a replan is required."#]
	OffRoute(crate::v2_5::types::ThresholdOffRouteTriggerDataType),
	#[doc = r#"This trigger refers to the proximity of two items (Systems, Entities or other items) that exceeded a minimum geospatial separation limit as specified in OpRouting messages."#]
	ProximityConflict(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to the location of a DMPI, updated LAR calculations or other changes that have resulted in the planned release point for a weapon falling outside the weapon LAR."#]
	ReleasePointOutsideLar(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to a route that exceeded a minimum geospatial separation limit with another item or items.  The separation is between a planned/navigated route and other routes or conflicting items as specified in OpRouting messages."#]
	RouteConflict(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to vulnerability due to exposure along the route with and without suppression."#]
	RouteVulnerability(crate::v2_5::types::PlanVulnerabilityType),
	#[doc = r#"This trigger refers to a system state transition, as indicated by the SystemStatus message."#]
	SystemStateChange(crate::v2_5::types::SystemStateFilterType),
	#[doc = r#"This trigger indicates a new Requirement."#]
	RequirementAdded(crate::v2_5::types::RequirementTriggerType),
	#[doc = r#"This trigger indicates a Requirement that cannot be planned/performed because a Requirement it is dependent on cannot be planned/performed."#]
	RequirementDependencyFailed(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to a Requirement execution state transition to DROPPED, as indicated by execution status message for the Requirement (EffectStatus, TaskStatus, etc.)."#]
	RequirementDropped(crate::v2_5::types::RequirementTriggerType),
	#[doc = r#"This trigger refers to a Requirement execution state transition to FAILED, as indicated by execution status message for the Requirement (EffectStatus, TaskStatus, etc.)."#]
	RequirementFailed(crate::v2_5::types::RequirementFailedTriggerType),
	#[doc = r#"This trigger refers to a change to a Requirement that impacts an existing plan.  For example, a change of the position of the target of the Requirement."#]
	RequirementChange(crate::v2_5::types::RequirementTriggerType),
	#[doc = r#"This trigger a Requirement that is not expected to meet its timing constraints."#]
	RequirementTiming(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to an OpZone conflicts with the current plan."#]
	ZoneViolation(crate::v2_5::types::ZoneViolationTriggerDataType),
	#[doc = r#"This trigger refers to the position along a future planned/predicted/propagated orbit for a spacecraft exceeding a minimum geospatial separation limit with another item or items."#]
	OrbitConflict(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger refers to a spacecraft that has deviated from its planned orbit to such a degree that a replan is required."#]
	OffPlannedOrbit(crate::v2_5::types::ThresholdOffOrbitTriggerDataType),
	#[doc = r#"This trigger refers to a low remaining endurance condition such as low fuel or battery power. This trigger can further be specialized by setting trigger thresholds in descendant elements."#]
	SpacecraftEnduranceLow(crate::v2_5::types::SatelliteEnduranceType),
	#[doc = r#"This trigger refers to the proximity of two spacecraft (Systems, Entities or other space items) that exceeded a minimum geospatial separation limit.  Monitoring for this trigger should be based on current position of live items, not planned/predicted/propagated positions of Systems; see sibling OrbitConflict element."#]
	SpacecraftProximityConflict(crate::v2_5::common::EmptyType),
	#[doc = r#"This trigger indicates planning triggered by a Response and/or ResponsePlan."#]
	ResponseId(crate::v2_5::types::ResponseIdType),
}
choice_convert_impls! {
	PlanningByCaseTriggerType - PlanningByCaseTriggerTypeSerde
	CapabilityAdded,
	CapabilityFailure,
	CommsLost,
	DmpiOverDesignation,
	DmpiUnderDesignation,
	EnduranceLow,
	OffRoute,
	ProximityConflict,
	ReleasePointOutsideLar,
	RouteConflict,
	RouteVulnerability,
	SystemStateChange,
	RequirementAdded,
	RequirementDependencyFailed,
	RequirementDropped,
	RequirementFailed,
	RequirementChange,
	RequirementTiming,
	ZoneViolation,
	OrbitConflict,
	OffPlannedOrbit,
	SpacecraftEnduranceLow,
	SpacecraftProximityConflict,
	ResponseId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanningByResultTriggerTypeSerde")]
#[serde(try_from = "PlanningByResultTriggerTypeSerde")]
pub enum PlanningByResultTriggerType {
	#[doc = r#"Indicates a replan of one *Plan resulting in the need to replan an existing Plan of the type given by this element. List size for this element is based on "Select All That Apply" condition."#]
	ReplanRequired(Vec<crate::v2_5::enums::PlanTypeEnum>),
	#[doc = r#"This result refers to a replan which results in vulnerability changes that exceed the threshold or thresholds given in descendant elements."#]
	VulnerabilityChanged(crate::v2_5::types::PlanVulnerabilityType),
	#[doc = r#"This result refers to a case where Requirements have been unallocated as a result of triggered replanning."#]
	RequirementUnallocated(crate::v2_5::types::RequirementTriggerType),
}
choice_convert_impls! {
	PlanningByResultTriggerType - PlanningByResultTriggerTypeSerde
	ReplanRequired,
	VulnerabilityChanged,
	RequirementUnallocated,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlanningPointTypeSerde")]
#[serde(try_from = "PlanningPointTypeSerde")]
pub enum PlanningPointType {
	#[doc = r#"Specifies a specific location to use for planning.  This choice is intended for users/services, such as allocators or route planners, which have and care to specify detailed vehicle routes and position data."#]
	Specific(crate::v2_5::types::PlanningLocationType),
	#[doc = r#"Specifies an OpPoint to use for planning.  This choice is intended for cases including pre-mission planning initiated at an ingress OpPoint, the first cycle of dynamic mission planning prior to the vehicle reaching its ingress OpPoint, etc."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Specifies a future time to use to establish/calculate/determine the point to use for planning.  This choice is intended for users/services which don't necessarily have, or care to specify, detailed vehicle position data.  Allocation and planning services would then be expected to determine a location corresponding to this Time, based on the current/previous *Plan or other information."#]
	Time(chrono::DateTime<chrono::Utc>),
}
choice_convert_impls! {
	PlanningPointType - PlanningPointTypeSerde
	Specific,
	OpPointId,
	Time,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PlatformFunctionStatusCategoryTypeSerde")]
#[serde(try_from = "PlatformFunctionStatusCategoryTypeSerde")]
pub enum PlatformFunctionStatusCategoryType {
	#[doc = r#"Enumeration listing systems that can be reported for aircraft."#]
	Air(crate::v2_5::enums::PlatformFunctionAirEnum),
	#[doc = r#"Enumeration listing systems that can be reported for sea surface vehicles."#]
	SeaSurface(crate::v2_5::enums::PlatformFunctionSeaSurfaceEnum),
	#[doc = r#"Enumeration listing systems that can be reported for ground vehicles."#]
	Ground(crate::v2_5::enums::PlatformFunctionGroundEnum),
	#[doc = r#"Enumeration listing systems that can be reported for electronic warfare systems."#]
	Ew(crate::v2_5::enums::PlatformFunctionEwEnum),
}
choice_convert_impls! {
	PlatformFunctionStatusCategoryType - PlatformFunctionStatusCategoryTypeSerde
	Air,
	SeaSurface,
	Ground,
	Ew,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PmopSequenceTypeSerde")]
#[serde(try_from = "PmopSequenceTypeSerde")]
pub enum PmopSequenceType {
	#[doc = r#"Indicates an instance of measured phase in a phase shift keyed Signal.  A series of measured phases constitute a PMOP sequence.  This field is only expected when the sibling PMOP element indicates PMOP was detected. List size for this element is based on "Order List of Values"."#]
	PmopSequencePhase(Vec<crate::v2_5::enums::PmopSequenceEnum>),
	#[doc = r#"Indicates the actual, full length of the PMOP sequence."#]
	PmopSequenceLength(u32),
}
choice_convert_impls! {
	PmopSequenceType - PmopSequenceTypeSerde
	PmopSequencePhase,
	PmopSequenceLength,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoAirTargetVolumeCommandTypeSerde")]
#[serde(try_from = "PoAirTargetVolumeCommandTypeSerde")]
pub enum PoAirTargetVolumeCommandType {
	#[doc = r#"Indicates the commanded air volume extents."#]
	AirVolume(PoAirTargetVolumeType),
	#[doc = r#"Indicates the unique ID of an Entity that is the target of a cued search. This element can be used as the full specification of the target volume for the cued search. It can also be used in combination with the sibling AirVolume element to define extents of the cue volume around the Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	PoAirTargetVolumeCommandType - PoAirTargetVolumeCommandTypeSerde
	AirVolume,
	EntityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoAirTargetVolumeTypeSerde")]
#[serde(try_from = "PoAirTargetVolumeTypeSerde")]
pub enum PoAirTargetVolumeType {
	#[doc = r#"Indicates a 3D sensor referenced volume that an air volume capability is directed against. This will define the volume in terms of azimuth and elevation angles."#]
	AirVolumeSensorReferenced(crate::v2_5::types::PoAirVolumeSensorReferencedType),
	#[doc = r#"Indicates a 3D area that an air volume capability is directed against. This should define the latitude/longitude extents of the volume along with any guidance regarding the height of the volume."#]
	AirVolumeLocation(crate::v2_5::types::ZoneType),
}
choice_convert_impls! {
	PoAirTargetVolumeType - PoAirTargetVolumeTypeSerde
	AirVolumeSensorReferenced,
	AirVolumeLocation,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoAngleConstraintControlsTypeSerde")]
#[serde(try_from = "PoAngleConstraintControlsTypeSerde")]
pub enum PoAngleConstraintControlsType {
	#[doc = r#"Specifies commanded controls for the collection constraint."#]
	Controls(crate::v2_5::types::PoConstraintControlsType),
	#[doc = r#"The constraints on the angle."#]
	Setting(crate::v2_5::types::AnglePairType),
}
choice_convert_impls! {
	PoAngleConstraintControlsType - PoAngleConstraintControlsTypeSerde
	Controls,
	Setting,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoCollectionPatternConstraintControlsTypeSerde")]
#[serde(try_from = "PoCollectionPatternConstraintControlsTypeSerde")]
pub enum PoCollectionPatternConstraintControlsType {
	#[doc = r#"Specifies commanded controls for the collection constraint."#]
	Controls(crate::v2_5::types::PoConstraintControlsType),
	#[doc = r#"The pattern to use when performing this collection."#]
	Setting(crate::v2_5::enums::CollectionPatternEnum),
}
choice_convert_impls! {
	PoCollectionPatternConstraintControlsType - PoCollectionPatternConstraintControlsTypeSerde
	Controls,
	Setting,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoCommandTypeSerde")]
#[serde(try_from = "PoCommandTypeSerde")]
pub enum PoCommandType {
	#[doc = r#"Indicates a new invocation of a PO Capability. Generally, if accepted, the command will result in one or more new PO Activities being created and reported via the PO_Activity message. The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command. Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::PoCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing PO Activity (which was previously reported via the PO_Activity message and was marked as "interactive"). The request/response interaction terminates as soon as the modification is accepted or rejected. The modifications are reflected in subsequent PO_Activity messages."#]
	Activity(crate::v2_5::types::PoActivityCommandType),
}
choice_convert_impls! {
	PoCommandType - PoCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentFStopSettingsTypeSerde")]
#[serde(try_from = "PoComponentFStopSettingsTypeSerde")]
pub enum PoComponentFStopSettingsType {
	#[doc = r#"Indicates an optical aperture setting."#]
	FStopSetting(f32),
	#[doc = r#"Indicates the automatic subsystem control of the aperture setting."#]
	AutoFStop(bool),
}
choice_convert_impls! {
	PoComponentFStopSettingsType - PoComponentFStopSettingsTypeSerde
	FStopSetting,
	AutoFStop,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentFStopTypeSerde")]
#[serde(try_from = "PoComponentFStopTypeSerde")]
pub enum PoComponentFStopType {
	#[doc = r#"Indicates that the aperture size is fixed."#]
	FixedAperture(f32),
	#[doc = r#"Indicates that the aperture size can be adjusted."#]
	VariableAperture(crate::v2_5::types::PoComponentFStopVariableType),
}
choice_convert_impls! {
	PoComponentFStopType - PoComponentFStopTypeSerde
	FixedAperture,
	VariableAperture,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentLensAssemblyFieldOfViewTypeSerde")]
#[serde(try_from = "PoComponentLensAssemblyFieldOfViewTypeSerde")]
pub enum PoComponentLensAssemblyFieldOfViewType {
	#[doc = r#"This field specifies the FOV for those lens that have a fixed FOV (do not support zooming)."#]
	FixedFov(crate::v2_5::common::AngleQuarterType),
	#[doc = r#"This field specifies the FOV for those lens whose FOV can be changed, i.e. support zooming. This field is for optical zoom only and not electronic zoom."#]
	Zoom(crate::v2_5::types::PoComponentLensAssemblyZoomType),
}
choice_convert_impls! {
	PoComponentLensAssemblyFieldOfViewType - PoComponentLensAssemblyFieldOfViewTypeSerde
	FixedFov,
	Zoom,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsBandpassFrequencyTypeSerde")]
#[serde(try_from = "PoComponentSettingsBandpassFrequencyTypeSerde")]
pub enum PoComponentSettingsBandpassFrequencyType {
	#[doc = r#"The new settings of the filter's band width."#]
	FilterBandSetting(crate::v2_5::types::FrequencyRangeType),
	#[doc = r#"Setting this field to true will enable the automatic enabling/disabling of the filter bandpass frequency. Setting this field to false will disable."#]
	AutoFilterBand(bool),
}
choice_convert_impls! {
	PoComponentSettingsBandpassFrequencyType - PoComponentSettingsBandpassFrequencyTypeSerde
	FilterBandSetting,
	AutoFilterBand,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayCollectionTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayCollectionTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayCollectionType {
	#[doc = r#"The new setting for this channel's collection time."#]
	CollectionTimeSetting(chrono::TimeDelta),
	#[doc = r#"Enabling auto collection time defers control of the channel's collection time settings to the Subsystem."#]
	CollectionTimeControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayCollectionType - PoComponentSettingsFocalPlaneArrayCollectionTypeSerde
	CollectionTimeSetting,
	CollectionTimeControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataType {
	#[doc = r#"The new setting for this channel's NUC."#]
	NucTableNumber(u32),
	#[doc = r#"The new reference and offset setting for this channel's NUC."#]
	NucReferenceOffset(crate::v2_5::types::FocalPlaneArrayNonUniformityCorrectionReferenceType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataType - PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataTypeSerde
	NucTableNumber,
	NucReferenceOffset,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionType {
	#[doc = r#"The new setting for this channel's NUC."#]
	NucSetting(PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionDataType),
	#[doc = r#"Enabling auto NUC defers control of the channel's NUC settings to the Subsystem."#]
	NucControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionType - PoComponentSettingsFocalPlaneArrayNonUniformityCorrectionTypeSerde
	NucSetting,
	NucControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayOpticalBandTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayOpticalBandTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayOpticalBandType {
	#[doc = r#"The new settings for this channel's frequency band. If a single frequency is desired, then set min and max to the same value."#]
	BandSetting(crate::v2_5::types::FrequencyRangeType),
	#[doc = r#"Enabling auto band control defers control of the channel's optical band to the Subsystem."#]
	BandControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayOpticalBandType - PoComponentSettingsFocalPlaneArrayOpticalBandTypeSerde
	BandSetting,
	BandControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayPixelAggregationTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayPixelAggregationTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayPixelAggregationType {
	#[doc = r#"The new settings for this channel's aggregation. For most channels, 0 and 1 indicates a 1-by-1 aggregation, 2 indicates a 2-by-2 aggregation, etc."#]
	AggregationSetting(u32),
	#[doc = r#"Enabling auto pixel aggregation defers control of the channel's pixel aggregation to the Subsystem."#]
	AggregationControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayPixelAggregationType - PoComponentSettingsFocalPlaneArrayPixelAggregationTypeSerde
	AggregationSetting,
	AggregationControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayPixelPolarityTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayPixelPolarityTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayPixelPolarityType {
	#[doc = r#"The new setting for this channel's polarity.  Indicates whether 0 represents hot (false) or cold (true)."#]
	PolaritySetting(bool),
	#[doc = r#"Enabling auto polarity defers control of the channel's polarity settings to the Subsystem."#]
	PolarityControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayPixelPolarityType - PoComponentSettingsFocalPlaneArrayPixelPolarityTypeSerde
	PolaritySetting,
	PolarityControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayPixelScalingTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayPixelScalingTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayPixelScalingType {
	#[doc = r#"The new settings for this channel's pixel scaling."#]
	ScalingSettings(crate::v2_5::types::PoComponentSettingsFocalPlaneArrayPixelScalingSettingsType),
	#[doc = r#"Enabling auto scaling defers control of the channel's gain and offset setting to the Subsystem."#]
	ScalingControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayPixelScalingType - PoComponentSettingsFocalPlaneArrayPixelScalingTypeSerde
	ScalingSettings,
	ScalingControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayScanDirectionTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayScanDirectionTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayScanDirectionType {
	#[doc = r#"The setting for this channel's scan direction, specifying the right or left side of the aircraft."#]
	DirectionSetting(crate::v2_5::enums::RelativeDirectionEnum),
	#[doc = r#"Enabling auto scan direction defers control of the channel's scan direction to the Subsystem."#]
	DirectionControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayScanDirectionType - PoComponentSettingsFocalPlaneArrayScanDirectionTypeSerde
	DirectionSetting,
	DirectionControls,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationTypeSerde")]
pub enum PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationType {
	#[doc = r#"The new setting for this channel's TDI. For most types of FPAs, this will be used to indicate the number of stages in the time dimension."#]
	TdiSetting(f64),
	#[doc = r#"Enabling autoTDI defers control of the channel's TDI to the Subsystem."#]
	TdiControls(crate::v2_5::types::ComponentControlsBType),
}
choice_convert_impls! {
	PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationType - PoComponentSettingsFocalPlaneArrayTimeDelayIntegrationTypeSerde
	TdiSetting,
	TdiControls,
}

#[doc = r#"The position to start the focus lens at when starting the focus sweep."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocusSweepSettingsStartingPositionTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocusSweepSettingsStartingPositionTypeSerde")]
pub enum PoComponentSettingsFocusSweepSettingsStartingPositionType {
	#[doc = r#"This field specifies the starting position as a percentage of the total length that the focus lens can be moved, offset from the zero position of the focus lens."#]
	Percentage(crate::v2_5::common::PercentType),
	#[doc = r#"This field specifies the starting position as number of steps, e.g. when used with a stepper motor, or a number of positions that the lens can be positioned at, offset from the zero position of the focus lens."#]
	NumberOfSteps(u32),
}
choice_convert_impls! {
	PoComponentSettingsFocusSweepSettingsStartingPositionType - PoComponentSettingsFocusSweepSettingsStartingPositionTypeSerde
	Percentage,
	NumberOfSteps,
}

#[doc = r#"The increment used to move the focus lens between steps."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocusSweepSettingsStepIncrementTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocusSweepSettingsStepIncrementTypeSerde")]
pub enum PoComponentSettingsFocusSweepSettingsStepIncrementType {
	#[doc = r#"This field specifies the increment as a percentage of the total length that the focus lens can be moved."#]
	Percentage(crate::v2_5::common::PercentType),
	#[doc = r#"This field specifies the increment as number of steps, e.g. when used with a stepper motor, or a number of positions that the lens can be positioned at."#]
	NumberOfSteps(u32),
}
choice_convert_impls! {
	PoComponentSettingsFocusSweepSettingsStepIncrementType - PoComponentSettingsFocusSweepSettingsStepIncrementTypeSerde
	Percentage,
	NumberOfSteps,
}

#[doc = r#"The time required of each step in the focus sweep.  Generally only specified for line array sensors as this step time is fixed for grid array sensors."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsFocusSweepSettingsStepTimeTypeSerde")]
#[serde(try_from = "PoComponentSettingsFocusSweepSettingsStepTimeTypeSerde")]
pub enum PoComponentSettingsFocusSweepSettingsStepTimeType {
	#[doc = r#"The size specified as the number of lines collected per step."#]
	NumberOfLines(u32),
	#[doc = r#"The time for each step in the focus sweep."#]
	CollectionTime(i64),
}
choice_convert_impls! {
	PoComponentSettingsFocusSweepSettingsStepTimeType - PoComponentSettingsFocusSweepSettingsStepTimeTypeSerde
	NumberOfLines,
	CollectionTime,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsLensAssemblyFieldOfViewTypeSerde")]
#[serde(try_from = "PoComponentSettingsLensAssemblyFieldOfViewTypeSerde")]
pub enum PoComponentSettingsLensAssemblyFieldOfViewType {
	#[doc = r#"The new setting for this lens' FOV."#]
	FovSetting(crate::v2_5::common::AngleQuarterType),
	#[doc = r#"Setting this field to true will enable auto FOV control (auto zoom). Setting this field to false will disable."#]
	AutoZoom(bool),
	#[doc = r#"This field supports incremental changes to the FOV where INCREASE increases the FOV, DECREASE decreases the FOV, and STOP cancels any current incremental change. The implementation can either step change by the change weight or some other value or use the STOP mechanic. The amount of change is affected by the ChangeWeight sub-element."#]
	IncrementalChange(crate::v2_5::types::IncrementalChangeType),
}
choice_convert_impls! {
	PoComponentSettingsLensAssemblyFieldOfViewType - PoComponentSettingsLensAssemblyFieldOfViewTypeSerde
	FovSetting,
	AutoZoom,
	IncrementalChange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsLensAssemblyFocusTypeSerde")]
#[serde(try_from = "PoComponentSettingsLensAssemblyFocusTypeSerde")]
pub enum PoComponentSettingsLensAssemblyFocusType {
	#[doc = r#"The new setting for this lens's focus. This value is interpreted as the current position of the lens as expressed as a percentage of the total distance the lens can travel."#]
	FocusSetting(crate::v2_5::common::PercentType),
	#[doc = r#"Setting this field to true will enable auto focus.  Setting this field to false will disable it."#]
	AutoFocus(bool),
	#[doc = r#"Some focus assemblies support adjusting the lens's focus setting while zooming.  Setting this field to true will enable this capability while setting it to false will disable it."#]
	AutoFocusZoom(bool),
	#[doc = r#"This field supports incremental changes to the focus where INCREASE increases the focus, DECREASE decreases the focus, and STOP cancels any current incremental change. The implementation can either step change by the change weight or some other value or use the STOP mechanic. The amount of change is affected by the ChangeWeight sub-element."#]
	IncrementalChange(crate::v2_5::types::IncrementalChangeType),
}
choice_convert_impls! {
	PoComponentSettingsLensAssemblyFocusType - PoComponentSettingsLensAssemblyFocusTypeSerde
	FocusSetting,
	AutoFocus,
	AutoFocusZoom,
	IncrementalChange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsOutputProductSettingsTypeSerde")]
#[serde(try_from = "PoComponentSettingsOutputProductSettingsTypeSerde")]
pub enum PoComponentSettingsOutputProductSettingsType {
	#[doc = r#"Specifies the settings that apply to this product."#]
	ProductSettings(crate::v2_5::types::PoComponentSettingsProductSettingsType),
	#[doc = r#"Setting this field to true will enable the automatic setting of this product generator's format specific configuration. Setting this field to false will disable."#]
	AutoOutputProductSettings(bool),
}
choice_convert_impls! {
	PoComponentSettingsOutputProductSettingsType - PoComponentSettingsOutputProductSettingsTypeSerde
	ProductSettings,
	AutoOutputProductSettings,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsProcessingStageSettingsTypeSerde")]
#[serde(try_from = "PoComponentSettingsProcessingStageSettingsTypeSerde")]
pub enum PoComponentSettingsProcessingStageSettingsType {
	#[doc = r#"This field contains the new settings of this processing stage."#]
	ProcessingSettings(crate::v2_5::types::PoComponentSettingsProcessingStageProcessingSettingsType),
	#[doc = r#"Setting this field to true will enable the automatic setting of this processing stage's sibling ProcessingSettings. Setting this field to false will disable."#]
	AutoProcessingSettings(bool),
}
choice_convert_impls! {
	PoComponentSettingsProcessingStageSettingsType - PoComponentSettingsProcessingStageSettingsTypeSerde
	ProcessingSettings,
	AutoProcessingSettings,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentSettingsProductGeneratorSettingsTypeSerde")]
#[serde(try_from = "PoComponentSettingsProductGeneratorSettingsTypeSerde")]
pub enum PoComponentSettingsProductGeneratorSettingsType {
	#[doc = r#"Indicates the minimum and maximum possible product generator settings for this subsystem."#]
	GeneratorSettings(crate::v2_5::types::PoComponentSettingsGeneratorSettingsType),
	#[doc = r#"Setting this field to true will enable the automatic setting of this product generator's sibling GeneratorSettings. Setting this field to false will disable."#]
	AutoGeneratorSettings(bool),
}
choice_convert_impls! {
	PoComponentSettingsProductGeneratorSettingsType - PoComponentSettingsProductGeneratorSettingsTypeSerde
	GeneratorSettings,
	AutoGeneratorSettings,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataTypeSerde")]
#[serde(try_from = "PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataTypeSerde")]
pub enum PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataType {
	#[doc = r#"This field indicates the table number for NUC settings."#]
	NucTableNumber(u32),
	#[doc = r#"This field indicates the reference and offset for the channel's NUC."#]
	NucReferenceOffset(crate::v2_5::types::FocalPlaneArrayNonUniformityCorrectionReferenceType),
}
choice_convert_impls! {
	PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataType - PoComponentStatusFocalPlaneArrayNonUniformityCorrectionDataTypeSerde
	NucTableNumber,
	NucReferenceOffset,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoComponentStatusLensAssemblyFocusTypeSerde")]
#[serde(try_from = "PoComponentStatusLensAssemblyFocusTypeSerde")]
pub enum PoComponentStatusLensAssemblyFocusType {
	#[doc = r#"This field contains the current focus setting of the lens. This value is interpreted as the current position of the lens as expressed as a percentage of the total distance the lens can travel."#]
	FocusSetting(crate::v2_5::common::PercentType),
	#[doc = r#"Some lens assemblies support an auto-focus capability in which the lens's focus setting is automatically set. This field is the current setting of this auto-focus capability.  This field indicates whether the lens's auto-focus capability is enabled (true) or disabled (false)."#]
	AutoFocus(bool),
	#[doc = r#"Some focus assemblies support the capability of adjusting the lens's focus setting while the zooming into and away from the target. For such assemblies, this field indicates whether this capability is enabled (true) or not (false)."#]
	AutoFocusZoom(bool),
	#[doc = r#"The current incremental change enumeration."#]
	ChangeType(crate::v2_5::enums::IncrementalChangeEnum),
}
choice_convert_impls! {
	PoComponentStatusLensAssemblyFocusType - PoComponentStatusLensAssemblyFocusTypeSerde
	FocusSetting,
	AutoFocus,
	AutoFocusZoom,
	ChangeType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoSlantRangeConstraintControlsTypeSerde")]
#[serde(try_from = "PoSlantRangeConstraintControlsTypeSerde")]
pub enum PoSlantRangeConstraintControlsType {
	#[doc = r#"Specifies commanded controls for the collection constraint."#]
	Controls(crate::v2_5::types::PoConstraintControlsType),
	#[doc = r#"Distance from the sensor reference point (e.g. aperture reference point) to the ground reference point."#]
	Setting(crate::v2_5::types::DistanceConstraintsType),
}
choice_convert_impls! {
	PoSlantRangeConstraintControlsType - PoSlantRangeConstraintControlsTypeSerde
	Controls,
	Setting,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoSweepSpeedConstraintControlsTypeSerde")]
#[serde(try_from = "PoSweepSpeedConstraintControlsTypeSerde")]
pub enum PoSweepSpeedConstraintControlsType {
	#[doc = r#"Specifies commanded controls for the collection constraint."#]
	Controls(crate::v2_5::types::PoConstraintControlsType),
	#[doc = r#"The rate at which the sensor look-point moves across the terrain.  This is only applicable to area or line targets."#]
	Setting(crate::v2_5::common::SpeedType),
}
choice_convert_impls! {
	PoSweepSpeedConstraintControlsType - PoSweepSpeedConstraintControlsTypeSerde
	Controls,
	Setting,
}

#[doc = r#"Indicates the position or location to point the XX Subsystem to do a XX collection, track, or search as part of an XX Activity. If the XX Subsystem cannot point itself, then the TurretSlaved Type is used. If the system wants to allow the subsystem to control its own LOS then the ActivitySlavedID is used. FixedPointing is used to point to a predetermined location defined by its Enum values."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PoTargetTypeSerde")]
#[serde(try_from = "PoTargetTypeSerde")]
pub enum PoTargetType {
	#[doc = r#"Indicates the source of or explicit values for geospatial characteristics of the target of the Command. When multiple points/targets are given, a best-fit, multi-track or similar behavior is expected."#]
	Geospatial(Vec<TargetType>),
	#[doc = r#"Indicates body or inertially referenced pointing angles as the "target" of the Command."#]
	Pointed(crate::v2_5::types::TurretCommandPositionType),
	#[doc = r#"Indicates body or inertially referenced line of sight pointing angles and/or rates as the "target" of the Command."#]
	LosOption(LosDType),
	#[doc = r#"Indicates the volume or specific Entity that defines the extents of the spatial region covered by the command. For non-cued Capabilities, one air volume instance of this element is expected. For cued Capabilities, two instances of this element are allowed: one with an Entity specified and a second with an air volume that define the extents of the cue volume around the Entity. For cued Capabilities, a single instance is allowed if it is an Entity in which case the Entity location uncertainty defines the extents of the cue volume."#]
	Volume(Vec<PoAirTargetVolumeCommandType>),
	#[doc = r#"Indicates a PO_Command that has an accompanying TurretCommand that determines the "target" of the Command. The PO Capability is then pointed/targeted by the TurretCommand with other command details coming from the PO_Command."#]
	TurretSlaved(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates a Pointing Command that has a secondary/coupled Activity, possibly of another Capability type such as AO or PO, that determines pointing."#]
	ActivitySlavedId(crate::v2_5::types::ActivityIdType),
	#[doc = r#"Indicates that a subsystem with point to a predetermined location defined by its Enum values."#]
	FixedPointing(crate::v2_5::enums::FixedPointingEnum),
}
choice_convert_impls! {
	PoTargetType - PoTargetTypeSerde
	Geospatial,
	Pointed,
	LosOption,
	Volume,
	TurretSlaved,
	ActivitySlavedId,
	FixedPointing,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PointChoice3DTypeSerde")]
#[serde(try_from = "PointChoice3DTypeSerde")]
pub enum PointChoice3DType {
	#[doc = r#"Contains a geospatial location."#]
	AbsolutePoint(crate::v2_5::types::Point3DType),
	#[doc = r#"The offset of the location from the origin of the separately defined reference frame."#]
	RelativePoint(crate::v2_5::types::Point3DRelativeType),
}
choice_convert_impls! {
	PointChoice3DType - PointChoice3DTypeSerde
	AbsolutePoint,
	RelativePoint,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PointChoice4DTypeSerde")]
#[serde(try_from = "PointChoice4DTypeSerde")]
pub enum PointChoice4DType {
	#[doc = r#"Contains a geospatial location."#]
	AbsolutePoint(crate::v2_5::types::Point4DType),
	#[doc = r#"The offset of the area from the origin of the reference frame at the time provided."#]
	RelativePoint(crate::v2_5::types::Point4DRelativeType),
}
choice_convert_impls! {
	PointChoice4DType - PointChoice4DTypeSerde
	AbsolutePoint,
	RelativePoint,
}

#[doc = r#"Specifies a location either as a geospatial location or a location relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PointChoiceTypeSerde")]
#[serde(try_from = "PointChoiceTypeSerde")]
pub enum PointChoiceType {
	#[doc = r#"Contains a geospatial location."#]
	AbsolutePoint(crate::v2_5::types::Point2DType),
	#[doc = r#"The offset of the area from the origin of the reference frame specified by the sibling field."#]
	RelativePoint(crate::v2_5::types::Point2DRelativeType),
}
choice_convert_impls! {
	PointChoiceType - PointChoiceTypeSerde
	AbsolutePoint,
	RelativePoint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PointMultiTypeSerde")]
#[serde(try_from = "PointMultiTypeSerde")]
pub enum PointMultiType {
	#[doc = r#"Indicates a point expressed in lat, lon, alt coordinates."#]
	CenterpointWgs(crate::v2_5::types::Point2DType),
	#[doc = r#"Indicates a point expressed in ECI J2K coordinates."#]
	CenterpointJ2k(crate::v2_5::types::J2kPositionType),
}
choice_convert_impls! {
	PointMultiType - PointMultiTypeSerde
	CenterpointWgs,
	CenterpointJ2k,
}

#[doc = r#"Indicates the position or location to point the XX Subsystem to do a XX collection, track, or search as part of an XX Activity. If the XX Subsystem cannot point itself, then the TurretSlaved Type is used. If the system wants to allow the subsystem to control its own LOS then the ActivitySlavedID is used. FixedPointing is used to point to a predetermined location defined by its Enum values."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PointingTypeSerde")]
#[serde(try_from = "PointingTypeSerde")]
pub enum PointingType {
	#[doc = r#"Indicates the source of or explicit values for geospatial characteristics of the target of the Command. When multiple points/targets are given, a best-fit, multi-track or similar behavior is expected."#]
	Geospatial(Vec<TargetType>),
	#[doc = r#"Indicates body or inertially referenced line of sight pointing angles and/or rates as the "target" of the Command."#]
	LosOption(LosDType),
	#[doc = r#"Indicates the volume or specific Entity that defines the extents of the spatial region covered by the command. For non-cued Capabilities, one air volume instance of this element is expected. For cued Capabilities, two instances of this element are allowed: one with an Entity specified and a second with an air volume that define the extents of the cue volume around the Entity. For cued Capabilities, a single instance is allowed if it is an Entity in which case the Entity location uncertainty defines the extents of the cue volume."#]
	Volume(Vec<PoAirTargetVolumeCommandType>),
	#[doc = r#"Indicates a Pointing Command that has an accompanying TurretCommand that determines the "target" of the Command. The XX Capability is then pointed/targeted by the TurretCommand with other command details coming from the XX_Command."#]
	TurretSlaved(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates a Pointing Command that has a secondary/coupled Activity, possibly of another Capability type such as AO or PO, that determines pointing."#]
	ActivitySlavedId(crate::v2_5::types::ActivityIdType),
	#[doc = r#"Indicates that a subsystem with point to a predetermined location defined by its Enum values."#]
	FixedPointing(crate::v2_5::enums::FixedPointingEnum),
}
choice_convert_impls! {
	PointingType - PointingTypeSerde
	Geospatial,
	LosOption,
	Volume,
	TurretSlaved,
	ActivitySlavedId,
	FixedPointing,
}

#[doc = r#"Specifies a polygon by geospatial locations or as locations relative to a separately defined reference frame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PolygonPointChoiceTypeSerde")]
#[serde(try_from = "PolygonPointChoiceTypeSerde")]
pub enum PolygonPointChoiceType {
	#[doc = r#"Geospatial points defining the vertices of a polygon."#]
	Point2D(Vec<crate::v2_5::types::Point2DType>),
	#[doc = r#"Indicates a polygon that is relative to a separately defined reference frame defined in ReferenceFrames message."#]
	RelativePolygon(crate::v2_5::types::PolygonRelativeType),
}
choice_convert_impls! {
	PolygonPointChoiceType - PolygonPointChoiceTypeSerde
	Point2D,
	RelativePolygon,
}

#[doc = r#"Indicates the source of position data."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PositionSourceIdChoiceTypeSerde")]
#[serde(try_from = "PositionSourceIdChoiceTypeSerde")]
pub enum PositionSourceIdChoiceType {
	#[doc = r#"Indicates the unique ID of the System that produced this report."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the unique ID of the Subsystem that produced this report."#]
	SubsystemId(crate::v2_5::types::SubsystemIdType),
	#[doc = r#"Indicates the unique ID of the Service that produced this report."#]
	ServiceId(crate::v2_5::types::ServiceIdType),
}
choice_convert_impls! {
	PositionSourceIdChoiceType - PositionSourceIdChoiceTypeSerde
	SystemId,
	SubsystemId,
	ServiceId,
}

#[doc = r#"This element defines a filter which can be applied to any product regardless of type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductFilterTypeSerde")]
#[serde(try_from = "ProductFilterTypeSerde")]
pub enum ProductFilterType {
	#[doc = r#"This field specifies a filter criteria for the source of a product."#]
	Source(crate::v2_5::types::SourceFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of a product or file.  If a zone is supplied, any intersection with the supplied zone is considered passing the filter criteria.  If a point is supplied, it indicates that the product or file must contain the point for the filter to apply.  Point based products or files with a point location filter have to match exactly to a defined point in the product or file."#]
	Geospatial(LocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of the source of a product."#]
	SourceGeospatial(crate::v2_5::types::SourceLocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the file format of a product."#]
	FileFormat(FileFormatType),
	#[doc = r#"This field specifies a filter criteria for the classification of a product."#]
	SecurityInformation(crate::v2_5::types::SecurityInformationType),
	#[doc = r#"This field specifies a filter criteria for a range of creation times of a product."#]
	DateTimeRange(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"This field specifies the criteria used to filter messages based on a specified QueryType expression."#]
	QueryFilter(QueryType),
	#[doc = r#"The product type that the subplan applies to."#]
	ProductType(crate::v2_5::enums::ProductTypeEnum),
}
choice_convert_impls! {
	ProductFilterType - ProductFilterTypeSerde
	Source,
	Geospatial,
	SourceGeospatial,
	FileFormat,
	SecurityInformation,
	DateTimeRange,
	QueryFilter,
	ProductType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductGeospatialLocationTypeSerde")]
#[serde(try_from = "ProductGeospatialLocationTypeSerde")]
pub enum ProductGeospatialLocationType {
	#[doc = r#"Indicates a point associated with the Product.  Generally, services are encouraged to send altitude and/or time data whenever it is known."#]
	Point2D(crate::v2_5::types::Point2DType),
	#[doc = r#"This element represents the bounding area for the product associated with this ProductMetadata."#]
	Zone(crate::v2_5::types::ZoneType),
	#[doc = r#"This element represents a line of sight reference for the product associated with this ProductMetadata."#]
	RelativePosition(crate::v2_5::types::RelativePositionType),
}
choice_convert_impls! {
	ProductGeospatialLocationType - ProductGeospatialLocationTypeSerde
	Point2D,
	Zone,
	RelativePosition,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductLocationTypeSerde")]
#[serde(try_from = "ProductLocationTypeSerde")]
pub enum ProductLocationType {
	#[doc = r#"Indicates the network location at which the file or product, whose data format is not defined in the UCI schema, can be found."#]
	Network(crate::v2_5::types::EndpointReferenceType),
	#[doc = r#"Indicates that the File or Product is located in/on a System and is not reachable via a network URI; however, it can possibly be made reachable via a network URI if requested by a download request."#]
	System(crate::v2_5::types::ProductSystemLocationType),
}
choice_convert_impls! {
	ProductLocationType - ProductLocationTypeSerde
	Network,
	System,
}

#[doc = r#"Indicates when the product is needed by."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductNeededByTypeSerde")]
#[serde(try_from = "ProductNeededByTypeSerde")]
pub enum ProductNeededByType {
	#[doc = r#"Indicates that the product or products is needed as soon as possible."#]
	AsSoonAsPossible(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates that the product is needed by a particular Date/Time."#]
	AbsoluteTime(chrono::DateTime<chrono::Utc>),
	#[doc = r#"Indicates that the product is needed no later by a particular time duration after an observed event (e.g. Maneuver or Detection)."#]
	RelativeToEventTime(chrono::TimeDelta),
}
choice_convert_impls! {
	ProductNeededByType - ProductNeededByTypeSerde
	AsSoonAsPossible,
	AbsoluteTime,
	RelativeToEventTime,
}

#[doc = r#"This element defines a filter which can be applied to any product or file regardless of type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductOrFileFilterTypeSerde")]
#[serde(try_from = "ProductOrFileFilterTypeSerde")]
pub enum ProductOrFileFilterType {
	#[doc = r#"This field specifies a filter criteria for the source of a product or file."#]
	Source(crate::v2_5::types::SourceFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of a product or file.  If a zone is supplied, any intersection with the supplied zone is considered passing the filter criteria.  If a point is supplied, it indicates that the product or file must contain the point for the filter to apply.  Point based products or files with a point location filter have to match exactly to a defined point in the product or file."#]
	Geospatial(LocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of the source of a product or file."#]
	SourceGeospatial(crate::v2_5::types::SourceLocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the file format of a product or file."#]
	FileFormat(FileFormatType),
	#[doc = r#"This field specifies a filter criteria for the classification of a product or file."#]
	SecurityInformation(crate::v2_5::types::SecurityInformationType),
	#[doc = r#"This field specifies a filter criteria for a range of creation times of a product or file."#]
	DateTimeRange(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"This field specifies the criteria used to filter messages based on a specified QueryType expression."#]
	QueryFilter(QueryType),
	#[doc = r#"The product type that the subplan applies to."#]
	ProductType(crate::v2_5::enums::ProductTypeEnum),
	#[doc = r#"The file type that the subplan applies to."#]
	FileType(crate::v2_5::enums::FileTypeEnum),
}
choice_convert_impls! {
	ProductOrFileFilterType - ProductOrFileFilterTypeSerde
	Source,
	Geospatial,
	SourceGeospatial,
	FileFormat,
	SecurityInformation,
	DateTimeRange,
	QueryFilter,
	ProductType,
	FileType,
}

#[doc = r#"Indicates the product or file type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductOrFileTypeSerde")]
#[serde(try_from = "ProductOrFileTypeSerde")]
pub enum ProductOrFileType {
	#[doc = r#"Indicates the product type."#]
	ProductType(crate::v2_5::enums::ProductTypeEnum),
	#[doc = r#"Indicates the content of the file."#]
	FileType(crate::v2_5::enums::FileTypeEnum),
}
choice_convert_impls! {
	ProductOrFileType - ProductOrFileTypeSerde
	ProductType,
	FileType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductParentTypeSerde")]
#[serde(try_from = "ProductParentTypeSerde")]
pub enum ProductParentType {
	#[doc = r#"The ID of the Requirement that created (or is yet to create) the product."#]
	RequirementId(RequirementInstanceIdChoiceType),
	#[doc = r#"The ID of the ProductProcessingRequest that created (or is yet to create) the product."#]
	ProductProcessingRequestId(Vec<crate::v2_5::types::RequestIdType>),
}
choice_convert_impls! {
	ProductParentType - ProductParentTypeSerde
	RequirementId,
	ProductProcessingRequestId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductReferenceTypeSerde")]
#[serde(try_from = "ProductReferenceTypeSerde")]
pub enum ProductReferenceType {
	#[doc = r#"This element defines the file name of the product.  This can be used if the name of a product has been advertised by the system via an out-of-band source.  This only works if the service interface receiving this message is the service interface advertising the filename via the out-of-band source."#]
	FileName(crate::v2_5::common::FileNameType),
	#[doc = r#"The product metadata ID for the product. This element is the most common method for referencing a product.  A service has to also obtain the ProductLocation message to discover how to retrieve the product.  A product location can be a network location or a reference to a system.  If the location is a reference to a system, a download request must be issued to request that the system download the product and make it available via a network URI."#]
	ProductMetadataId(crate::v2_5::types::ProductMetadataIdType),
	#[doc = r#"This element references a product by the parent task or command that created it (or is yet to create it).  In some cases, a task or command can result in generation of more than one product type.  As a result, it is sometimes necessary to include the product type when referencing a product in this way."#]
	ProductReferenceByParent(crate::v2_5::types::ProductReferenceByParentType),
}
choice_convert_impls! {
	ProductReferenceType - ProductReferenceTypeSerde
	FileName,
	ProductMetadataId,
	ProductReferenceByParent,
}

#[doc = r#"This element defines a filter which can be applied to a specific product type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProductTypeFilterTypeSerde")]
#[serde(try_from = "ProductTypeFilterTypeSerde")]
pub enum ProductTypeFilterType {
	#[doc = r#"This field specifies a filter criteria for the source of a product."#]
	Source(crate::v2_5::types::SourceFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of a product."#]
	Geospatial(LocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the location of the source of a product."#]
	SourceGeospatial(crate::v2_5::types::SourceLocationFilterType),
	#[doc = r#"This field specifies a filter criteria for the file format of a product."#]
	FileFormat(FileFormatType),
	#[doc = r#"This field specifies a filter criteria for the classification of a product."#]
	SecurityInformation(crate::v2_5::types::SecurityInformationType),
	#[doc = r#"This field specifies a filter criteria for a range of creation times of a product."#]
	DateTimeRange(crate::v2_5::types::DateTimeRangeType),
	#[doc = r#"This field specifies the criteria used to filter messages based on a specified QueryType expression."#]
	QueryFilter(QueryType),
}
choice_convert_impls! {
	ProductTypeFilterType - ProductTypeFilterTypeSerde
	Source,
	Geospatial,
	SourceGeospatial,
	FileFormat,
	SecurityInformation,
	DateTimeRange,
	QueryFilter,
}

#[doc = r#"Indicates the choice of propagator types: A general Propagator or a VCM Propagator."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "PropagatorChoiceTypeSerde")]
#[serde(try_from = "PropagatorChoiceTypeSerde")]
pub enum PropagatorChoiceType {
	#[doc = r#"A fully-defined set of general propagator settings."#]
	GeneralPropagator(crate::v2_5::types::PropagatorType),
	#[doc = r#"A fully-defined set of USSF Astrodynamic Standards propagator settings."#]
	VcmPropagator(crate::v2_5::types::VcmPropagatorType),
}
choice_convert_impls! {
	PropagatorChoiceType - PropagatorChoiceTypeSerde
	GeneralPropagator,
	VcmPropagator,
}

#[doc = r#"This complex type provides the different types of proximity operations."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ProximityOrbitChoiceTypeSerde")]
#[serde(try_from = "ProximityOrbitChoiceTypeSerde")]
pub enum ProximityOrbitChoiceType {
	#[doc = r#"Indicates the proximity operation is a natural motion type. Natural motion circumnavigation where the orbital parameters are such that minimal stationkeeping is required to maintain proximity operations."#]
	NaturalMotion(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the proximity operation is a forced motion type. Forced motion requires consistent maneuvers to maintain proximity operations."#]
	ForcedMotion(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the proximity operation is an R-Bar Perch type. This means the proximity operation is along the radial vector of the target."#]
	RBarPerch(crate::v2_5::enums::RBarApproachEnum),
	#[doc = r#"Indicates the proximity operation is a V-Bar Perch type. This means the proximity operation is along the velocity vector of the target."#]
	VBarPerch(crate::v2_5::enums::VBarApproachEnum),
	#[doc = r#"Indicates the proximity operation's orbital solution must be within the defined relative plane angles min and max."#]
	DeltaOrbitalPlaneTolerance(crate::v2_5::types::AngleHalfPairType),
}
choice_convert_impls! {
	ProximityOrbitChoiceType - ProximityOrbitChoiceTypeSerde
	NaturalMotion,
	ForcedMotion,
	RBarPerch,
	VBarPerch,
	DeltaOrbitalPlaneTolerance,
}

#[doc = r#"Compares the length of the sequence formed by the specified Step to the value indicated by this choice."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "QueryCountValueTypeSerde")]
#[serde(try_from = "QueryCountValueTypeSerde")]
pub enum QueryCountValueType {
	#[doc = r#"Tests if the length of the sequence formed by the specified Step is equal to this value."#]
	Equals(u32),
	#[doc = r#"Tests if the length of the sequence formed by the specified Step is less than this value."#]
	LessThan(u32),
	#[doc = r#"Tests if the length of the sequence formed by the specified Step is less than or equal to this value."#]
	LessThanOrEqualTo(u32),
	#[doc = r#"Tests if the length of the sequence formed by the specified Step is greater than this value."#]
	GreaterThan(u32),
	#[doc = r#"Tests if the length of the sequence formed by the specified Step is greater than or equal to this value."#]
	GreaterThanOrEqualTo(u32),
}
choice_convert_impls! {
	QueryCountValueType - QueryCountValueTypeSerde
	Equals,
	LessThan,
	LessThanOrEqualTo,
	GreaterThan,
	GreaterThanOrEqualTo,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "QueryResultTypeSerde")]
#[serde(try_from = "QueryResultTypeSerde")]
pub enum QueryResultType {
	#[doc = r#"The messages located in response to the query request."#]
	Message(Vec<crate::v2_5::types::MessageType>),
	#[doc = r#"The object identifiers located in response to the query request."#]
	Id(Vec<crate::v2_5::types::IdType>),
}
choice_convert_impls! {
	QueryResultType - QueryResultTypeSerde
	Message,
	Id,
}

#[doc = r#"The mechanism by which queries navigate the data model of a UCI Message, which should be considered as a tree structure containing branch and leaf nodes.  Steps may either traverse up the tree (Ancestor and Parent), down the tree (Element, Descendant, and Child), or remain at the current context.  Steps are evaluated sequentially from the current context, with the default context of a query as the root of the tree, i.e. the top-level global element declaration.  The result of each Step forms a sequence of zero or more nodes that is then used as the input to the next Step, where each node of the input sequence is used as the current context with all sequences concatenated together, repeated until all Steps are evaluated.  Each node in this sequence is either a present optional field, a required field, or an item in a list.  For example, a Step that matches a list field with a length of two will result in a sequence of two nodes.  The resulting sequence is then evaluated by the query with a given operation.  Some operations, such as Equals, that operate on a single value are instead performed on each individual node in the sequence and the result is computed by the logical OR of all the results."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "QueryStepTypeSerde")]
#[serde(try_from = "QueryStepTypeSerde")]
pub enum QueryStepType {
	#[doc = r#"Forms a sequence of zero or more nodes by navigating to the direct child of the current context with the specified local name and namespace URI."#]
	Element(crate::v2_5::types::NamedElementType),
	#[doc = r#"Forms a sequence of exactly one node that is the root of the current context.  This root is the Message that is the same context at which this query was initially evaluated."#]
	Root(crate::v2_5::common::EmptyType),
	#[doc = r#"Forms a sequence of zero or more nodes by navigating up the tree to any ancestor of the current context (regardless of depth) that matches the specified wildcard local name and namespace URI."#]
	Ancestor(crate::v2_5::types::WildcardElementType),
	#[doc = r#"Forms a sequence of zero or more nodes by navigating down the tree to any descendant of the current context (regardless of depth) that matches the specified wildcard local name and namespace URI."#]
	Descendant(crate::v2_5::types::WildcardElementType),
	#[doc = r#"Forms a sequence of one node (the parent of the current context) if the parent of the current context matches the specified wildcard local name and namespace URI, otherwise the empty sequence."#]
	Parent(crate::v2_5::types::WildcardElementType),
	#[doc = r#"Forms a sequence of zero or more nodes by navigating down the tree to any direct child of the current context that matches the specified wildcard local name and namespace URI."#]
	Child(crate::v2_5::types::WildcardElementType),
}
choice_convert_impls! {
	QueryStepType - QueryStepTypeSerde
	Element,
	Root,
	Ancestor,
	Descendant,
	Parent,
	Child,
}

#[doc = r#"Defines the generic UCI Query Language (UQL) operations.  Each operation has a single input, a node that is the current context of the query, and will output either true or false.  When determining whether a Message should be sent in a corresponding status, it should be sent if the output of the query is true.  The data model of a UCI Message should be considered as a tree structure containing branch and leaf nodes.  The default context of a query is the Message, i.e. the top-level global element declaration.  Each UQL query is evaluated separately for each Message.  For more information on how the tree is evaluated, see the annotations in QueryStepType."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "QueryTypeSerde")]
#[serde(try_from = "QueryTypeSerde")]
pub enum QueryType {
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is equal to the Value sub-element."#]
	Equals(crate::v2_5::types::QueryEqualsType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is less than the Value sub-element."#]
	LessThan(crate::v2_5::types::QueryComparisonType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is less than or equal to the Value sub-element."#]
	LessThanOrEqualTo(crate::v2_5::types::QueryComparisonType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is greater than the Value sub-element."#]
	GreaterThan(crate::v2_5::types::QueryComparisonType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is greater than or equal to the Value sub-element."#]
	GreaterThanOrEqualTo(crate::v2_5::types::QueryComparisonType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is a string value that has a substring (case-sensitive) equal to the Value sub-element."#]
	ContainsCaseSensitive(crate::v2_5::types::QueryContainsType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is a string value that has a substring (case-insensitive) equal to the Value sub-element."#]
	ContainsCaseInsensitive(crate::v2_5::types::QueryContainsType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is or derives from the given type name."#]
	InstanceOf(crate::v2_5::types::QueryInstanceOfType),
	#[doc = r#"Evaluates the query to true if the length of the sequence formed by the Step sub-element matches the Value sub-element."#]
	Count(crate::v2_5::types::QueryCountType),
	#[doc = r#"Evaluates the query to true if the sequence formed by the Step sub-element contains one or more nodes."#]
	Exists(crate::v2_5::types::QueryExistsType),
	#[doc = r#"Evaluates the query to true if any node in the sequence formed by the Step sub-element is evaluated by the sub-query (i.e. the Test sub-element) to true."#]
	AnyMatch(crate::v2_5::types::QueryMatchType),
	#[doc = r#"Evaluates the query to true if all nodes in the sequence formed by the Step sub-element are evaluated by the sub-query (i.e. the Test sub-element) to true."#]
	AllMatch(crate::v2_5::types::QueryMatchType),
	#[doc = r#"Evaluates the query to true if all of the sub-queries are evaluated to true."#]
	And(Vec<crate::v2_5::types::QueryPet>),
	#[doc = r#"Evaluates the query to true if any of the sub-queries are evaluated to true."#]
	Or(Vec<crate::v2_5::types::QueryPet>),
	#[doc = r#"Evaluates the query to true if the sub-query is evaluated to false."#]
	Not(crate::v2_5::types::QueryPet),
}
choice_convert_impls! {
	QueryType - QueryTypeSerde
	Equals,
	LessThan,
	LessThanOrEqualTo,
	GreaterThan,
	GreaterThanOrEqualTo,
	ContainsCaseSensitive,
	ContainsCaseInsensitive,
	InstanceOf,
	Count,
	Exists,
	AnyMatch,
	AllMatch,
	And,
	Or,
	Not,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RadarAltimeterCommandTypeSerde")]
#[serde(try_from = "RadarAltimeterCommandTypeSerde")]
pub enum RadarAltimeterCommandType {
	#[doc = r#"Indicates a new invocation of an RadarAltimeter Capability.  Generally, if accepted, the command will result in one or more new RadarAltimeter Activities being created and reported via the RadarAltimeter_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::RadarAltimeterCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Activity (which was previously reported via the Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent Activity messages."#]
	Activity(crate::v2_5::types::RadarAltimeterActivityCommandType),
}
choice_convert_impls! {
	RadarAltimeterCommandType - RadarAltimeterCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"This is a switch that allows TargetType to be a sibling of SensorReferencedCoverageArea."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RadarPointingTargetTypeSerde")]
#[serde(try_from = "RadarPointingTargetTypeSerde")]
pub enum RadarPointingTargetType {
	#[doc = r#"Indicates details of the target of the Command."#]
	GeospatialTarget(TargetType),
	#[doc = r#"Indicates a coverage area specified by azimuth extent and elevation extent or azimuth extent and range extent."#]
	SensorReferencedCoverageArea(crate::v2_5::types::SensorReferencedCoverageAreaType),
}
choice_convert_impls! {
	RadarPointingTargetType - RadarPointingTargetTypeSerde
	GeospatialTarget,
	SensorReferencedCoverageArea,
}

#[doc = r#"Beam spoiling or taper to be applied to transmit or receive beam."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RadarSpoilTaperTypeSerde")]
#[serde(try_from = "RadarSpoilTaperTypeSerde")]
pub enum RadarSpoilTaperType {
	#[doc = r#"Specific taper to be applied to this command. Note that capability does not necessarily implement every combination of taper that can be commanded.  If this value is chosen, collection performance may degrade."#]
	Taper(crate::v2_5::types::RadarTaperType),
	#[doc = r#"Beam spoiling to be applied.  This value is the ratio of the spoiled beam width to the unspoiled beam width.  If this value is chosen, collection performance may degrade."#]
	Spoil(crate::v2_5::types::RadarSpoilType),
}
choice_convert_impls! {
	RadarSpoilTaperType - RadarSpoilTaperTypeSerde
	Taper,
	Spoil,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RadarTaperWeightingFunctionTypeSerde")]
#[serde(try_from = "RadarTaperWeightingFunctionTypeSerde")]
pub enum RadarTaperWeightingFunctionType {
	#[doc = r#"Beam taper to be used during collection.  If this value is chosen, collection performance may degrade."#]
	StandardWeightingFunction(crate::v2_5::enums::RadarWeightingFunctionsEnum),
	#[doc = r#"Additional taper weighting functions may be commanded through the use of a foreign key type."#]
	OtherTaper(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	RadarTaperWeightingFunctionType - RadarTaperWeightingFunctionTypeSerde
	StandardWeightingFunction,
	OtherTaper,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RangeElevationExtentChoiceTypeSerde")]
#[serde(try_from = "RangeElevationExtentChoiceTypeSerde")]
pub enum RangeElevationExtentChoiceType {
	#[doc = r#"Elevation extent of the look area."#]
	Elevation(crate::v2_5::types::AnglePairType),
	#[doc = r#"Range extent of the look area."#]
	Range(crate::v2_5::types::RangeExtentType),
}
choice_convert_impls! {
	RangeElevationExtentChoiceType - RangeElevationExtentChoiceTypeSerde
	Elevation,
	Range,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ReadinessTimeSpanTypeSerde")]
#[serde(try_from = "ReadinessTimeSpanTypeSerde")]
pub enum ReadinessTimeSpanType {
	#[doc = r#"Indicates the time period is the same as the time period for the Mission referenced by the associated MissionID.  The Mission's time period is given in the corresponding Mission message.  This choice allows a simple deferral to the time period of the Mission."#]
	ByMission(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the time period is a discrete one, shorter than that of the entire associated Mission."#]
	SubMission(ScheduleType),
}
choice_convert_impls! {
	ReadinessTimeSpanType - ReadinessTimeSpanTypeSerde
	ByMission,
	SubMission,
}

#[doc = r#"Container to provide the appropriate object that is the origin of a reference frame created with message ReferenceFrame."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ReferenceFrameObjectToFollowTypeSerde")]
#[serde(try_from = "ReferenceFrameObjectToFollowTypeSerde")]
pub enum ReferenceFrameObjectToFollowType {
	#[doc = r#"An Entity to use as the object reference."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"A System to use as the object reference."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"A SignalReport to use as the object reference.  When specifying a SignalReportID, the sender should ensure the SignalReport includes a location as this data is optional in that message."#]
	SignalReportId(crate::v2_5::types::SignalReportIdType),
}
choice_convert_impls! {
	ReferenceFrameObjectToFollowType - ReferenceFrameObjectToFollowTypeSerde
	EntityId,
	SystemId,
	SignalReportId,
}

#[doc = r#"Provides the object that is the origin of a reference frame. This allows defining an area around a point (object) that is not stationary, it moves along with the object so its definition is relative to that object."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ReferenceFrameOriginChoiceTypeSerde")]
#[serde(try_from = "ReferenceFrameOriginChoiceTypeSerde")]
pub enum ReferenceFrameOriginChoiceType {
	#[doc = r#"This points to an alternative source that can provide reference frame origin information.  This would be established by individual Programs.  The "key" is a unique identifier for the alternative source."#]
	AlternateSource(crate::v2_5::types::ForeignKeyType),
	#[doc = r#"UUID of the source providing the position of the reference frame origin."#]
	ObjectToFollowIdentifier(ReferenceFrameObjectToFollowType),
	#[doc = r#"Physical location of the reference frame origin in geospatial coordinates."#]
	GeospatialPosition(crate::v2_5::types::ReferenceFrameOriginType),
	#[doc = r#"Physical location of the reference frame origin in geospatial coordinates."#]
	KinematicsReferenceFrameOrigin(crate::v2_5::types::ReferenceFrameOriginKinematicsType),
}
choice_convert_impls! {
	ReferenceFrameOriginChoiceType - ReferenceFrameOriginChoiceTypeSerde
	AlternateSource,
	ObjectToFollowIdentifier,
	GeospatialPosition,
	KinematicsReferenceFrameOrigin,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ReferenceObjectTypeSerde")]
#[serde(try_from = "ReferenceObjectTypeSerde")]
pub enum ReferenceObjectType {
	#[doc = r#"Indicates the reference object is an Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the reference object is an OpPoint."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Indicates the reference object is a System UUID."#]
	SystemId(crate::v2_5::types::SystemIdType),
}
choice_convert_impls! {
	ReferenceObjectType - ReferenceObjectTypeSerde
	EntityId,
	OpPointId,
	SystemId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RefuelCapabilityCommandTypeSerde")]
#[serde(try_from = "RefuelCapabilityCommandTypeSerde")]
pub enum RefuelCapabilityCommandType {
	#[doc = r#"Expansion point for commands associated with a capability of a tanker equipped with a boom."#]
	Boom(crate::v2_5::common::EmptyType),
	#[doc = r#"Commands associated for a capability for a tanker equipped with a drogue."#]
	Drogue(crate::v2_5::types::RefuelCapabilityDrogueCommandType),
}
choice_convert_impls! {
	RefuelCapabilityCommandType - RefuelCapabilityCommandTypeSerde
	Boom,
	Drogue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RefuelCapabilityStatusTypeSerde")]
#[serde(try_from = "RefuelCapabilityStatusTypeSerde")]
pub enum RefuelCapabilityStatusType {
	#[doc = r#"Expansion point for a capability for a tanker equipped with a boom."#]
	Boom(crate::v2_5::common::EmptyType),
	#[doc = r#"Status associated for a capability for a tanker equipped with a drogue."#]
	Drogue(crate::v2_5::types::RefuelCapabilityDrogueStatusType),
}
choice_convert_impls! {
	RefuelCapabilityStatusType - RefuelCapabilityStatusTypeSerde
	Boom,
	Drogue,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RefuelCommandTypeSerde")]
#[serde(try_from = "RefuelCommandTypeSerde")]
pub enum RefuelCommandType {
	#[doc = r#"Indicates a new invocation of an Refuel Capability.  Generally, if accepted, the command will result in one or more new Tanking Activities being created and reported via the TankingActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::RefuelCapabilityExtendCommandType),
	#[doc = r#"Indicates a command to modify an existing Tanking Activity (which was previously reported via the TankingActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent TankingActivity messages."#]
	Activity(crate::v2_5::types::RefuelActivityCommandType),
}
choice_convert_impls! {
	RefuelCommandType - RefuelCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RefuelConnectionTypeSerde")]
#[serde(try_from = "RefuelConnectionTypeSerde")]
pub enum RefuelConnectionType {
	#[doc = r#"Expansion point for an activity of a tanker equipped with a boom."#]
	Boom(crate::v2_5::common::EmptyType),
	#[doc = r#"Commands associated for an activity for a tanker equipped with a drogue."#]
	Drogue(crate::v2_5::types::RefuelActivityDrogueCommandType),
}
choice_convert_impls! {
	RefuelConnectionType - RefuelConnectionTypeSerde
	Boom,
	Drogue,
}

#[doc = r#"Describes the relationship between two identified objects."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RelationshipTypeSerde")]
#[serde(try_from = "RelationshipTypeSerde")]
pub enum RelationshipType {
	#[doc = r#"Reports an active engagement between a friendly entity and a hostile."#]
	EngagementStatus(crate::v2_5::enums::ExternalCommandExecutionStateEnum),
	#[doc = r#"Reports a pairing, coupling, or association between two objects."#]
	Pairing(crate::v2_5::enums::PairingRelationshipEnum),
	#[doc = r#"Reports an active threat between a hostile entity and a friendly."#]
	Threat(crate::v2_5::types::RelationshipThreatType),
	#[doc = r#"Reports a suspected association between two emitters."#]
	ElectronicWarfare(crate::v2_5::types::RelationshipEwType),
	#[doc = r#"Reports a control relationship between the Source and Destination. This is separate from the control status reported in a ControlStatus message. The distinction is required to communicate control of vehicles that aren't UCI Systems. This would occur if the vehicle is controlled by voice or a data link that is not being received."#]
	ControllingUnit(crate::v2_5::types::RelationshipControllingUnitType),
}
choice_convert_impls! {
	RelationshipType - RelationshipTypeSerde
	EngagementStatus,
	Pairing,
	Threat,
	ElectronicWarfare,
	ControllingUnit,
}

#[doc = r#"Encoding types for CVEnumISMCATRelTo Version 2 controlled vocabulary enumerations.  Derived from the CVEnumISMCATRelTo.xml CVE.(U) 
				  USA, followed by all currently valid GENC trigraphs except USA in alphabetical order by trigraph, 
				  followed by all currently valid CAPCO Coalition tetragraphs in alphabetical order by tetragraph.

						   PERMISSIBLE VALUES

						   The permissible values for this simple type are defined in the Controlled Value Enumeration:

						   CVEnumISMCATRelTo.xml"#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ReleasableToChoiceTypeSerde")]
#[serde(try_from = "ReleasableToChoiceTypeSerde")]
pub enum ReleasableToChoiceType {
	#[doc = r#"CVEnumISMCATRelTo Values"#]
	GovernmentIdentifier(crate::v2_5::enums::ReleasableToEnum),
	#[doc = r#"North Atlantic Treaty Organization Special Words"#]
	NatoSpecialWord(crate::v2_5::common::NatoSpecialWordsType),
}
choice_convert_impls! {
	ReleasableToChoiceType - ReleasableToChoiceTypeSerde
	GovernmentIdentifier,
	NatoSpecialWord,
}

#[doc = r#"Provides a choice between event types to act as a trigger for an event-based repetition."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RepetitionEventTypeSerde")]
#[serde(try_from = "RepetitionEventTypeSerde")]
pub enum RepetitionEventType {
	#[doc = r#"PositionChange is a way to specify a time based on an object change in position."#]
	PositionChange(RepetitionPositionChangeType),
	#[doc = r#"RouteEvent is a way to specify a time based on when the system/asset reaches a specific event in the life cycle of the route."#]
	RouteEvent(crate::v2_5::enums::RouteEventEnum),
	#[doc = r#"OrbitalEvent is a way to specify a time based on when a space object reaches a specific point in orbit or in life cycle."#]
	OrbitalEvent(crate::v2_5::enums::OrbitalEventEnum),
}
choice_convert_impls! {
	RepetitionEventType - RepetitionEventTypeSerde
	PositionChange,
	RouteEvent,
	OrbitalEvent,
}

#[doc = r#"Provides a choice of position delta types."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RepetitionPositionChangeTypeSerde")]
#[serde(try_from = "RepetitionPositionChangeTypeSerde")]
pub enum RepetitionPositionChangeType {
	#[doc = r#"Indicates the change in LOS from commanded System to target, which triggers repetition of the Requirement."#]
	LosBearingElevation(crate::v2_5::types::LosType),
	#[doc = r#"Indicates the change in LOS from commanded System to target, which triggers repetition of the Requirement."#]
	LosAzEl(crate::v2_5::types::LosInertialAType),
	#[doc = r#"Indicates the change in orbital RTN kinematics from the commanded System to target, which triggers repetition of the Requirement."#]
	OrbitalRtn(crate::v2_5::types::ThresholdOffOrbitTriggerDataType),
}
choice_convert_impls! {
	RepetitionPositionChangeType - RepetitionPositionChangeTypeSerde
	LosBearingElevation,
	LosAzEl,
	OrbitalRtn,
}

#[doc = r#"Represents a Time-Based repetition."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RepetitionTimeBasedTypeSerde")]
#[serde(try_from = "RepetitionTimeBasedTypeSerde")]
pub enum RepetitionTimeBasedType {
	#[doc = r#"Indicates the Requirement should be done continuously, within other temporal constraints of the Requirement. This element is oriented towards Capabilities that are continuous in nature and interruptible. This element generally shouldn't be used for Capabilities that are discrete and/or single shot in nature with a physics- and/or design-driven duration."#]
	Continuous(crate::v2_5::types::RepetitionContinuousType),
	#[doc = r#"Indicates the Requirement should be repeated a specific number of times within the overall temporal constraints of the Requirement. This element is oriented towards capabilities that are discrete and/or single shot in nature."#]
	Finite(crate::v2_5::types::RepetitionFiniteType),
	#[doc = r#"Indicates the Requirement should be repeated periodically within the overall temporal constraints of the Requirement. Depending on perspective and/or Capability type, this element indicates the time duration between starts of successive repetitions of the Requirement, time between revisits to a target or area in a search volume or other meanings. A periodic command with no end time continues until canceled. This element is oriented towards Capabilities that are discrete and/or single shot in nature."#]
	Periodic(crate::v2_5::types::RepetitionPeriodicType),
}
choice_convert_impls! {
	RepetitionTimeBasedType - RepetitionTimeBasedTypeSerde
	Continuous,
	Finite,
	Periodic,
}

#[doc = r#"Provides a choice between Time-Based and Event-Based Repetition."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RepetitionTypeSerde")]
#[serde(try_from = "RepetitionTypeSerde")]
pub enum RepetitionType {
	#[doc = r#"Indicates when the Requirement should be repeated on a Time basis."#]
	TimeBased(RepetitionTimeBasedType),
	#[doc = r#"Since it may be difficult to predict the exact time of an event, this element allows for the same input to be used in planning as the cycle moves from week-ahead planning to day-ahead planning."#]
	EventBased(crate::v2_5::types::RepetitionEventBasedType),
}
choice_convert_impls! {
	RepetitionType - RepetitionTypeSerde
	TimeBased,
	EventBased,
}

#[doc = r#"This type provides the correlation between an activity, a BIT, or a Calibration and a subsystem's RF_ResourceAllocationRequest."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequestingFunctionIdChoiceTypeSerde")]
#[serde(try_from = "RequestingFunctionIdChoiceTypeSerde")]
pub enum RequestingFunctionIdChoiceType {
	#[doc = r#"Activity ID of the activity that is requesting resources."#]
	ActivityId(crate::v2_5::types::ActivityIdType),
	#[doc = r#"BIT ID of the internal subsystem test that requires resources."#]
	BitId(crate::v2_5::types::BitIdType),
	#[doc = r#"Calibration ID of the internal subsystem calibration that requires resources."#]
	CalibrationId(crate::v2_5::types::CalibrationIdType),
}
choice_convert_impls! {
	RequestingFunctionIdChoiceType - RequestingFunctionIdChoiceTypeSerde
	ActivityId,
	BitId,
	CalibrationId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementAssociationConstraintTypeSerde")]
#[serde(try_from = "RequirementAssociationConstraintTypeSerde")]
pub enum RequirementAssociationConstraintType {
	#[doc = r#"Indicates a collection of Requirements which must all be planned and executed in order for any of them to be useful. If all of the Requirements can't be planned and executed together then none should be."#]
	AllOrNothing(crate::v2_5::types::AssociatedRequirementsType),
	#[doc = r#"Indicates a collection of Requirements in which only one should be planned and executed. If any one of the Requirements in the collection can be planned and executed then the intent is satisfied."#]
	EitherOr(crate::v2_5::types::AssociatedRequirementsType),
	#[doc = r#"Indicates a collection of Requirements which must be allocated to one or more Systems. If all Requirements must be performed this should be used with an all-or-nothing association constraint."#]
	NumberOfSystems(crate::v2_5::types::RequirementsAssociatedSystemType),
}
choice_convert_impls! {
	RequirementAssociationConstraintType - RequirementAssociationConstraintTypeSerde
	AllOrNothing,
	EitherOr,
	NumberOfSystems,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementChoiceTypeSerde")]
#[serde(try_from = "RequirementChoiceTypeSerde")]
pub enum RequirementChoiceType {
	#[doc = r#"Indicates a kind or type of Requirement."#]
	ByType(RequirementTaxonomyChoiceType),
	#[doc = r#"Indicates a specific instance of a Requirement."#]
	ByInstance(RequirementInstanceIdChoiceType),
}
choice_convert_impls! {
	RequirementChoiceType - RequirementChoiceTypeSerde
	ByType,
	ByInstance,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementInstanceIdChoiceTypeSerde")]
#[serde(try_from = "RequirementInstanceIdChoiceTypeSerde")]
pub enum RequirementInstanceIdChoiceType {
	#[doc = r#"Indicates an instance of an Effect Requirement."#]
	EffectId(crate::v2_5::types::EffectIdType),
	#[doc = r#"Indicates an instance of an Action Requirement."#]
	ActionId(crate::v2_5::types::ActionIdType),
	#[doc = r#"Indicates an instance of a Task Requirement."#]
	TaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates an instance of a [Capability]Command Requirement."#]
	CapabilityCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates an instance of a Response Requirement."#]
	ResponseId(crate::v2_5::types::ResponseIdType),
	#[doc = r#"Indicates an instance of a CommSupport Requirement."#]
	CommSupportId(crate::v2_5::types::CommSupportIdType),
}
choice_convert_impls! {
	RequirementInstanceIdChoiceType - RequirementInstanceIdChoiceTypeSerde
	EffectId,
	ActionId,
	TaskId,
	CapabilityCommandId,
	ResponseId,
	CommSupportId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementMetricsCategoryTypeSerde")]
#[serde(try_from = "RequirementMetricsCategoryTypeSerde")]
pub enum RequirementMetricsCategoryType {
	#[doc = r#"The collection metrics from the execution of a mission plan."#]
	CollectionMetrics(crate::v2_5::types::CollectionTaskMetricsType),
	#[doc = r#"The strike metrics from the execution of a mission plan."#]
	StrikeMetrics(crate::v2_5::types::StrikeTaskMetricsType),
	#[doc = r#"The comm support metrics from the execution of a mission plan."#]
	CommSupportMetrics(crate::v2_5::types::CommSupportTaskMetricsType),
}
choice_convert_impls! {
	RequirementMetricsCategoryType - RequirementMetricsCategoryTypeSerde
	CollectionMetrics,
	StrikeMetrics,
	CommSupportMetrics,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementObjectChoiceTypeSerde")]
#[serde(try_from = "RequirementObjectChoiceTypeSerde")]
pub enum RequirementObjectChoiceType {
	#[doc = r#"Indicates which Requirement actor the sibling kinematic constraints are applicable to."#]
	SingleObject(crate::v2_5::enums::RequirementObjectEnum),
	#[doc = r#"Indicates the Requirement-related objects which the sibling kinematic constraints apply to."#]
	MultiObject(crate::v2_5::enums::DependentRequirementObjectEnum),
}
choice_convert_impls! {
	RequirementObjectChoiceType - RequirementObjectChoiceTypeSerde
	SingleObject,
	MultiObject,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RequirementTaxonomyChoiceTypeSerde")]
#[serde(try_from = "RequirementTaxonomyChoiceTypeSerde")]
pub enum RequirementTaxonomyChoiceType {
	#[doc = r#"Indicates a kind of Effect Requirement."#]
	Effect(crate::v2_5::enums::EffectTypeEnum),
	#[doc = r#"Indicates a kind of Action Requirement."#]
	Action(crate::v2_5::enums::ActionTypeEnum),
	#[doc = r#"Indicates a kind of Task Requirement."#]
	Task(crate::v2_5::enums::TaskTypeEnum),
	#[doc = r#"Indicates a kind of [Capability]Command Requirement."#]
	CapabilityCommand(crate::v2_5::enums::CapabilityTypeEnum),
	#[doc = r#"Indicates a kind of Response Requirement."#]
	Response(crate::v2_5::enums::ResponseTypeEnum),
	#[doc = r#"Indicates a kind of CommSupport Requirement."#]
	CommSupport(crate::v2_5::types::CommSupportType),
}
choice_convert_impls! {
	RequirementTaxonomyChoiceType - RequirementTaxonomyChoiceTypeSerde
	Effect,
	Action,
	Task,
	CapabilityCommand,
	Response,
	CommSupport,
}

#[doc = r#"Specifies the desired aspects of the spacecraft to be characterized."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResolvedCharacterizationAspectCoverageTypeSerde")]
#[serde(try_from = "ResolvedCharacterizationAspectCoverageTypeSerde")]
pub enum ResolvedCharacterizationAspectCoverageType {
	#[doc = r#"See Base Description. List size for this element is based on "Select All That Apply" condition."#]
	BodyReference(Vec<crate::v2_5::enums::BodyReferenceEnum>),
	#[doc = r#"Specifies set of spacecraft aspects to be covered in body coordinates."#]
	Attitude(crate::v2_5::types::QuaternionType),
	#[doc = r#"Specifies span of attitude angles over which structural characterization data is required."#]
	AspectSpan(crate::v2_5::common::AngleType),
}
choice_convert_impls! {
	ResolvedCharacterizationAspectCoverageType - ResolvedCharacterizationAspectCoverageTypeSerde
	BodyReference,
	Attitude,
	AspectSpan,
}

#[doc = r#"Allows a request or allocation to be directed to either RF or digital resources."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResourceDefinitionChoiceTypeSerde")]
#[serde(try_from = "ResourceDefinitionChoiceTypeSerde")]
pub enum ResourceDefinitionChoiceType {
	#[doc = r#"Allows the requester to identify RF aperture and payload resources, connected through an RF Distribution matrix and controlled by an RF Distribution and Control Subsystem."#]
	AllocateRfResources(crate::v2_5::types::RfResourceDefinitionType),
	#[doc = r#"Allows the requester to identify digital MFA and digital MFP resources, along with sample rates and network rates, connected on a digital backbone."#]
	AllocateDigitalResources(crate::v2_5::types::DigitalResourceDefinitionType),
}
choice_convert_impls! {
	ResourceDefinitionChoiceType - ResourceDefinitionChoiceTypeSerde
	AllocateRfResources,
	AllocateDigitalResources,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResponseCommandTypeSerde")]
#[serde(try_from = "ResponseCommandTypeSerde")]
pub enum ResponseCommandType {
	#[doc = r#"Indicates a new invocation of an Response Capability.  Generally, if accepted, the command will result in one or more new Response Activities being created and reported via the ResponseActivity message.  The request/response interresponse terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interresponse with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::ResponseCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Response Activity (which was previously reported via the ResponseActivity message and was marked as "interactive").  The request/response interresponse terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent ResponseActivity messages."#]
	Activity(crate::v2_5::types::ActivityCommandBaseType),
}
choice_convert_impls! {
	ResponseCommandType - ResponseCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResponseOptionTriggerTypeSerde")]
#[serde(try_from = "ResponseOptionTriggerTypeSerde")]
pub enum ResponseOptionTriggerType {
	#[doc = r#"Indicates a trigger based on an Entity with filters/preconditions that must be met before the Option is triggered.  All immediate child element filters must be met before the Option is triggered; child elements are logically ANDed.  If a child element is omitted then any Entity matches that element's filter."#]
	Entity(crate::v2_5::types::EntityFilterType),
	#[doc = r#"Indicates a trigger based on a System with filters/preconditions that must be met before the Option is triggered.  All immediate child element filters must be met before the Option is triggered; child elements are logically ANDed.  If a child element is omitted then any  System matches that element's filter."#]
	System(crate::v2_5::types::SystemFilterType),
	#[doc = r#"Indicates a trigger based on a Requirement with filter/preconditions that must be met before the Option is triggered.  All immediate child element filters must be met before the Option is triggered; child elements are logically ANDed.  If a child element is omitted then any Requirement matches that element's filter."#]
	Requirement(crate::v2_5::types::RequirementFilterType),
	#[doc = r#"Indicates a trigger based on a Requirement with filters/preconditions that must be met before the Option is triggered.  All immediate child element filters must be met before the Option is triggered; child elements are logically ANDed.  If a child element is omitted then any Requirement matches that element's filter."#]
	AccessAssessment(crate::v2_5::types::AccessAssessmentFilterType),
	#[doc = r#"Indicates a trigger based on an OperatorLocationOfInterest (OLOI) with filters/preconditions that must be met before the Option is triggered.  All immediate child element filters must be met before the Option is triggered; child elements are logically ANDed.  If a child element is omitted then any OLOI matches that element's filter."#]
	Oloi(crate::v2_5::types::OperatorLocationOfInterestClauseType),
	#[doc = r#"Indicates a ResponseCommand message as the trigger."#]
	ResponseCommand(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates a trigger based on any UCI message, filtered/constrained by a query filter.  Filter elements with cardinality greater than 1 are logically ORed.  Sibling filter elements are logically ANDed."#]
	AnyMessage(crate::v2_5::types::QueryMessageType),
}
choice_convert_impls! {
	ResponseOptionTriggerType - ResponseOptionTriggerTypeSerde
	Entity,
	System,
	Requirement,
	AccessAssessment,
	Oloi,
	ResponseCommand,
	AnyMessage,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResponseOptionTypeSerde")]
#[serde(try_from = "ResponseOptionTypeSerde")]
pub enum ResponseOptionType {
	#[doc = r#"Indicates the Effect type to generate for use in Planned C2.  This option leaves nearly all details of the Effect to be determined by an operator or Response management service based on trigger and/or battlespace context."#]
	Effect(crate::v2_5::enums::EffectTypeEnum),
	#[doc = r#"Indicates an Effect to use as a template for generating a Response option for use in Planned C2.  The Effect indicated by this element should have a RESPONSE_C2 constraint and be used as template to copy to create a new PLANNED_C2 or DIRECT_C2 Effect with a new EffectID."#]
	EffectId(crate::v2_5::types::EffectIdType),
	#[doc = r#"Indicates the Action type to generate for use in Planned C2.  This option leaves nearly all details of the Action to be determined by an operator or Response management service based on trigger and/or battlespace context."#]
	Action(crate::v2_5::enums::ActionTypeEnum),
	#[doc = r#"Indicates an Action to use as a template for generating a Response option for use in Planned C2.  The Action indicated by this element should have a RESPONSE_C2 constraint and be used as template to copy to create a new PLANNED_C2 or DIRECT_C2 Action with a new ActionID."#]
	ActionId(crate::v2_5::types::ActionIdType),
	#[doc = r#"Indicates the Task type to generate for use in Planned C2.  This option leaves many details of the Task to be determined by an operator or Response management service based on trigger and/or battlespace context.

Note: When new Task types are added to TaskTypeEnum, they should also be added to this choice statement."#]
	Task(TaskResponseType),
	#[doc = r#"Indicates a Task to use as a template for generating a Response option for use in Planned C2.  The Task indicated by this element should have a RESPONSE_C2 constraint and be used as template to copy to create a new PLANNED_C2 or DIRECT_C2 Task with a new TaskID."#]
	TaskId(crate::v2_5::types::TaskIdType),
	#[doc = r#"Indicates the [Capability]Command type to generate along with type specific details for use in Direct C2.  This option leaves many details of the [Capability]Command to be determined by an operator or Response management service based on trigger and/or battlespace context.

Note: When new Capability types are added to CapabilityTypeEnum, they should also be added to this choice statement if they have a corresponding [Capability]Command message."#]
	CapabilityCommand(CommandResponseType),
	#[doc = r#"Indicates a [Capability]Command to use as a template for generating a new [Capability]Command (with new CapabilityCommandID) for use in Direct C2."#]
	CapabilityCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates CommSupport to use as a template for generating a Response option for use in Planned C2.  The CommSupport indicated by this element should have a RESPONSE_C2 constraint and be used as template to copy to create a new PLANNED_C2 or DIRECT_C2 CommSupport with a new CommSupportID."#]
	CommSupportId(crate::v2_5::types::CommSupportIdType),
}
choice_convert_impls! {
	ResponseOptionType - ResponseOptionTypeSerde
	Effect,
	EffectId,
	Action,
	ActionId,
	Task,
	TaskId,
	CapabilityCommand,
	CapabilityCommandId,
	CommSupportId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ResponsePlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "ResponsePlanCommandIdChoiceTypeSerde")]
pub enum ResponsePlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the ResponsePlanCommand associated with the ResponsePlan."#]
	ResponsePlanCommandId(crate::v2_5::types::ResponsePlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the ResponsePlanValidationCommand associated with the ResponsePlan."#]
	ResponsePlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the ResponsePlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the ResponsePlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	ResponsePlanCommandIdChoiceType - ResponsePlanCommandIdChoiceTypeSerde
	ResponsePlanCommandId,
	ResponsePlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RfThreadInstanceCommandTypeSerde")]
#[serde(try_from = "RfThreadInstanceCommandTypeSerde")]
pub enum RfThreadInstanceCommandType {
	#[doc = r#"Sets up the allocated RF Thread Instance that will be utilized by the subsystem (e.g. EA, ESM).  This information is passed to the Real Time Arbitration and Control component of the RFDC for configuration for future use."#]
	SetupRfThreadInstance(Vec<crate::v2_5::types::RfThreadInstanceSetupType>),
	#[doc = r#"Modify RF Thread Instances that has been setup."#]
	ModifyRfThreadInstance(Vec<crate::v2_5::types::RfThreadInstanceModifyType>),
	#[doc = r#"Remove RF Thread Instances that has been setup."#]
	RemoveRfThreadInstance(Vec<crate::v2_5::types::RfThreadInstanceRemoveType>),
}
choice_convert_impls! {
	RfThreadInstanceCommandType - RfThreadInstanceCommandTypeSerde
	SetupRfThreadInstance,
	ModifyRfThreadInstance,
	RemoveRfThreadInstance,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RfdGainSettingTypeSerde")]
#[serde(try_from = "RfdGainSettingTypeSerde")]
pub enum RfdGainSettingType {
	#[doc = r#"The valid gain range for the RFD."#]
	GainRange(crate::v2_5::types::GainRangeType),
	#[doc = r#"A list of predefined gain settings.  Can be defined as Low, Med, High, or 1,2,3, or other subsystem-specific list."#]
	ValidGainSettings(Vec<crate::v2_5::common::VisibleString32Type>),
}
choice_convert_impls! {
	RfdGainSettingType - RfdGainSettingTypeSerde
	GainRange,
	ValidGainSettings,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RollHoldOrConstraintChoiceTypeSerde")]
#[serde(try_from = "RollHoldOrConstraintChoiceTypeSerde")]
pub enum RollHoldOrConstraintChoiceType {
	#[doc = r#"Indicates navigation by roll hold."#]
	RollHold(crate::v2_5::common::AngleType),
	#[doc = r#"Indicates navigation by updated roll hold constraint, seen in the sibling Constraints element."#]
	RollHoldConstraint(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	RollHoldOrConstraintChoiceType - RollHoldOrConstraintChoiceTypeSerde
	RollHold,
	RollHoldConstraint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RollRateTypeSerde")]
#[serde(try_from = "RollRateTypeSerde")]
pub enum RollRateType {
	#[doc = r#"Indicates the navigational roll rate constraint."#]
	RollRateValue(crate::v2_5::common::AngleRateType),
	#[doc = r#"Indicates the ranged navigational roll rate constraint."#]
	RollRateRange(crate::v2_5::types::BankRateRangeType),
}
choice_convert_impls! {
	RollRateType - RollRateTypeSerde
	RollRateValue,
	RollRateRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RouteActivityPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "RouteActivityPlanCommandIdChoiceTypeSerde")]
pub enum RouteActivityPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the RouteActivityPlanCommand associated with the RouteActivityPlan."#]
	RouteActivityPlanCommandId(crate::v2_5::types::RouteActivityPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the RouteActivityPlanValidationCommand associated with the RouteActivityPlan."#]
	RouteActivityPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the RouteActivityPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the RouteActivityPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	RouteActivityPlanCommandIdChoiceType - RouteActivityPlanCommandIdChoiceTypeSerde
	RouteActivityPlanCommandId,
	RouteActivityPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RoutePlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "RoutePlanCommandIdChoiceTypeSerde")]
pub enum RoutePlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the RoutePlanCommand associated with the RoutePlan."#]
	RoutePlanCommandId(crate::v2_5::types::RoutePlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the RoutePlanValidationCommand associated with the RoutePlan."#]
	RoutePlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the RoutePlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the RoutePlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	RoutePlanCommandIdChoiceType - RoutePlanCommandIdChoiceTypeSerde
	RoutePlanCommandId,
	RoutePlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "RuleResponseTypeSerde")]
#[serde(try_from = "RuleResponseTypeSerde")]
pub enum RuleResponseType {
	#[doc = r#"Indicates a Response of processing a Requirements template."#]
	RequirementsTemplate(crate::v2_5::types::ResponseTemplateType),
	#[doc = r#"Indicates a Response of activating a specific, previously created MissionPlan or other sub-*Plan."#]
	ActivatePlan(crate::v2_5::types::MissionPlanActivationCommandType),
	#[doc = r#"Indicates a Response of generating a MissionContingencyAlert."#]
	GenerateAlert(crate::v2_5::types::ResponseAlertType),
	#[doc = r#"Indicates an explicit desire for no response when Option Rule is triggered."#]
	DoNothing(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	RuleResponseType - RuleResponseTypeSerde
	RequirementsTemplate,
	ActivatePlan,
	GenerateAlert,
	DoNothing,
}

#[doc = r#"Indicates the subcapability, SAR or ISAR, for this command."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SarCapabilityCommandSubCapabilityTypeSerde")]
#[serde(try_from = "SarCapabilityCommandSubCapabilityTypeSerde")]
pub enum SarCapabilityCommandSubCapabilityType {
	#[doc = r#"Indicates a subcapability of Synthetic Aperture Radar (SAR) for this command."#]
	Sar(crate::v2_5::types::SarSubCapabilityType),
	#[doc = r#"Indicates a subcapability of Inverse Synthetic Aperture Radar (ISAR) for this command."#]
	Isar(crate::v2_5::types::IsarSubCapabilityType),
}
choice_convert_impls! {
	SarCapabilityCommandSubCapabilityType - SarCapabilityCommandSubCapabilityTypeSerde
	Sar,
	Isar,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SarCommandTypeSerde")]
#[serde(try_from = "SarCommandTypeSerde")]
pub enum SarCommandType {
	#[doc = r#"Indicates a new invocation of a SAR Capability.  Generally, if accepted, the command will result in one or more new SAR Activities being created and reported via the SAR_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::SarCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing SAR Activity (which was previously reported via the SAR_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent SAR_Activity messages."#]
	Activity(crate::v2_5::types::SarActivityCommandType),
}
choice_convert_impls! {
	SarCommandType - SarCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SarDesiredWaveformTypeSerde")]
#[serde(try_from = "SarDesiredWaveformTypeSerde")]
pub enum SarDesiredWaveformType {
	#[doc = r#"This element defined the desired waveform to use for the collection."#]
	WaveformType(crate::v2_5::enums::SarWaveformSelectionEnum),
	#[doc = r#"This element defined the desired waveform to use for the collection as a foreign key type."#]
	ForeignWaveform(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	SarDesiredWaveformType - SarDesiredWaveformTypeSerde
	WaveformType,
	ForeignWaveform,
}

#[doc = r#"Indicates whether this is a SAR task or an ISAR task."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SarTaskTargetTypeSerde")]
#[serde(try_from = "SarTaskTargetTypeSerde")]
pub enum SarTaskTargetType {
	#[doc = r#"Indicates the details of a SAR target."#]
	Sar(crate::v2_5::types::SarTargetType),
	#[doc = r#"Indicated the details of an ISAR target."#]
	Isar(IsarTargetType),
}
choice_convert_impls! {
	SarTaskTargetType - SarTaskTargetTypeSerde
	Sar,
	Isar,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SarWaveformTypeSerde")]
#[serde(try_from = "SarWaveformTypeSerde")]
pub enum SarWaveformType {
	#[doc = r#"This element defined the desired waveform to use for the collection."#]
	WaveformType(crate::v2_5::enums::SarWaveformSelectionEnum),
	#[doc = r#"This element defined the desired waveform to use for the collection as a foreign key type."#]
	ForeignWaveform(crate::v2_5::types::ForeignKeyType),
}
choice_convert_impls! {
	SarWaveformType - SarWaveformTypeSerde
	WaveformType,
	ForeignWaveform,
}

#[doc = r#"Indicates the identity of an asset either by type or instance."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SatelliteIdentityChoiceTypeSerde")]
#[serde(try_from = "SatelliteIdentityChoiceTypeSerde")]
pub enum SatelliteIdentityChoiceType {
	#[doc = r#"Identifies the object as being a 1st-person self-reporting asset or a 3rd-person tracked asset. This option differs from the sibling "ByType" element in that "ByInstance" references UCI defined IDs."#]
	ByInstance(AssetType),
	#[doc = r#"Identifies an orbiting object via non-UCI ID designators, such as satellite numbers."#]
	ByType(crate::v2_5::types::SatelliteIdentityType),
}
choice_convert_impls! {
	SatelliteIdentityChoiceType - SatelliteIdentityChoiceTypeSerde
	ByInstance,
	ByType,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ScheduleTypeSerde")]
#[serde(try_from = "ScheduleTypeSerde")]
pub enum ScheduleType {
	#[doc = r#"Supports specifying multiple spans of time."#]
	TimeSpan(Vec<crate::v2_5::types::DateTimeRangeType>),
	#[doc = r#"Supports specifying daily periodicity."#]
	WeekdayInterval(Vec<crate::v2_5::types::WeekdayIntervalType>),
}
choice_convert_impls! {
	ScheduleType - ScheduleTypeSerde
	TimeSpan,
	WeekdayInterval,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SdaSpecialInstructionsConstraintTypeSerde")]
#[serde(try_from = "SdaSpecialInstructionsConstraintTypeSerde")]
pub enum SdaSpecialInstructionsConstraintType {
	#[doc = r#"When "All" is chosen, all of the special instructions must be accomplished to satisfy the Task."#]
	All(crate::v2_5::types::SdaSpecialInstructionsSetType),
	#[doc = r#"When "Any" is chosen, the Task can be satisfied by performing only one of the special instructions provided."#]
	Any(crate::v2_5::types::SdaSpecialInstructionsSetType),
}
choice_convert_impls! {
	SdaSpecialInstructionsConstraintType - SdaSpecialInstructionsConstraintTypeSerde
	All,
	Any,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SearchPatternTypeSerde")]
#[serde(try_from = "SearchPatternTypeSerde")]
pub enum SearchPatternType {
	#[doc = r#"Define a Commanded Timeline with Az/El Grid Control Activity Request."#]
	AzElGridControls(crate::v2_5::types::AzElGridControlsType),
	#[doc = r#"Define a Commanded Timeline with LLA Control Activity Request."#]
	LlaGridControls(crate::v2_5::types::LlaGridControlsType),
	#[doc = r#"Define a Commanded Timeline with ECEF Control Activity Request."#]
	EcefControls(crate::v2_5::types::EcefControlsType),
}
choice_convert_impls! {
	SearchPatternType - SearchPatternTypeSerde
	AzElGridControls,
	LlaGridControls,
	EcefControls,
}

#[doc = r#"Used to identify the RF payload resource which is the subject of an RF_ResourceAllocationRequest."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SelectPayloadResourceTypeSerde")]
#[serde(try_from = "SelectPayloadResourceTypeSerde")]
pub enum SelectPayloadResourceType {
	#[doc = r#"Payload Resource the requesting subsystem would like to use. Defined in Shared Aperture Information Files."#]
	PayloadResourceTypeIndex(u32),
	#[doc = r#"Payload Resource the requesting subsystem would like to use. Defined in Shared Aperture Information Files."#]
	PayloadResourceInstanceIndex(u32),
}
choice_convert_impls! {
	SelectPayloadResourceType - SelectPayloadResourceTypeSerde
	PayloadResourceTypeIndex,
	PayloadResourceInstanceIndex,
}

#[doc = r#"Indicates collection requirements for each sensor characterization choice."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SensorCharacterizationChoiceTypeSerde")]
#[serde(try_from = "SensorCharacterizationChoiceTypeSerde")]
pub enum SensorCharacterizationChoiceType {
	#[doc = r#"Indicates phenomenology specific collection requirements for a sensor task."#]
	PhemonemologySpecific(CharacterizationChoiceType),
	#[doc = r#"Indicates collection requirements to perform a stability orientation assessment sensor task."#]
	StabilityAndOrientationAssessment(crate::v2_5::types::StabilityCharacterizationType),
	#[doc = r#"Indicates collection requirements to perform a structure change detection sensor task."#]
	StructureAssessment(crate::v2_5::types::StructureAssessmentCharacterizationType),
	#[doc = r#"Indicates collection requirements to perform an identification verification sensor task."#]
	IdentificationVerification(crate::v2_5::types::IdentificationVerificationCharacterizationType),
	#[doc = r#"Indicates collection requirements to perform an operations changes sensor task."#]
	OperationsChanges(crate::v2_5::types::SatelliteOperationsChangesCharacterizationType),
}
choice_convert_impls! {
	SensorCharacterizationChoiceType - SensorCharacterizationChoiceTypeSerde
	PhemonemologySpecific,
	StabilityAndOrientationAssessment,
	StructureAssessment,
	IdentificationVerification,
	OperationsChanges,
}

#[doc = r#"See the annotation in the associated message for an overall description of the message and this type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SensorForTypeSerde")]
#[serde(try_from = "SensorForTypeSerde")]
pub enum SensorForType {
	#[doc = r#"Indicates the azimuth and elevation extents of the field of regard."#]
	Extents(crate::v2_5::types::ForExtentsType),
	#[doc = r#"Specified a geometric volume that defines the sensor field of regard."#]
	Volume(crate::v2_5::types::GeometricVolumeType),
	#[doc = r#"Indicates a longitude range corresponding to geocentric/geostationary orbits.

The Min value reflects the Western most extent and the Max value represents the Eastern most extent. As such, the Min value may be a greater number than the Max value.

Pay special attention to the values when the field crosses the Prime Meridian (0) and the 180th Meridian (-pi/pi)."#]
	GeoLongitude(crate::v2_5::types::AnglePairType),
}
choice_convert_impls! {
	SensorForType - SensorForTypeSerde
	Extents,
	Volume,
	GeoLongitude,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SensorKinematicsChoiceTypeSerde")]
#[serde(try_from = "SensorKinematicsChoiceTypeSerde")]
pub enum SensorKinematicsChoiceType {
	#[doc = r#"Indicates the kinematics of the sensor system (or geographically disperse component of a sensor system) in LLA coordinates.   Each sibling element represents the kinematics at a different time which would be used to correlate to the timestamps of the measurements. If a sensor is stationary, only one element would need to be populated."#]
	Kinematics(crate::v2_5::types::SignalNavDataType),
	#[doc = r#"Indicates the kinematics of the sensor system (or geographically disperse component of a sensor system) in space-based coordinates. Each sibling element represents the kinematics at a different time that would be used to correlate to the timestamps of the measurements."#]
	OrbitalKinematics(crate::v2_5::types::SensorKinematicsOrbitalType),
	#[doc = r#"Indicates the kinematics of the sensor system (or geographically disperse component of a sensor system) in ECEF coordinates. Each sibling element represents the kinematics at a different time that would be used to correlate to the timestamps of the measurements."#]
	EcefKinematics(crate::v2_5::types::EcefSensorKinematicsType),
}
choice_convert_impls! {
	SensorKinematicsChoiceType - SensorKinematicsChoiceTypeSerde
	Kinematics,
	OrbitalKinematics,
	EcefKinematics,
}

#[doc = r#"This is a set of points or directions desired for a sensor collection. The sensor coordinates are the origin. This can be used to specify a sensor search pattern."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SensorPointListTypeSerde")]
#[serde(try_from = "SensorPointListTypeSerde")]
pub enum SensorPointListType {
	#[doc = r#"A list of azimuth angle, elevation angle and (if desired) range distance pairs/triplets desired for a sensor collection."#]
	AzimuthElevationRangePointList(Vec<crate::v2_5::types::AzElRangePointType>),
	#[doc = r#"A list of right ascension angle and declination angle pairs desired for a sensor collection."#]
	RightAscensionDeclinationPointList(Vec<crate::v2_5::types::RightAscensionDeclinationPointType>),
	#[doc = r#"A list of latitudes, longitudes, altitudes, and optional times desired for a sensor collection."#]
	Point3Dlist(Vec<crate::v2_5::types::Point3DType>),
}
choice_convert_impls! {
	SensorPointListType - SensorPointListTypeSerde
	AzimuthElevationRangePointList,
	RightAscensionDeclinationPointList,
	Point3Dlist,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ServiceConfigurationChangeTypeSerde")]
#[serde(try_from = "ServiceConfigurationChangeTypeSerde")]
pub enum ServiceConfigurationChangeType {
	#[doc = r#"The service configuration parameters to change along with the requested new value."#]
	ParameterChange(Vec<crate::v2_5::types::ParameterAssertType>),
	#[doc = r#"ItemsAffected indicates what ancillary items were affected by the configuration change."#]
	ConfigurationFileReload(Vec<crate::v2_5::common::AttributedUriType>),
}
choice_convert_impls! {
	ServiceConfigurationChangeType - ServiceConfigurationChangeTypeSerde
	ParameterChange,
	ConfigurationFileReload,
}

#[doc = r#"This generic type provides a choice for 3-dimensional shapes (e.g. spheres, cones, etc.)."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "Shape3DChoiceTypeSerde")]
#[serde(try_from = "Shape3DChoiceTypeSerde")]
pub enum Shape3DChoiceType {
	#[doc = r#"This element describes the geometric parameters of a sphere via a radius. The location of the sphere is referenced by its center (see sibling Kinematics element for position of the center)."#]
	Sphere(crate::v2_5::types::SphereType),
	#[doc = r#"This element describes the geometric parameters of a dome via a radius. The location of the dome is referenced by the point representing the center of a sphere of the same radius; in other words, the center of the circle which is the base of the dome (see sibling Kinematics element for position of this point). The  X'-Y'-axes define a plane perpendicular to the Z' axes aligned to the center line of the dome. The attitude of the  X', Y' and Z' axes are expressed in terms of a quaternion rotation in the reference frame of choice. As an example, domes are a convenient way to represent an OpVolume relating to the ranges of ground based sites, such as SAM sites."#]
	Dome(crate::v2_5::types::DomeType),
	#[doc = r#"This element describes the geometric parameters of an ellipsoid along three axes: (1)semi-major length A, (2)semi-minor length B, and (3)semi-minor length C (the third axis perpendicular to the plane containing A and B). The location of an ellipsoid is referenced by its center (see sibling Kinematics element for position of the center). The attitude of an ellipsoid in a reference frame is represented by the body frame axes X', Y', and Z' where X is the access of the longest element of the ellipsoid (semi-major length A), and Y and Z are aligned with the semi-minor length B and semi-minor length C axes respectively. For example, an ellipsoid can express a keep out zone relative to a space based object."#]
	Ellipsoid(crate::v2_5::types::EllipsoidType),
	#[doc = r#"This element describes the geometric parameters of a cylinder via a radius and a length. If the cylinder has a length, it is finite in length; otherwise, it is infinite. The location of a cylinder is in reference to its center point (see sibling Kinematics element for position of the center) and the attitude is represented by the body frame axis Z' that is aligned to the center line of the cylinder, and the X' and Y' axes that are in the plane of the radius. For example, a cylinder can represent a region around a space object or a region of space in which it can operate or communicate with other space objects. COMM lines between two satellites."#]
	Cylinder(crate::v2_5::types::CylinderType),
	#[doc = r#"This element describes the geometric parameters of a cone via its vertex, cone half-angle, range (optional), and attitude. If the cone has a range, it is finite in length; otherwise, it is infinite. The location of a cone is in reference to the cone's vertex (see sibling Kinematics element for position of the cone's vertex).  The attitude of a cone in a reference frame is represented by the Z' body axis that is aligned to the centerline of the cone, and the X' and Y' axes that are in the plane of the radius. In the case of a sensor on a mobile platform, the cone's position may be expressed relative to the body frame of the object. A cone may represent the spatial coverage of a sensor with a conical field of view. For example, a cone can express a keep out zone relative to a sensor on Earth or a space-based sensor."#]
	Cone(crate::v2_5::types::ConeType),
	#[doc = r#"This element describes the geometric parameters of a complex cone via its vertex, inner cone half-angle, outer cone half-angle, minimum clock angle, maximum clock angle, range (optional), and attitude. If the cone has a range, it is finite in length; otherwise, it is infinite. The location of a cone is in reference to the cone's vertex (see sibling Kinematics element for position of the cone's vertex). The attitude of a cone in a reference frame is represented by the Z' body axis that is aligned to the centerline (boresight) of the cone, and the X' and Y' axes that are in the plane of the radius. In the case of a sensor on a mobile platform, the cone's position may be expressed relative to the body frame of the object. A complex cone may represent the spatial coverage of a sensor with a partially-conical field of view, such as a wedge. For example, a complex cone can express a wedge-shaped keep out zone relative to a sensor on Earth or a space-based sensor."#]
	ComplexCone(crate::v2_5::types::ComplexConeType),
	#[doc = r#"This element describes the geometric parameters of a rectangular cone via its vertex, length half-angle, width half-angle, range (optional), and attitude. The size of the half-angles is determined by the length and width half-angles relative to the cone's centerline defined by the attitude. If the rectangular cone has a range, it is finite in length; otherwise, it is infinite. The location of a rectangular cone is in reference to the rectangular cone's vertex (see sibling Kinematics element for position of the rectangular cone's vertex). The attitude of a cone in a reference frame is represented by the Z' body axis that is aligned to the centerline of the cone, and the X' and Y' axes that are in the plane perpendicular to the centerline of the cone. In the case of a sensor on a mobile platform, the cone's position may be expressed relative to the body frame of the object. A rectangular cone may represent the spatial coverage of a sensor with a conical field of view. For example, a rectangular cone can express a keep out zone relative to a sensor on Earth or a space-based sensor."#]
	RectangularCone(crate::v2_5::types::RectangularConeType),
	#[doc = r#"This element describes the geometric parameters of a volumetric arc as defined by the Radial, Along Orbit, and Cross-Track deltas from the reference object (see sibling Kinematics element for position of the center). An arch volume may represent a volume in space along circular, near-circular, or elliptical orbits. Arc volumes can be used to include space objects in keep-in zones or to exclude space objects in keep-out zones."#]
	ArcVolume(crate::v2_5::types::ArcVolumeType),
	#[doc = r#"Indicates a volume defined by Inclination, Period, and Right Ascension deltas from the reference object."#]
	IncRaPeriodVolume(crate::v2_5::types::IncRaPeriodVolumeType),
}
choice_convert_impls! {
	Shape3DChoiceType - Shape3DChoiceTypeSerde
	Sphere,
	Dome,
	Ellipsoid,
	Cylinder,
	Cone,
	ComplexCone,
	RectangularCone,
	ArcVolume,
	IncRaPeriodVolume,
}

#[doc = r#"Provides different status fields depending on the particular SupportCapability type providing the status."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SharedApertureSupportCapabilityStatusItemTypeSerde")]
#[serde(try_from = "SharedApertureSupportCapabilityStatusItemTypeSerde")]
pub enum SharedApertureSupportCapabilityStatusItemType {
	#[doc = r#"This choice provides Support Capability status of a single Antenna used by a subsystem (e.g. EA, ESM) to update the status of Subsystem Capabilities that use those Support Capabilities.  This message correlates a single Support Capability with multiple Subsystem Capabilities that use them."#]
	AntennaStatus(crate::v2_5::types::AntennaStatusType),
	#[doc = r#"This choice provides Support Capability status of a resource allocation Service to allow Subsystems (e.g. EA, ESM Subsystems) to update the status of Subsystem Capabilities and receive schedule allocation for array resources.  This message correlates a single Support Capability with multiple Subsystem Capabilities that use them."#]
	ResourceAllocatorStatus(crate::v2_5::types::ResourceAllocatorStatusType),
	#[doc = r#"This choice provides Support Capability status and settings of an RF Distribution and Control (RFDC) subsystem."#]
	RfdcStatus(crate::v2_5::types::RfdcStatusType),
}
choice_convert_impls! {
	SharedApertureSupportCapabilityStatusItemType - SharedApertureSupportCapabilityStatusItemTypeSerde
	AntennaStatus,
	ResourceAllocatorStatus,
	RfdcStatus,
}

#[doc = r#"Indicates collection requirements to perform size estimation characterization in support a structure change detection sensor task."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SizeEstimationCharacterizationTypeSerde")]
#[serde(try_from = "SizeEstimationCharacterizationTypeSerde")]
pub enum SizeEstimationCharacterizationType {
	#[doc = r#"Specifies span of time for individual collection based on duration or rotational periods of target."#]
	MinCollection(OrbitalSurveillanceSensorMinimumCollectionRequirementsType),
	#[doc = r#"Indicates the reporting requirements for RCS and Visual Magnitude sensor collections used for size estimation characterization."#]
	SizeData(crate::v2_5::enums::OrbitalSurveillanceSizeDataEnum),
}
choice_convert_impls! {
	SizeEstimationCharacterizationType - SizeEstimationCharacterizationTypeSerde
	MinCollection,
	SizeData,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SlavedNavigationTypeSerde")]
#[serde(try_from = "SlavedNavigationTypeSerde")]
pub enum SlavedNavigationType {
	#[doc = r#"Indicates navigation to a location."#]
	SlavedToTarget(crate::v2_5::types::SlavedToTargetType),
	#[doc = r#"Navigation is slaved to a Service."#]
	SlavedByServiceId(crate::v2_5::types::ServiceIdType),
	#[doc = r#"Navigation is slaved to a Capability."#]
	SlavedByCapabilityId(crate::v2_5::types::CapabilityIdType),
}
choice_convert_impls! {
	SlavedNavigationType - SlavedNavigationTypeSerde
	SlavedToTarget,
	SlavedByServiceId,
	SlavedByCapabilityId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SmtiCollectionConstraintsQualityTypeSerde")]
#[serde(try_from = "SmtiCollectionConstraintsQualityTypeSerde")]
pub enum SmtiCollectionConstraintsQualityType {
	#[doc = r#"Moving Target Indication Interpretability Rating Scale."#]
	Mtiirs(crate::v2_5::common::NiirsType),
	#[doc = r#"The radius of a circle, centered on the MTI observation, in which there is a 90 percent probability that the target lies within."#]
	CircularErrorProbable90(crate::v2_5::common::DistanceType),
}
choice_convert_impls! {
	SmtiCollectionConstraintsQualityType - SmtiCollectionConstraintsQualityTypeSerde
	Mtiirs,
	CircularErrorProbable90,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SmtiCommandTypeSerde")]
#[serde(try_from = "SmtiCommandTypeSerde")]
pub enum SmtiCommandType {
	#[doc = r#"Indicates a new invocation of an SMTI Capability.  Generally, if accepted, the command will result in one or more new SMTI Activities being created and reported via the SMTI_Activity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::SmtiCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing SMTI Activity (which was previously reported via the SMTI_Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent SMTI_Activity messages."#]
	Activity(crate::v2_5::types::SmtiActivityCommandType),
}
choice_convert_impls! {
	SmtiCommandType - SmtiCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"A choice of solar radiation pressure (SRP) coefficient to use."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SolarRadiationPressureCoefficientChoiceTypeSerde")]
#[serde(try_from = "SolarRadiationPressureCoefficientChoiceTypeSerde")]
pub enum SolarRadiationPressureCoefficientChoiceType {
	#[doc = r#"Used for VCM propagation. The SRP coefficient is defined as reflectivity coefficient times area over mass. It is a quantity with units of meters squared per kilogram. It accounts for the effects imposed by the reflectivity of the RSO with regard to the pressure exerted by solar radiation and includes mass and area."#]
	VcmSolarRadiationPressureCoefficient(f64),
	#[doc = r#"A dimensionless value based on how well the RSO reflects light that is used to help quantify the solar radiation pressure on the RSO."#]
	ReflectivityCoefficient(f64),
}
choice_convert_impls! {
	SolarRadiationPressureCoefficientChoiceType - SolarRadiationPressureCoefficientChoiceTypeSerde
	VcmSolarRadiationPressureCoefficient,
	ReflectivityCoefficient,
}

#[doc = r#"Indicates the System, Subsystem, or Service for which this applies."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SourceIdChoiceTypeSerde")]
#[serde(try_from = "SourceIdChoiceTypeSerde")]
pub enum SourceIdChoiceType {
	#[doc = r#"Indicates the System for which this applies."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the Subsystem for which this applies."#]
	SubsystemId(crate::v2_5::types::SubsystemIdType),
	#[doc = r#"Indicates the Service for which this applies."#]
	ServiceId(crate::v2_5::types::ServiceIdType),
}
choice_convert_impls! {
	SourceIdChoiceType - SourceIdChoiceTypeSerde
	SystemId,
	SubsystemId,
	ServiceId,
}

#[doc = r#"Defines the type that allows a choice of space weather data type: SpaceWeather message or static values."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SpaceWeatherDataChoiceTypeSerde")]
#[serde(try_from = "SpaceWeatherDataChoiceTypeSerde")]
pub enum SpaceWeatherDataChoiceType {
	#[doc = r#"Indicates the ID of the set of Space Weather data to be used by the atmospheric density model."#]
	SpaceWeatherValuesId(crate::v2_5::types::SpaceWeatherIdType),
	#[doc = r#"The static (non-changing) space weather values to be used by the atmospheric density model."#]
	StaticValues(crate::v2_5::types::SpaceWeatherParameterType),
}
choice_convert_impls! {
	SpaceWeatherDataChoiceType - SpaceWeatherDataChoiceTypeSerde
	SpaceWeatherValuesId,
	StaticValues,
}

#[doc = r#"Defines the type of geomagnetic index to use: Kp or Ap."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SpaceWeatherKpApChoiceTypeSerde")]
#[serde(try_from = "SpaceWeatherKpApChoiceTypeSerde")]
pub enum SpaceWeatherKpApChoiceType {
	#[doc = r#"Kp value of geomagnetic activity. Kp is an index of a 3-hourly range of geomagnetic measurements."#]
	Kp(crate::v2_5::common::GeomagneticKpIndexType),
	#[doc = r#"Ap value of geomagnetic activity. Ap is derived from Kp values."#]
	Ap(crate::v2_5::common::GeomagneticApIndexType),
}
choice_convert_impls! {
	SpaceWeatherKpApChoiceType - SpaceWeatherKpApChoiceTypeSerde
	Kp,
	Ap,
}

#[doc = r#"See the annotation in the associated message for an overall description of the message and this type."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SpeedChoiceTypeSerde")]
#[serde(try_from = "SpeedChoiceTypeSerde")]
pub enum SpeedChoiceType {
	#[doc = r#"Indicates the speed that the vehicle shall traverse the path segment."#]
	SpeedValue(crate::v2_5::types::PathSegmentSpeedValueType),
	#[doc = r#"Indicates the range of speed that the vehicle shall traverse the path segment."#]
	SpeedValueRange(crate::v2_5::types::PathSegmentSpeedValueRangeType),
	#[doc = r#"Indicates the unitless Mach number realized at a particular speed during flight."#]
	MachValue(crate::v2_5::common::MachType),
	#[doc = r#"Indicates a min/max range for a mach-based speed."#]
	MachRange(crate::v2_5::types::MachRangeType),
}
choice_convert_impls! {
	SpeedChoiceType - SpeedChoiceTypeSerde
	SpeedValue,
	SpeedValueRange,
	MachValue,
	MachRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StoreCommandTypeSerde")]
#[serde(try_from = "StoreCommandTypeSerde")]
pub enum StoreCommandType {
	#[doc = r#"For stores systems that must move stores into release position, this commands the stores manager to move the specified store station into release position."#]
	NextStoreStation(crate::v2_5::types::ForeignKeyType),
	#[doc = r#"For carriage systems that can carry a mixed load, this dictates what type of store is associated with the next release command.  The currently installed store types are reported in the StoreManagementStatus messages.  This is an alternative to selecting a specific store instance for release using the sibling NextStoreStation element."#]
	NextStoreType(crate::v2_5::types::StoreType),
	#[doc = r#"When true, this commands the carriage to ignore LAR constraints when releasing a store."#]
	OverrideLar(bool),
	#[doc = r#"When true, this commands the carriage to ignore the safety constraints placed on the host platform's attitude when releasing a store."#]
	OverrideAttitudeConstraints(bool),
	#[doc = r#"When true, the carriage is commanded to the armed state."#]
	MasterArm(bool),
	#[doc = r#"This provides the carriage the operator's consent to release or jettison a store."#]
	ReleaseConsent(crate::v2_5::types::ReleaseConsentType),
	#[doc = r#"TRUE = hold the current wind velocity reported by the host platform and stop accepting updates.

FALSE = begin accepting wind velocity updates from the host platform and replace the previously held or override value."#]
	LarCalculationWindHold(bool),
	#[doc = r#"By specifying the wind velocity, the command source dictates the winds to be used in the calculation of the LAR for the hosted expendables."#]
	LarCalculationWindOverride(crate::v2_5::types::Velocity2DType),
	#[doc = r#"Command to verify carriages and mission stores loaded onto a platform match an allowable configuration."#]
	VerifyInventory(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	StoreCommandType - StoreCommandTypeSerde
	NextStoreStation,
	NextStoreType,
	OverrideLar,
	OverrideAttitudeConstraints,
	MasterArm,
	ReleaseConsent,
	LarCalculationWindHold,
	LarCalculationWindOverride,
	VerifyInventory,
}

#[doc = r#"What store type Mission or Carriage."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StoreItemTypeSerde")]
#[serde(try_from = "StoreItemTypeSerde")]
pub enum StoreItemType {
	#[doc = r#"A mission store in a loadout (i.e. a weapon)."#]
	Mission(crate::v2_5::types::StoreLoadoutMissionType),
	#[doc = r#"A carriage within a loadout which may contain one or more weapons or other carriages."#]
	Carriage(crate::v2_5::types::StoreLoadoutCarriageType),
}
choice_convert_impls! {
	StoreItemType - StoreItemTypeSerde
	Mission,
	Carriage,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StoreLoadoutChoiceTypeSerde")]
#[serde(try_from = "StoreLoadoutChoiceTypeSerde")]
pub enum StoreLoadoutChoiceType {
	#[doc = r#"The list of all StoreType held  at this location and by any child StoreType carriage items in the order they appear in StoreType list."#]
	StoreList(Vec<crate::v2_5::types::StoreLoadoutItemPet>),
	#[doc = r#"A uci:EmptyType used to signal the end of recursion."#]
	Terminator(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	StoreLoadoutChoiceType - StoreLoadoutChoiceTypeSerde
	StoreList,
	Terminator,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StrikeCommandTypeSerde")]
#[serde(try_from = "StrikeCommandTypeSerde")]
pub enum StrikeCommandType {
	#[doc = r#"Indicates a new invocation of a Strike Capability.  Generally, if accepted, the command will result in one or more new Strike Activities being created and reported via the StrikeActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::StrikeCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Strike Activity (which was previously reported via the StrikeActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent StrikeActivity messages."#]
	Activity(crate::v2_5::types::StrikeActivityCommandType),
}
choice_convert_impls! {
	StrikeCommandType - StrikeCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StrikeTaskMetricsTargetingTypeSerde")]
#[serde(try_from = "StrikeTaskMetricsTargetingTypeSerde")]
pub enum StrikeTaskMetricsTargetingType {
	#[doc = r#"This element represents the positional accuracy of the entity.  It is the area, in square data miles, within which it is assessed that there is a 95% probability that the track/point lies."#]
	Quality(f32),
	#[doc = r#"This element represents an ellipse describing the position uncertainty.  The ellipse is defined by the length of its semi-major and semi-minor axes.  The orientation of the ellipse defines the angle between the semi-major axis and true north."#]
	UncertaintyEllipse(crate::v2_5::types::EllipseType),
}
choice_convert_impls! {
	StrikeTaskMetricsTargetingType - StrikeTaskMetricsTargetingTypeSerde
	Quality,
	UncertaintyEllipse,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StrikeTaskReleaseConstraintsTypeSerde")]
#[serde(try_from = "StrikeTaskReleaseConstraintsTypeSerde")]
pub enum StrikeTaskReleaseConstraintsType {
	#[doc = r#"Release point within a Launch Acceptable Region (LAR)."#]
	ReleasePoint(crate::v2_5::types::Point3DType),
	#[doc = r#"Defines an area where release is acceptable."#]
	ReleaseArea(crate::v2_5::types::AreaConstraintsType),
}
choice_convert_impls! {
	StrikeTaskReleaseConstraintsType - StrikeTaskReleaseConstraintsTypeSerde
	ReleasePoint,
	ReleaseArea,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StrikeWeaponCommandTypeSerde")]
#[serde(try_from = "StrikeWeaponCommandTypeSerde")]
pub enum StrikeWeaponCommandType {
	#[doc = r#"If true, this store will be commanded into a state where it is ready to receive cryptographic keys."#]
	SelectForKeyLoad(bool),
	#[doc = r#"The identifier used to uniquely address the target."#]
	AssignTarget(GeoLocatedObjectType),
	#[doc = r#"When true, the weapon is commanded to the armed state."#]
	WeaponArm(bool),
	#[doc = r#"If true, this store will be selected for ejection from the host platform without being armed or otherwise prepared for launch.  The next time all release conditions are met and release consent is provided this store will be jettisoned."#]
	SelectForJettison(bool),
	#[doc = r#"Indicates the weapon will calculate the LAR information."#]
	GenerateDynamicLar(bool),
	#[doc = r#"If true, the next time all release conditions are met and release consent is provided this store will be released."#]
	SelectForRelease(bool),
	#[doc = r#"Indicates a PRF and/or PIM code supported by this store."#]
	SelectAoCode(crate::v2_5::types::AoCodeType),
}
choice_convert_impls! {
	StrikeWeaponCommandType - StrikeWeaponCommandTypeSerde
	SelectForKeyLoad,
	AssignTarget,
	WeaponArm,
	SelectForJettison,
	GenerateDynamicLar,
	SelectForRelease,
	SelectAoCode,
}

#[doc = r#"Indicates a task and requirements to enable characterization of or assessment of changes to the structure of a spacecraft."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "StructureAssessmentTypeSerde")]
#[serde(try_from = "StructureAssessmentTypeSerde")]
pub enum StructureAssessmentType {
	#[doc = r#"Task for collections to enable the estimation of the size of the object."#]
	SizeEstimation(crate::v2_5::types::SizeEstimationType),
	#[doc = r#"Conduct collection (and appropriate processing) to support assessment of length, width, height of a spacecraft as well as similar values and relative positions/orientations of spacecraft components (e.g., solar panels, telescopes, antennae)."#]
	Resolved(crate::v2_5::types::ResolvedCharacterizationType),
}
choice_convert_impls! {
	StructureAssessmentType - StructureAssessmentTypeSerde
	SizeEstimation,
	Resolved,
}

#[doc = r#"Indicates whether an UCI Entity subject or an UCI System subject is to be selected."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubjectTypeSerde")]
#[serde(try_from = "SubjectTypeSerde")]
pub enum SubjectType {
	#[doc = r#"Indicates the unique ID of a UCI System subject.  See the SystemStatus message annotations for details of what it means to be a UCI System."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the unique ID of a UCI Entity subject.  See the Entity message annotations for details of what it means to be a UCI Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
}
choice_convert_impls! {
	SubjectType - SubjectTypeSerde
	SystemId,
	EntityId,
}

#[doc = r#"Identifies specific BIT IDs or Fault codes relevant to this command."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubsystemBitCommandTypeSerde")]
#[serde(try_from = "SubsystemBitCommandTypeSerde")]
pub enum SubsystemBitCommandType {
	#[doc = r#"Indicates the unique ID or IDs of the BIT or BITs to initiate."#]
	InitiateBitId(Vec<crate::v2_5::types::BitIdType>),
	#[doc = r#"Indicates the unique ID or IDs of the BIT or BITs to cancel."#]
	CancelBitId(Vec<crate::v2_5::types::BitIdType>),
	#[doc = r#"Indicates the "codes" or names of the faults to clear."#]
	ClearFaultCode(Vec<crate::v2_5::common::VisibleString256Type>),
}
choice_convert_impls! {
	SubsystemBitCommandType - SubsystemBitCommandTypeSerde
	InitiateBitId,
	CancelBitId,
	ClearFaultCode,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubsystemCalibrationCommandIdChoiceTypeSerde")]
#[serde(try_from = "SubsystemCalibrationCommandIdChoiceTypeSerde")]
pub enum SubsystemCalibrationCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the Calibration to initiate."#]
	InitiateCalibrationId(crate::v2_5::types::CalibrationIdType),
	#[doc = r#"Indicates the unique ID of the Calibration to cancel."#]
	CancelCalibrationId(crate::v2_5::types::CalibrationIdType),
}
choice_convert_impls! {
	SubsystemCalibrationCommandIdChoiceType - SubsystemCalibrationCommandIdChoiceTypeSerde
	InitiateCalibrationId,
	CancelCalibrationId,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubsystemMaintenanceSubtestChoiceTypeSerde")]
#[serde(try_from = "SubsystemMaintenanceSubtestChoiceTypeSerde")]
pub enum SubsystemMaintenanceSubtestChoiceType {
	#[doc = r#"The list of Subtests."#]
	Subtest(Vec<crate::v2_5::types::SubsystemMaintenanceTestPet>),
	#[doc = r#"A uci:EmptyType used to signal the end of recursion."#]
	Terminator(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	SubsystemMaintenanceSubtestChoiceType - SubsystemMaintenanceSubtestChoiceTypeSerde
	Subtest,
	Terminator,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubsystemMaintenanceSubtestCommandChoiceTypeSerde")]
#[serde(try_from = "SubsystemMaintenanceSubtestCommandChoiceTypeSerde")]
pub enum SubsystemMaintenanceSubtestCommandChoiceType {
	#[doc = r#"The list of Subtests."#]
	Subtest(Vec<crate::v2_5::types::SubsystemMaintenanceTestCommandPet>),
	#[doc = r#"A uci:EmptyType used to signal the end of recursion."#]
	Terminator(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	SubsystemMaintenanceSubtestCommandChoiceType - SubsystemMaintenanceSubtestCommandChoiceTypeSerde
	Subtest,
	Terminator,
}

#[doc = r#"A choice type to allow further recursion or a terminator to signal the end of recursion."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SubsystemMaintenanceSubtestResultChoiceTypeSerde")]
#[serde(try_from = "SubsystemMaintenanceSubtestResultChoiceTypeSerde")]
pub enum SubsystemMaintenanceSubtestResultChoiceType {
	#[doc = r#"List of test results associated with the sibling TestID. This may be a partial or full list. In the CompletedTest, this element will provide list of completed or failed maintenance test unit results.  In the CurrentTest, this element will provide list of results for maintenance test units in process or in queue."#]
	SubtestResultData(Vec<crate::v2_5::types::SubsystemMaintenanceTestResultPet>),
	#[doc = r#"A uci:EmptyType used to signal the end of recursion."#]
	Terminator(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	SubsystemMaintenanceSubtestResultChoiceType - SubsystemMaintenanceSubtestResultChoiceTypeSerde
	SubtestResultData,
	Terminator,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SupportApprovalItemTypeSerde")]
#[serde(try_from = "SupportApprovalItemTypeSerde")]
pub enum SupportApprovalItemType {
	#[doc = r#"Indicates the approval item is a SupportPlan.  This approval is given in response to a SupportPlan in the PENDING_APPROVAL or APPROVAL_REQUESTED state."#]
	SupportPlan(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the approval item is a SupportPlan activation state transition to the indicated state."#]
	ActivationStateTransition(crate::v2_5::enums::SupportPlanActivationRequestEnum),
	#[doc = r#"Indicates the approval item is a support Requirement for execution.  This approval is given in response to a support Requirement in the PENDING_APPROVAL or APPROVAL_REQUESTED state."#]
	RequirementExecution(crate::v2_5::types::ApprovalRequestItemType),
}
choice_convert_impls! {
	SupportApprovalItemType - SupportApprovalItemTypeSerde
	SupportPlan,
	ActivationStateTransition,
	RequirementExecution,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SupportManagementRequestTypeSerde")]
#[serde(try_from = "SupportManagementRequestTypeSerde")]
pub enum SupportManagementRequestType {
	#[doc = r#"Indicates a request to replan the subject SupportPlan.  A replan is based on the set of Requirements and constraints from the SupportPlanRequest message(s) that led to the SupportPlan.  Other data is used from "current" sources.  For example current supporting system availability, battlespace conditions and threat locations could be different than when the SupportPlan was created.  A replan updates the SupportPlan to reflect current conditions.  Note that "current" has a different meaning in pre-mission planning vs. live mission planning.  Results of the replan are reported in the corresponding SupportPlanStatus message."#]
	Replan(crate::v2_5::types::SupportReplanRequestType),
	#[doc = r#"Indicates a request to validate the subject SupportPlan.  Results of the validation are reported in the corresponding SupportPlanStatus message."#]
	Validation(crate::v2_5::types::SupportValidationRequestType),
	#[doc = r#"Indicates a "request" to give an approval response related to the subject SupportPlan."#]
	ApprovalResponse(crate::v2_5::types::SupportApprovalResponseType),
	#[doc = r#"Indicates a request to transition the subject SupportPlan to the specified activation state.  Note that this enumerated choice can be used to request activation and deactivation of a SupportPlan.  Activated SupportPlans are stopped by deactivating them, not by cancelling them.  SupportPlans should only be cancelled prior to activation.  Note that the state machine that includes cancellation is in the SupportPlanStatus message while the state machine for activation state is in the SupportPlanExecutionStatus."#]
	Activation(crate::v2_5::enums::SupportPlanActivationRequestEnum),
	#[doc = r#"Indicates a request to cancel a Requirement which has been allocated and planned when a replan to deallocate the Requirement isn't possible or can't be initiated by the cancelling actor.  This request is analogous to the  [Requirement]CancelCommand* messages used in non-Support use cases.  This request should result in the Requirement being canceled unless execution was already inevitable.  If the Requirement was inevitable and started executing but can be stopped (such as a long imagery collection) it should cancel as soon as possible, prior to completion.  Cancelling a Requirement doesn't necessarily trigger a replan, therefore a System could continue to navigate routes related to the canceled Requirement, even though the Requirement won't occur.  A subsequent replan could exclude the Requirement.  Status or feedback for this request is given through SupportManagementRequestStatus but the resulting status of the Requirement is given through [Requirement]Status messages."#]
	CancelRequirement(RequirementInstanceIdChoiceType),
}
choice_convert_impls! {
	SupportManagementRequestType - SupportManagementRequestTypeSerde
	Replan,
	Validation,
	ApprovalResponse,
	Activation,
	CancelRequirement,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SupportRequestTypeSerde")]
#[serde(try_from = "SupportRequestTypeSerde")]
pub enum SupportRequestType {
	#[doc = r#"Indicates a new request for support.  Generally, if accepted, the request will result in one or more new Activities being created and reported via the corresponding *Activity message.  The request/response interaction terminates as soon as the request is accepted or rejected; this element is not used to interact with an "active" request.  Updates and/or additional interaction with the resulting Activity (after the request is accepted) is accomplished via the sibling ActivityUpdate element."#]
	NewRequest(crate::v2_5::types::SupportRequestNewType),
	#[doc = r#"Indicates a request to modify an existing support Activity (which was previously reported via the corresponding *Activity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent *Activity messages."#]
	ActivityUpdate(crate::v2_5::types::SupportRequestActivityUpdateType),
}
choice_convert_impls! {
	SupportRequestType - SupportRequestTypeSerde
	NewRequest,
	ActivityUpdate,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SystemCharacteristicTypeSerde")]
#[serde(try_from = "SystemCharacteristicTypeSerde")]
pub enum SystemCharacteristicType {
	#[doc = r#"Indicates an identity to be compared to the identity of the System."#]
	Identity(EntityIdentityChoiceType),
	#[doc = r#"Indicates the position uncertainty to be compared to the position uncertainty of the System's kinematics.  This element represents the positional accuracy of the target.  It is the area, in square data miles, within which it is assessed that there is a 95% probability that the target lies.  The value given in this element can be compared to either the Quality element of the target or a calculated uncertainty area for the target based on its uncertainty ellipse."#]
	PositionUncertainty(f32),
	#[doc = r#"Indicates the position staleness to be compared to the kinematic staleness of the System's kinematics."#]
	PositionStaleness(chrono::TimeDelta),
	#[doc = r#"Indicates a value priority/rank  to be compared to the priority/rank in PrioritizationList referencing the System."#]
	PrioritizationList(crate::v2_5::types::PrioritizationListValueType),
	#[doc = r#"Indicates behaviors, activities, use of capabilities, etc. to be compared to those of the battlespace object associated with the System."#]
	Behavior(crate::v2_5::types::BehaviorType),
}
choice_convert_impls! {
	SystemCharacteristicType - SystemCharacteristicTypeSerde
	Identity,
	PositionUncertainty,
	PositionStaleness,
	PrioritizationList,
	Behavior,
}

#[doc = r#"Indicates whether an ElementSet, EntityElementSetID, KinematicVector, SystemVCM_ID, or OrbitPlanID will be used to determine the ephemeris."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SystemEphemerisBasisChoiceTypeSerde")]
#[serde(try_from = "SystemEphemerisBasisChoiceTypeSerde")]
pub enum SystemEphemerisBasisChoiceType {
	#[doc = r#"Indicates the default or "catalog" orbital element set (also known as two line element or TLE) for the satellite."#]
	ElementSet(crate::v2_5::types::TleBaseType),
	#[doc = r#"The unique system identifier that correspond to the orbital element set (TLE)."#]
	SystemElementSetId(crate::v2_5::types::SystemOrbitalElementSetIdType),
	#[doc = r#"Specifies the kinematics in terms of a standard coordinate reference frame."#]
	KinematicVector(OrbitalKinematicsStandardFrameChoiceType),
	#[doc = r#"Propagate using a complete VCM (vector covariance message) which includes a kinematic vector, along with other special perturbation parameters."#]
	SystemVcmId(crate::v2_5::types::SystemOrbitalVcmIdType),
	#[doc = r#"Specifies the Orbit Plan ID that the ephemeris should be based on."#]
	OrbitPlanId(crate::v2_5::types::OrbitPlanIdType),
}
choice_convert_impls! {
	SystemEphemerisBasisChoiceType - SystemEphemerisBasisChoiceTypeSerde
	ElementSet,
	SystemElementSetId,
	KinematicVector,
	SystemVcmId,
	OrbitPlanId,
}

#[doc = r#"Indicates the initial conditions for a system estimation."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SystemEstimationStartTypeSerde")]
#[serde(try_from = "SystemEstimationStartTypeSerde")]
pub enum SystemEstimationStartType {
	#[doc = r#"This element contains the time to use for the start of the estimation."#]
	StartTime(chrono::DateTime<chrono::Utc>),
	#[doc = r#"This element contains the estimation initial conditions for a route based system.  If this element is populated with a segment ID and SegmentCaptureTime is omitted, then the use of current system time is appropriate."#]
	RouteEstimationStart(crate::v2_5::types::EstimationStartType),
	#[doc = r#"This element contains the estimation initial conditions for a space based system."#]
	OrbitEstimationStart(crate::v2_5::types::OrbitEstimationStartType),
}
choice_convert_impls! {
	SystemEstimationStartType - SystemEstimationStartTypeSerde
	StartTime,
	RouteEstimationStart,
	OrbitEstimationStart,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SystemEstimationStopTypeSerde")]
#[serde(try_from = "SystemEstimationStopTypeSerde")]
pub enum SystemEstimationStopType {
	#[doc = r#"This element is used to specify the time at which to stop estimation."#]
	Time(chrono::DateTime<chrono::Utc>),
	#[doc = r#"This element is used to specify the route segment at which to stop estimation."#]
	RouteSegmentId(crate::v2_5::types::SegmentIdType),
	#[doc = r#"This element is used to specify the orbit maneuver segment at which to stop estimation."#]
	OrbitManeuverSegmentId(crate::v2_5::types::OrbitManeuverSegmentIdType),
}
choice_convert_impls! {
	SystemEstimationStopType - SystemEstimationStopTypeSerde
	Time,
	RouteSegmentId,
	OrbitManeuverSegmentId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "SystemManagementRequestTypeSerde")]
#[serde(try_from = "SystemManagementRequestTypeSerde")]
pub enum SystemManagementRequestType {
	#[doc = r#"Indicates a request to modify the mode of a system."#]
	SetMode(crate::v2_5::enums::MessageModeEnum),
	#[doc = r#"Indicates a request to set the identity of a system."#]
	SetIdentity(crate::v2_5::types::SystemIdentityType),
	#[doc = r#"Indicates a request to modify the Link16_Metadata of a system."#]
	SetLink16Metadata(crate::v2_5::types::Link16MetadataType),
	#[doc = r#"Set the voice control frequency of a system."#]
	SetVoiceControl(crate::v2_5::types::VoiceControlType),
	#[doc = r#"When TRUE, indicates an order to report all locally derived sensor, signal, track or Entity data.  When FALSE, indicates an order to stop reporting."#]
	SetSensorEntityReporting(bool),
	#[doc = r#"Indicates a request to modify vehicle settings."#]
	VehicleSettings(crate::v2_5::types::VehicleCommandDataType),
}
choice_convert_impls! {
	SystemManagementRequestType - SystemManagementRequestTypeSerde
	SetMode,
	SetIdentity,
	SetLink16Metadata,
	SetVoiceControl,
	SetSensorEntityReporting,
	VehicleSettings,
}

#[doc = r#"Indicates the target of a TagAssociation, which could be a message or a string value."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TagAssociationTargetTypeSerde")]
#[serde(try_from = "TagAssociationTargetTypeSerde")]
pub enum TagAssociationTargetType {
	#[doc = r#"Indicates a message as the TagAssociation target."#]
	ByMessage(crate::v2_5::types::AssociatedMessageType),
	#[doc = r#"Indicates the TagAssociation target."#]
	ByValue(crate::v2_5::types::SecureStringType),
}
choice_convert_impls! {
	TagAssociationTargetType - TagAssociationTargetTypeSerde
	ByMessage,
	ByValue,
}

#[doc = r#"Indicates or references geospatial characteristics of a target."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TargetTypeSerde")]
#[serde(try_from = "TargetTypeSerde")]
pub enum TargetType {
	#[doc = r#"Indicates the target is an Entity."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Indicates the target is a System."#]
	SystemId(crate::v2_5::types::SystemIdType),
	#[doc = r#"Indicates the target is an OperatorLocationOfInterest and provides its ID."#]
	OperatorLocationOfInterestId(crate::v2_5::types::OperatorLocationOfInterestIdType),
	#[doc = r#"Indicates the target is a Signal."#]
	SignalId(crate::v2_5::types::SignalIdType),
	#[doc = r#"Indicates the target is an OpPoint."#]
	OpPointId(crate::v2_5::types::OpPointIdType),
	#[doc = r#"Indicates the target is an OpZone."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"Indicates the target is an OpVolume."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
	#[doc = r#"Indicates the target is an OpLine."#]
	OpLineId(crate::v2_5::types::OpLineIdType),
	#[doc = r#"Indicates the target is a geospatial point."#]
	PointTarget(crate::v2_5::types::PointTargetType),
	#[doc = r#"Indicates the target is an orbital point."#]
	OrbitPointTarget(OrbitalKinematicsChoiceType),
	#[doc = r#"Indicates the target is a geospatial zone."#]
	ZoneTarget(crate::v2_5::types::ZoneExternalType),
	#[doc = r#"Indicates the target is a geospatial volume."#]
	VolumeTarget(OpVolumeType),
	#[doc = r#"Indicates the target is a geospatial line."#]
	LineTarget(crate::v2_5::types::LineTargetType),
}
choice_convert_impls! {
	TargetType - TargetTypeSerde
	EntityId,
	SystemId,
	OperatorLocationOfInterestId,
	SignalId,
	OpPointId,
	OpZoneId,
	OpVolumeId,
	OpLineId,
	PointTarget,
	OrbitPointTarget,
	ZoneTarget,
	VolumeTarget,
	LineTarget,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TaskPlanCommandIdChoiceTypeSerde")]
#[serde(try_from = "TaskPlanCommandIdChoiceTypeSerde")]
pub enum TaskPlanCommandIdChoiceType {
	#[doc = r#"Indicates the unique ID of the TaskPlanCommand associated with the TaskPlan."#]
	TaskPlanCommandId(crate::v2_5::types::TaskPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the TaskPlanValidationCommand associated with the TaskPlan."#]
	TaskPlanValidationCommandId(crate::v2_5::types::CommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanCommand associated with the TaskPlan."#]
	MissionPlanCommandId(crate::v2_5::types::MissionPlanCommandIdType),
	#[doc = r#"Indicates the unique ID of the MissionPlanValidationCommand associated with the TaskPlan."#]
	MissionPlanValidationCommandId(crate::v2_5::types::CommandIdType),
}
choice_convert_impls! {
	TaskPlanCommandIdChoiceType - TaskPlanCommandIdChoiceTypeSerde
	TaskPlanCommandId,
	TaskPlanValidationCommandId,
	MissionPlanCommandId,
	MissionPlanValidationCommandId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TaskResponseTypeSerde")]
#[serde(try_from = "TaskResponseTypeSerde")]
pub enum TaskResponseType {
	#[doc = r#"Indicates a desire to create an AirSample Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	AirSample(crate::v2_5::types::AirSampleTaskBaseType),
	#[doc = r#"Indicates a desire to create an AMTI Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Amti(crate::v2_5::types::AmtiTaskBaseType),
	#[doc = r#"Indicates a desire to create an AO Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Ao(crate::v2_5::types::AoTaskBaseType),
	#[doc = r#"Indicates a desire to create a CargoDelivery Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	CargoDelivery(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates a desire to create a COMINT Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Comint(crate::v2_5::types::ComintTaskBaseType),
	#[doc = r#"Indicates a desire to create a CommSupport Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	CommSupport(crate::v2_5::types::CommSupportTaskBaseType),
	#[doc = r#"Indicates a desire to create a CounterSpace Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	CounterSpace(crate::v2_5::types::CounterSpaceTaskBaseType),
	#[doc = r#"Indicates a desire to create a DE Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	De(crate::v2_5::types::DeTaskBaseType),
	#[doc = r#"Indicates a desire to create an ElectronicAttack Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Ea(crate::v2_5::types::EaResponseType),
	#[doc = r#"Indicates a desire to create an ESM Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Esm(crate::v2_5::types::EsmTaskBaseType),
	#[doc = r#"Indicates a desire to create a Flight Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Flight(crate::v2_5::types::FlightTaskBaseType),
	#[doc = r#"Indicates a desire to create a Navigation Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Navigation(crate::v2_5::types::NavigationTaskType),
	#[doc = r#"Indicates a desire to create an Orbit Change Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	OrbitChange(crate::v2_5::types::OrbitChangeTaskBaseType),
	#[doc = r#"Indicates a desire to create an Orbital Surveillance Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	OrbitalSurveillance(crate::v2_5::types::OrbitalSurveillanceTaskBaseType),
	#[doc = r#"Indicates a desire to create an Orbital Surveillance Sensor Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	OrbitalSurveillanceSensor(crate::v2_5::types::OrbitalSurveillanceSensorTaskBaseType),
	#[doc = r#"Indicates a desire to create a PO Task as a mission response. If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Po(crate::v2_5::types::PoTaskBaseType),
	#[doc = r#"Indicates a desire to create a Refuel Task as a mission response. If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Refuel(crate::v2_5::types::RefuelTaskBaseType),
	#[doc = r#"Indicates a desire to create a SAR Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Sar(crate::v2_5::types::SarTaskBaseType),
	#[doc = r#"Indicates a desire to create a SMTI Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Smti(crate::v2_5::types::SmtiTaskBaseType),
	#[doc = r#"Indicates a desire to create a Strike Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	Strike(crate::v2_5::types::StrikeTaskWeaponListType),
	#[doc = r#"Indicates a desire to create a System Deployment Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	SystemDeployment(crate::v2_5::types::SystemDeploymentTaskBaseType),
	#[doc = r#"Indicates a desire to create a Tactical Order Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	TacticalOrder(crate::v2_5::types::TacticalOrderTaskBaseType),
	#[doc = r#"Indicates a desire to create a WeatherRadar Task as a mission response.  If details aren't specified, they are left to the discretion of the Task creator and/or implication of the response trigger."#]
	WeatherRadar(crate::v2_5::common::EmptyType),
}
choice_convert_impls! {
	TaskResponseType - TaskResponseTypeSerde
	AirSample,
	Amti,
	Ao,
	CargoDelivery,
	Comint,
	CommSupport,
	CounterSpace,
	De,
	Ea,
	Esm,
	Flight,
	Navigation,
	OrbitChange,
	OrbitalSurveillance,
	OrbitalSurveillanceSensor,
	Po,
	Refuel,
	Sar,
	Smti,
	Strike,
	SystemDeployment,
	TacticalOrder,
	WeatherRadar,
}

#[doc = r#"Identifies the type of this Task instance. Note: When modifying this complexType (whether adding or removing choices), there are equivalent complexTypes that require the same modifications. Changes to this type may necessitate a modification to CapabilityTaxonomyType."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TaskTypeSerde")]
#[serde(try_from = "TaskTypeSerde")]
pub enum TaskType {
	#[doc = r#"Air sample includes direct sampling of the air (SAMPLE) and remote sensing with spectral analysis (SPECTROMETER) with the intent of detecting NBC events."#]
	AirSample(crate::v2_5::types::AirSampleTaskType),
	#[doc = r#"Indicates a Task to collect Air Moving Target Indicator (AMTI) data."#]
	Amti(crate::v2_5::types::AmtiTaskType),
	#[doc = r#"Indicates a Task to perform an optical emission such as laser designation."#]
	Ao(crate::v2_5::types::AoTaskType),
	#[doc = r#"Indicates a Task to transfer cargo between locations."#]
	CargoDelivery(CargoDeliveryTaskType),
	#[doc = r#"Indicates a Task to provide COMINT."#]
	Comint(crate::v2_5::types::ComintTaskType),
	#[doc = r#"Indicates a Task to provide communications."#]
	CommSupport(crate::v2_5::types::CommSupportTaskType),
	#[doc = r#"Indicates a Task to employ a CounterSpace capability."#]
	CounterSpace(crate::v2_5::types::CounterSpaceTaskType),
	#[doc = r#"Indicates a Task to perform a Directed Energy emission."#]
	De(crate::v2_5::types::DeTaskType),
	#[doc = r#"Indicates a Task to provide electronic attack support to another System.  It guides/constrains the EA System by specifying where it should fly, what it should protect and what the threat is."#]
	Ea(crate::v2_5::types::EaTaskType),
	#[doc = r#"Indicates a Task to collect ESM data."#]
	Esm(crate::v2_5::types::EsmTaskType),
	#[doc = r#"Indicates a Task to effect the flight path/plan of the System."#]
	Flight(crate::v2_5::types::FlightTaskType),
	#[doc = r#"Indicates a non-orbital Navigation Task."#]
	Navigation(crate::v2_5::types::NavigationTaskType),
	#[doc = r#"Indicates a task to perform an orbit change via a spacecraft maneuver."#]
	OrbitChange(crate::v2_5::types::OrbitChangeTaskType),
	#[doc = r#"Indicates an Orbital Surveillance Task."#]
	OrbitalSurveillance(crate::v2_5::types::OrbitalSurveillanceTaskType),
	#[doc = r#"Indicates a task to perform orbital surveillance sensor tasking."#]
	OrbitalSurveillanceSensor(crate::v2_5::types::OrbitalSurveillanceSensorTaskType),
	#[doc = r#"Indicates a Task to collect Passive Optical data, imagery and video as well as perform PO search and track capabilities."#]
	Po(crate::v2_5::types::PoTaskType),
	#[doc = r#"Indicates a Task for one System to refuel another."#]
	Refuel(crate::v2_5::types::RefuelTaskType),
	#[doc = r#"Indicates a Task to collect a Synthetic Aperture Radar (SAR) image."#]
	Sar(crate::v2_5::types::SarTaskType),
	#[doc = r#"Indicates a Task to collect Moving Target Indicator (MTI) data."#]
	Smti(crate::v2_5::types::SmtiTaskType),
	#[doc = r#"Indicates a Task to kinetically attack/strike, with a weapon that can be released from the System."#]
	Strike(crate::v2_5::types::StrikeTaskType),
	#[doc = r#"Indicates a task to perform a deployment or release of a system at a specified location."#]
	SystemDeployment(crate::v2_5::types::SystemDeploymentTaskType),
	#[doc = r#"Indicates a task to perform a tactical order."#]
	TacticalOrder(crate::v2_5::types::TacticalOrderTaskType),
	#[doc = r#"Indicates a task to collect weather radar data."#]
	WeatherRadar(crate::v2_5::types::WeatherRadarTaskType),
}
choice_convert_impls! {
	TaskType - TaskTypeSerde
	AirSample,
	Amti,
	Ao,
	CargoDelivery,
	Comint,
	CommSupport,
	CounterSpace,
	De,
	Ea,
	Esm,
	Flight,
	Navigation,
	OrbitChange,
	OrbitalSurveillance,
	OrbitalSurveillanceSensor,
	Po,
	Refuel,
	Sar,
	Smti,
	Strike,
	SystemDeployment,
	TacticalOrder,
	WeatherRadar,
}

#[doc = r#"Provides a choice of timing constraints including repetitive timing and event based repetition."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TimingConstraintsTypeSerde")]
#[serde(try_from = "TimingConstraintsTypeSerde")]
pub enum TimingConstraintsType {
	#[doc = r#"Indicates that the timing should be As Soon As Possible."#]
	AsSoonAsPossible(crate::v2_5::common::EmptyType),
	#[doc = r#"Indicates the time window during which the Requirement should be initiated and, once initiated, the duration."#]
	TimeWindow(crate::v2_5::types::TimeWindowType),
	#[doc = r#"Indicates a recurring time.  For example, a recurring time as a constraint on a Task may result in the need for a Task or other Requirement to be broken down into multiple discrete children corresponding to each interval."#]
	WeekdayInterval(crate::v2_5::types::WeekdayIntervalType),
	#[doc = r#"Indicates a repetition and defines the number and frequency of product collection and generation such as start/stop/frequency or min # of collections."#]
	Repetitive(crate::v2_5::types::RepetitionConstraintsType),
}
choice_convert_impls! {
	TimingConstraintsType - TimingConstraintsTypeSerde
	AsSoonAsPossible,
	TimeWindow,
	WeekdayInterval,
	Repetitive,
}

#[doc = r#"Choice between a Link 16 TN or UCI EntityID_Type value."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TrackNumberOrEntityTypeSerde")]
#[serde(try_from = "TrackNumberOrEntityTypeSerde")]
pub enum TrackNumberOrEntityType {
	#[doc = r#"UUID of the referenced vehicle."#]
	EntityId(crate::v2_5::types::EntityIdType),
	#[doc = r#"Link 16 Track Number of the referenced vehicle. Used to reference TNs without a matching UCI object."#]
	TrackNumber(crate::v2_5::types::Link16TrackIdentifierType),
}
choice_convert_impls! {
	TrackNumberOrEntityType - TrackNumberOrEntityTypeSerde
	EntityId,
	TrackNumber,
}

#[doc = r#"This element is used to specify whether a turn is a bank angle or turn radius."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TurnGeometryChoiceTypeSerde")]
#[serde(try_from = "TurnGeometryChoiceTypeSerde")]
pub enum TurnGeometryChoiceType {
	#[doc = r#"Indicates the radius of the turn.  When neither BankAngle nor TurnRadius are specified, indicates that the TurnPoint is in the end of a turn."#]
	TurnRadiusValue(crate::v2_5::common::DistanceType),
	#[doc = r#"Indicates the radius range of the turn.  When neither BankAngle nor TurnRadius are specified, indicates that the TurnPoint is in the end of a turn."#]
	TurnRadiusRange(crate::v2_5::types::TurnRadiusRangeType),
	#[doc = r#"Indicates bank angle of the turn.  When neither BankAngle nor TurnRadius are specified, indicates that the TurnPoint is in the end of a turn."#]
	BankAngleValue(crate::v2_5::common::AngleHalfType),
	#[doc = r#"Indicates bank angle range of the turn.  When neither BankAngle nor TurnRadius are specified, indicates that the TurnPoint is in the end of a turn."#]
	BankAngleRange(crate::v2_5::types::BankAngleRangeType),
}
choice_convert_impls! {
	TurnGeometryChoiceType - TurnGeometryChoiceTypeSerde
	TurnRadiusValue,
	TurnRadiusRange,
	BankAngleValue,
	BankAngleRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TurnRateTypeSerde")]
#[serde(try_from = "TurnRateTypeSerde")]
pub enum TurnRateType {
	#[doc = r#"Indicates the navigational turn rate constraint."#]
	TurnRateValue(crate::v2_5::common::SpeedType),
	#[doc = r#"Indicates the ranged navigational turn rate constraint."#]
	TurnRateRange(crate::v2_5::types::TurnRateRangeType),
}
choice_convert_impls! {
	TurnRateType - TurnRateTypeSerde
	TurnRateValue,
	TurnRateRange,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TurretCommandChoiceTypeSerde")]
#[serde(try_from = "TurretCommandChoiceTypeSerde")]
pub enum TurretCommandChoiceType {
	#[doc = r#"Enumeration defining the Gimbal Modes that must be explicitly commanded. Other modes can be reached by generating an appropriate command (e.g. sending a platform referenced Position command will put the turret in a body fixed mode, while sending a PointTarget will put it into a geo-referenced mode)."#]
	FixedMode(crate::v2_5::enums::FixedPointingEnum),
	#[doc = r#"Command to specify the look angle of the gimbal using Azimuth/Elevation and Reference Frame (INERTIAL/BODY) specifications."#]
	Position(crate::v2_5::types::TurretCommandPositionType),
	#[doc = r#"Command to specify the look angle of the gimbal. The option provides two choices: LOS which includes Reference Frame, Azimuth, Elevation, Roll and associated LOS Rates or LOS_Rates which allows for Azimuth, Elevation and Roll Rate settings."#]
	LosPosition(LosDType),
	#[doc = r#"Indicates the volume or specific Entity that defines the extents of the spatial region covered by the command. For non-cued Capabilities, one air volume instance of this element is expected. For cued Capabilities, two instances of this element are allowed: one with an Entity specified and a second with an air volume that define the extents of the cue volume around the Entity. For cued Capabilities, a single instance is allowed if it is an Entity in which case the Entity location uncertainty defines the extents of the cue volume."#]
	Volume(PoAirTargetVolumeCommandType),
	#[doc = r#"Indicates the source of or explicit values for geospatial characteristics of the target of the Command."#]
	Geospatial(TargetType),
}
choice_convert_impls! {
	TurretCommandChoiceType - TurretCommandChoiceTypeSerde
	FixedMode,
	Position,
	LosPosition,
	Volume,
	Geospatial,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "TurretReportPointingTypeSerde")]
#[serde(try_from = "TurretReportPointingTypeSerde")]
pub enum TurretReportPointingType {
	#[doc = r#"Enumeration defining the Gimbal Modes that must be explicitly commanded. Other modes can be reached by generating an appropriate command (e.g. sending a platform referenced Position command will put the turret in a body fixed mode, while sending a PointTarget will put it into a geo-referenced mode)."#]
	FixedPointing(crate::v2_5::enums::FixedPointingEnum),
	#[doc = r#"Specifies the look angle of the gimbal using heading and pitch and optional heading and pitch rate of change."#]
	Dynamic(crate::v2_5::types::TurretReportDynamicPointingType),
}
choice_convert_impls! {
	TurretReportPointingType - TurretReportPointingTypeSerde
	FixedPointing,
	Dynamic,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "UsmtfIdentificationChoiceTypeSerde")]
#[serde(try_from = "UsmtfIdentificationChoiceTypeSerde")]
pub enum UsmtfIdentificationChoiceType {
	#[doc = r#"Indicates the unique operation name, nickname or code word that identifies a specific operation. Derived from MIL-STD-6040."#]
	OperationCodeword(crate::v2_5::common::UsmtfOperationCodewordType),
	#[doc = r#"The unique code name or nickname assigned to a joint exercise or plan or to designate exercise message traffic. Derived from MIL-STD-6040 message schema."#]
	ExerciseNickname(crate::v2_5::common::UsmtfExerciseNicknameType),
}
choice_convert_impls! {
	UsmtfIdentificationChoiceType - UsmtfIdentificationChoiceTypeSerde
	OperationCodeword,
	ExerciseNickname,
}

#[doc = r#"The ID type for UCI IDs that correspond to a Validator."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ValidatorTypeSerde")]
#[serde(try_from = "ValidatorTypeSerde")]
pub enum ValidatorType {
	#[doc = r#"Indicates validation by a human operator."#]
	Operator(OperatorReferenceType),
	#[doc = r#"Indicates automated validation by a System."#]
	System(crate::v2_5::types::SystemServiceType),
}
choice_convert_impls! {
	ValidatorType - ValidatorTypeSerde
	Operator,
	System,
}

#[doc = r#"Unique identifier for a vehicle. This will differ depending on the type of vehicle. Air platforms will have a Tail Number. Space assets will have a Sat ID. Ships and Subs will have AIS Numbers."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "VehicleUniqueIdentifierTypeSerde")]
#[serde(try_from = "VehicleUniqueIdentifierTypeSerde")]
pub enum VehicleUniqueIdentifierType {
	#[doc = r#"Indicates select maritime Automatic Identification System (AIS) settings for the asset/vessel."#]
	Ais(crate::v2_5::types::AisType),
	#[doc = r#"Unique identifier for air platforms.  See type annotations for further details."#]
	TailNumber(crate::v2_5::common::TailNumberType),
	#[doc = r#"The Satellite Identifier used to identify a Satellite.  See type annotations for further details."#]
	Satellite(crate::v2_5::types::SatelliteIdentifierType),
	#[doc = r#"Unique identifier that does not fall into any of the other categories. e.g. A Land based vehicle."#]
	AlternateIdentifier(crate::v2_5::common::AlphanumericDashSpaceUnderscoreString20Type),
}
choice_convert_impls! {
	VehicleUniqueIdentifierType - VehicleUniqueIdentifierTypeSerde
	Ais,
	TailNumber,
	Satellite,
	AlternateIdentifier,
}

#[doc = r#"Video encoder output defines the multicast or file to contain the output from an encoder."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "VideoEncoderOutputTypeSerde")]
#[serde(try_from = "VideoEncoderOutputTypeSerde")]
pub enum VideoEncoderOutputType {
	#[doc = r#"Configure a Multicast or Broadcast IP connection for the video encoder."#]
	SocketAddress(IpConnectionChoiceType),
	#[doc = r#"File(s) to output the encoded video data. Type of file could be deduced from the file name extension or from the Type field."#]
	File(crate::v2_5::types::FileNameAndOutputType),
}
choice_convert_impls! {
	VideoEncoderOutputType - VideoEncoderOutputTypeSerde
	SocketAddress,
	File,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "VolumeChoiceTypeSerde")]
#[serde(try_from = "VolumeChoiceTypeSerde")]
pub enum VolumeChoiceType {
	#[doc = r#"Indicates a volume defined by an existing OpVolume."#]
	OpVolumeId(crate::v2_5::types::OpVolumeIdType),
	#[doc = r#"Indicates a volume defined directly here."#]
	VolumeDefinition(OpVolumeType),
}
choice_convert_impls! {
	VolumeChoiceType - VolumeChoiceTypeSerde
	OpVolumeId,
	VolumeDefinition,
}

#[doc = r#"Location associated with the specified waypoint expressed as either a geospatial or relative point."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WayPointPointChoiceTypeSerde")]
#[serde(try_from = "WayPointPointChoiceTypeSerde")]
pub enum WayPointPointChoiceType {
	#[doc = r#"Indicates the geospatial point corresponding to the waypoint.  Generally, services are encouraged to send altitude and/or time data whenever it is known."#]
	Point2D(crate::v2_5::types::Point2DType),
	#[doc = r#"Indicates the Relative point corresponding to the waypoint.  Generally, services are encouraged to send altitude data whenever it is known."#]
	RelativePoint(crate::v2_5::types::Point2DRelativeType),
}
choice_convert_impls! {
	WayPointPointChoiceType - WayPointPointChoiceTypeSerde
	Point2D,
	RelativePoint,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeaponRestrictionTypeSerde")]
#[serde(try_from = "WeaponRestrictionTypeSerde")]
pub enum WeaponRestrictionType {
	#[doc = r#"Only weapons of this type may be used against a target type and or within a zone."#]
	WeaponsAllowed(Vec<crate::v2_5::types::StoreType>),
	#[doc = r#"Weapons of this type are restricted against a target type and or within a zone."#]
	WeaponsNotAllowed(Vec<crate::v2_5::types::StoreType>),
}
choice_convert_impls! {
	WeaponRestrictionType - WeaponRestrictionTypeSerde
	WeaponsAllowed,
	WeaponsNotAllowed,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeaponTargetPairingChoiceTypeSerde")]
#[serde(try_from = "WeaponTargetPairingChoiceTypeSerde")]
pub enum WeaponTargetPairingChoiceType {
	#[doc = r#"Indicates a weapon-target pair or pairs given in a DMPI_Designation."#]
	DmpiDesignationId(crate::v2_5::types::DmpiDesignationIdType),
	#[doc = r#"Indicates a weapon-target pair or pairs given in a DMPI_Pattern."#]
	DmpiPatternId(crate::v2_5::types::DmpiPatternIdType),
	#[doc = r#"Indicates a weapon-target pair given in a DMPI."#]
	DmpiId(Vec<crate::v2_5::types::DmpiIdType>),
	#[doc = r#"Indicates a weapon-target pair expressed directly, independent of DMPIs."#]
	Weaponeering(Vec<crate::v2_5::types::WeaponeeringLocationType>),
}
choice_convert_impls! {
	WeaponTargetPairingChoiceType - WeaponTargetPairingChoiceTypeSerde
	DmpiDesignationId,
	DmpiPatternId,
	DmpiId,
	Weaponeering,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeatherChoiceTypeSerde")]
#[serde(try_from = "WeatherChoiceTypeSerde")]
pub enum WeatherChoiceType {
	#[doc = r#"Indicates weather data described by an existing WeatherDataset message."#]
	WeatherDatasetId(crate::v2_5::types::WeatherDatasetIdType),
	#[doc = r#"Indicates weather data described directly here."#]
	WeatherData(crate::v2_5::types::WeatherAreaDataType),
}
choice_convert_impls! {
	WeatherChoiceType - WeatherChoiceTypeSerde
	WeatherDatasetId,
	WeatherData,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeatherDataTypeSerde")]
#[serde(try_from = "WeatherDataTypeSerde")]
pub enum WeatherDataType {
	#[doc = r#"This element represents a report of current or forecasted weather."#]
	WeatherReport(crate::v2_5::types::WeatherReportDataType),
	#[doc = r#"This element represents a weather warning.  The time period over which the warning applies is given in the Zone element of the WeatherArea element."#]
	WeatherWarning(crate::v2_5::types::WeatherWarningDataType),
	#[doc = r#"This element references a weather product that is defined by ProductMetadata and ProductLocation messages. An example of this type of product would be a GRIB file."#]
	WeatherProductId(crate::v2_5::types::ProductMetadataIdType),
}
choice_convert_impls! {
	WeatherDataType - WeatherDataTypeSerde
	WeatherReport,
	WeatherWarning,
	WeatherProductId,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeatherRadarCommandTypeSerde")]
#[serde(try_from = "WeatherRadarCommandTypeSerde")]
pub enum WeatherRadarCommandType {
	#[doc = r#"Indicates a new invocation of a Weather Radar Capability.  Generally, if accepted, the command will result in one or more new Weather Radar Activities being created and reported via the WeatherRadarActivity message.  The request/response interaction terminates as soon as the command is accepted or rejected; this element is not used to interact with an "active" command.  Updates and/or additional interaction with the resulting Activity (after the command is accepted) is accomplished via the sibling Activity element."#]
	Capability(crate::v2_5::types::WeatherRadarCapabilityCommandType),
	#[doc = r#"Indicates a command to modify an existing Weather Radar Activity (which was previously reported via the WeatherRadarActivity message and was marked as "interactive").  The request/response interaction terminates as soon as the modification is accepted or rejected.  The modifications are reflected in subsequent WeatherRadarActivity messages."#]
	Activity(crate::v2_5::types::RadarActivityCommandType),
}
choice_convert_impls! {
	WeatherRadarCommandType - WeatherRadarCommandTypeSerde
	Capability,
	Activity,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WeatherReportTypeSerde")]
#[serde(try_from = "WeatherReportTypeSerde")]
pub enum WeatherReportType {
	#[doc = r#"This element represents a summary of the weather conditions across the entire weather area."#]
	AreaData(crate::v2_5::types::WeatherAreaDataType),
	#[doc = r#"This element represents a point at which Weather applies.  This allows weather to be expressed as a grid of points across the weather area."#]
	GridData(Vec<crate::v2_5::types::WeatherReportGridDataType>),
}
choice_convert_impls! {
	WeatherReportType - WeatherReportTypeSerde
	AreaData,
	GridData,
}

#[doc = r#"Indicates wind as a velocity or as a magnitude and speed value."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "WindDataChoiceTypeSerde")]
#[serde(try_from = "WindDataChoiceTypeSerde")]
pub enum WindDataChoiceType {
	#[doc = r#"Indicates wind speed and direction."#]
	WindVelocity(crate::v2_5::types::Velocity2DType),
	#[doc = r#"Indicates a direction and a wind speed ."#]
	WindMagnitude(crate::v2_5::types::WindMagnitudeType),
}
choice_convert_impls! {
	WindDataChoiceType - WindDataChoiceTypeSerde
	WindVelocity,
	WindMagnitude,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ZChoiceTypeSerde")]
#[serde(try_from = "ZChoiceTypeSerde")]
pub enum ZChoiceType {
	#[doc = r#"Offset in the Z direction from the reference frame origin. Down is positive."#]
	Z(crate::v2_5::common::DistanceOffsetType),
	#[doc = r#"Offset in the Z direction is considered an altitude difference with positive values corresponding to increases in altitude."#]
	AltitudeOffset(crate::v2_5::types::AltitudeOffsetReferenceType),
	#[doc = r#"Optional choice to use Absolute Altitude per altitude reference instead of a vertical offset from the reference frame origin."#]
	AbsoluteAltitude(crate::v2_5::types::AltitudeReferenceType),
}
choice_convert_impls! {
	ZChoiceType - ZChoiceTypeSerde
	Z,
	AltitudeOffset,
	AbsoluteAltitude,
}

#[doc = r#"See annotations in child elements and messages/elements that use this type for details."#]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(into = "ZoneChoiceTypeSerde")]
#[serde(try_from = "ZoneChoiceTypeSerde")]
pub enum ZoneChoiceType {
	#[doc = r#"Indicates a zone defined by an existing OpZone."#]
	OpZoneId(crate::v2_5::types::OpZoneIdType),
	#[doc = r#"Indicates a zone defined directly here."#]
	ZoneDefinition(crate::v2_5::types::ZoneExternalType),
}
choice_convert_impls! {
	ZoneChoiceType - ZoneChoiceTypeSerde
	OpZoneId,
	ZoneDefinition,
}

