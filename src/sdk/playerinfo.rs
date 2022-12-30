#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::modules::memory::{get_symbol_addr, read_mem};

use super::vector::CVector;

enum WastedBustedState
{
	WbstatePlaying,
	WbstateWasted,
	WbstateBusted,
	WbstateFailedCriticalMission,
}

type CPed = i64;
type CVehicle = i64;
type CCivilianPed = i64;
type RwTexture = i64;
type CChar = u8;

#[repr(C)]
pub struct CPlayerInfo {
	pub m_pPed : *mut CPed,
    pub m_pRemoteVehicle : *mut CVehicle,
	pub m_ColModel : [u8; 112], // CColModel
	pub m_pVehicleEx : *mut CVehicle,	// vehicle using the col model above
	pub m_aPlayerName : [CChar; 70],
	pub m_nMoney : i32,
	pub m_nVisibleMoney : i32,
	pub m_nCollectedPackages : i32,
	pub m_nTotalPackages : i32,
	pub m_nLastBumpPlayerCarTimer : u32,
	pub m_nUnusedTaxiTimer : u32,
	pub m_bUnusedTaxiThing : bool,
	pub m_nNextSexFrequencyUpdateTime : u32,
	pub m_nNextSexMoneyUpdateTime : u32,
	pub m_nSexFrequency : i32,
	pub m_pHooker : *mut CCivilianPed,
	pub m_WBState : u8, // eWastedBustedState
	pub m_nWBTime : u32,
	pub m_bInRemoteMode : bool,
	pub m_nTimeLostRemoteCar : u32,
	pub m_nTimeLastHealthLoss : u32,
	pub m_nTimeLastArmourLoss : u32,
	pub m_nTimeTankShotGun : u32,
	pub m_nUpsideDownCounter : i32,
	pub field_248 : i32,
	pub m_nTrafficMultiplier : i16,
	pub m_fRoadDensity : f32,
	pub m_nPreviousTimeRewardedForExplosion : u32,
	pub m_nExplosionsSinceLastReward : i32,
	pub field_268 : i32,
	pub field_272 : i32,
	pub m_bInfiniteSprint : bool,
	pub m_bFastReload : bool,
	pub m_bGetOutOfJailFree : bool,
	pub m_bGetOutOfHospitalFree : bool,
	pub m_aSkinName : [CChar; 32],
	pub m_pSkinTexture : *mut RwTexture,

	// const CVector &GetPos();
	// void KillPlayer(void);
	// void ArrestPlayer(void);
	// void PlayerFailedCriticalMission(void);
	// void Clear(void);
	// void BlowUpRCBuggy(void);
	// void CancelPlayerEnteringCars(CVehicle*);
	// bool IsRestartingAfterDeath(void);
	// bool IsRestartingAfterArrest(void);
	// void EvaluateCarPosition(CEntity*, CPed*, float, float*, CVehicle**);
	// void LoadPlayerInfo(uint8 *buf, size);
	// void SavePlayerInfo(uint8 *buf, uint32* size);
	// void FindClosestCarSectorList(CPtrList&, CPed*, float, float, float, float, float*, CVehicle**);

	// void LoadPlayerSkin();
	// void SetPlayerSkin(const char *skin);
	// void DeletePlayerSkin();
}

lazy_static! {
    static ref CPLAYER_INFO: u64 = get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A");
}

impl CPlayerInfo {

	// Returns static ref to CPlayerInfo
	pub fn get_mut() -> &'static mut CPlayerInfo {
		let pinfo: &mut CPlayerInfo = unsafe {std::mem::transmute(*CPLAYER_INFO)};
		pinfo
	}

	// Retuns player's current position in world
	pub fn get_pos(&self) -> *mut CVector {
		let addr = get_symbol_addr("?GetPos@CPlayerInfo@@QEAAAEBVCVector@@XZ");
        unsafe {
            type Func = extern "fastcall" fn(&CPlayerInfo) -> *mut CVector;
            let func: Func = std::mem::transmute(addr); 
            func(self)
        }
	}

	// Returns true if player is in RC Vehicle remote mode
	pub fn is_player_in_remote_mode(&self) -> bool {
		self.m_bInRemoteMode
	}

	// Disables the player control
	pub fn make_player_safe(flag : bool) {
		let addr = get_symbol_addr("?MakePlayerSafe@CPlayerInfo@@QEAAX_N@Z");
        unsafe {
            type MakePlayerSafe = extern "fastcall" fn(u64, bool);
            let func: MakePlayerSafe = std::mem::transmute(addr); 
			let pinfo = get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A");
            if read_mem::<u64>(pinfo, true) != 0 {
                func(pinfo, flag);
            }
        }
	}
}