#![allow(non_snake_case)]
#![allow(dead_code)]
use c2rust_bitfields::BitfieldStruct;
use crate::sdk::vector;

use super::vector::CVector;

type RwObject = u64;
type CReference = u64;
type CTreadable = u64;
type CEntryInfoList = u64;
type CPtrNode = u64;
type CEntity = u64;

const PHYSICAL_MAX_COLLISIONRECORDS: usize = 6;

#[repr(C, align(1))]
#[derive(BitfieldStruct)]
pub struct CPed {

    ////////////////////////////////////////////////////////////////
    /// CEntity
    ////////////////////////////////////////////////////////////////
    pub CPlaceable : [u8; 0x58], // CColModel
    pub m_rwObject : *mut RwObject,
    _type : u16,
    _status : u16,

    // flagsA
	#[bitfield(name = "bUsesCollision", ty = "libc::c_uchar", bits = "0..=0")]			// does entity use collision
	#[bitfield(name = "bCollisionProcessed", ty = "libc::c_uchar", bits = "1..=1")]		// has object been processed by a ProcessEntityCollision function
	#[bitfield(name = "bIsStatic", ty = "libc::c_uchar", bits = "2..=2")]				// is entity static
	#[bitfield(name = "bHasContacted", ty = "libc::c_uchar", bits = "3..=3")]			// has entity processed some contact forces
	#[bitfield(name = "bPedPhysics", ty = "libc::c_uchar", bits = "4..=4")]
	#[bitfield(name = "bIsStuck", ty = "libc::c_uchar", bits = "5..=5")]				// is entity stuck
	#[bitfield(name = "bIsInSafePosition", ty = "libc::c_uchar", bits = "6..=6")]		// is entity in a collision free safe position
	#[bitfield(name = "bUseCollisionRecords", ty = "libc::c_uchar", bits = "7..=7")]

	// flagsB
	#[bitfield(name = "bWasPostponed", ty = "libc::c_uchar", bits = "8..=8")]			// was entity control processing postponed
	#[bitfield(name = "bExplosionProof", ty = "libc::c_uchar", bits = "9..=9")]
	#[bitfield(name = "bIsVisible", ty = "libc::c_uchar", bits = "10..=10")]				//is the entity visible
	#[bitfield(name = "bHasCollided", ty = "libc::c_uchar", bits = "11..=11")]
	#[bitfield(name = "bRenderScorched", ty = "libc::c_uchar", bits = "12..=12")]
	#[bitfield(name = "bHasBlip", ty = "libc::c_uchar", bits = "13..=13")]
	#[bitfield(name = "bIsBIGBuilding", ty = "libc::c_uchar", bits = "14..=14")]			// Set if this entity is a big building
	#[bitfield(name = "bRenderDamaged", ty = "libc::c_uchar", bits = "15..=15")]			// use damaged LOD models for objects with applicable damage

	// flagsC
	#[bitfield(name = "bBulletProof", ty = "libc::c_uchar", bits = "16..=16")]
	#[bitfield(name = "bFireProof", ty = "libc::c_uchar", bits = "17..=17")]
	#[bitfield(name = "bCollisionProof", ty = "libc::c_uchar", bits = "18..=18")]
	#[bitfield(name = "bMeleeProof", ty = "libc::c_uchar", bits = "19..=19")]
	#[bitfield(name = "bOnlyDamagedByPlayer", ty = "libc::c_uchar", bits = "20..=20")]
	#[bitfield(name = "bStreamingDontDelete", ty = "libc::c_uchar", bits = "21..=21")]	// Dont let the streaming remove this 
	#[bitfield(name = "bZoneCulled", ty = "libc::c_uchar", bits = "22..=22")]
	#[bitfield(name = "bZoneCulled2", ty = "libc::c_uchar", bits = "23..=23")]    // only treadables+10m

	// flagsD
	#[bitfield(name = "bRemoveFromWorld", ty = "libc::c_uchar", bits = "24..=24")]		// remove this entity next time it should be processed
	#[bitfield(name = "bHasHitWall", ty = "libc::c_uchar", bits = "25..=25")]				// has collided with a building (changes subsequent collisions)
	#[bitfield(name = "bImBeingRendered", ty = "libc::c_uchar", bits = "26..=26")]		// don't delete me because I'm being rendered
	#[bitfield(name = "bTouchingWater", ty = "libc::c_uchar", bits = "27..=27")]	// used by cBuoyancy::ProcessBuoyancy
	#[bitfield(name = "bIsSubway", ty = "libc::c_uchar", bits = "28..=28")]	// set when subway, but maybe different meaning?
	#[bitfield(name = "bDrawLast", ty = "libc::c_uchar", bits = "29..=29")]				// draw object last
	#[bitfield(name = "bNoBrightHeadLights", ty = "libc::c_uchar", bits = "30..=30")]
	#[bitfield(name = "bDoNotRender", ty = "libc::c_uchar", bits = "31..=31")]
    _flags: [u8; 4],

	// flagsE
	// #[bitfield(name = "bDistanceFade", ty = "libc::c_uchar", bits = "32..=32")]			// Fade entity because it is far away
	// #[bitfield(name = "m_flagE2", ty = "libc::c_uchar", bits = "33..=33")]
    // _flagsE: [u8; 1],
    // _pad: [u8; 1],

    m_scanCode: u16,
	m_randomSeed: u16,
    m_modelIndex: i16,
	m_level: u16,	// int16
	m_pFirstReference: *mut CReference,

    ////////////////////////////////////////////////////////////////
    /// CPhysical
    ////////////////////////////////////////////////////////////////
    m_audioEntityId: i32,
	m_phys_unused1: f32,
	m_treadable: *mut [CTreadable; 2],	// car and ped
	m_nLastTimeCollided: u32,
	m_vecMoveSpeed: CVector,		// velocity
	m_vecTurnSpeed: CVector,		// angular velocity
	m_vecMoveFriction: CVector,
	m_vecTurnFriction: CVector,
	m_vecMoveSpeedAvg: CVector,
	m_vecTurnSpeedAvg: CVector,
	m_fMass: f32,
	m_fTurnMass: f32,	// moment of inertia
	m_fForceMultiplier: f32,
	m_fAirResistance: f32,
	m_fElasticity: f32,
	m_fBuoyancy: f32,
	m_vecCentreOfMass: CVector,
	m_entryInfoList: CEntryInfoList,
	m_movingListNode: *mut CPtrNode,

	m_phys_unused2: i8,
	m_nStaticFrames: u8,
	m_nCollisionRecords: u8,
	m_bIsVehicleBeingShifted: bool,
	m_aCollisionRecords: *mut [CEntity; PHYSICAL_MAX_COLLISIONRECORDS],

	m_fDistanceTravelled: f32,

	// // damaged piece
	m_fDamageImpulse: f32,
	m_pDamageEntity: *mut CEntity,
	m_vecDamageNormal: CVector,
	m_nDamagePieceType: i16,

	#[bitfield(name = "bIsHeavy", ty = "libc::c_uchar", bits = "0..=0")]
	#[bitfield(name = "bAffectedByGravity", ty = "libc::c_uchar", bits = "1..=1")]
	#[bitfield(name = "bInfiniteMass", ty = "libc::c_uchar", bits = "2..=2")]
	#[bitfield(name = "bIsInWater", ty = "libc::c_uchar", bits = "3..=3")]
	#[bitfield(name = "m_phy_flagA10", ty = "libc::c_uchar", bits = "4..=4")]
	#[bitfield(name = "m_phy_flagA20", ty = "libc::c_uchar", bits = "5..=5")]
	#[bitfield(name = "bHitByTrain", ty = "libc::c_uchar", bits = "6..=6")]
	#[bitfield(name = "bSkipLineCol", ty = "libc::c_uchar", bits = "7..=7")]
    _flags2: [u8; 8],

	m_nSurfaceTouched: u8,
	m_nZoneLevel: i8,
    _pad: [u8; 7],
}