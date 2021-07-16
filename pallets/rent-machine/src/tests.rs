use crate::mock::*;
use dbc_price_ocw::MAX_LEN;
use frame_support::assert_ok;
use online_profile::{
    CommitteeUploadInfo, LiveMachine, StakerCustomizeInfo, StandardGpuPointPrice,
};
use std::convert::TryInto;

#[test]
fn should_works() {
    new_test_ext_after_machine_online().execute_with(|| {
        run_to_block(1);

        // 上线一台机器
        let _alice: sp_core::sr25519::Public = sr25519::Public::from(Sr25519Keyring::Alice).into();
        let _bob: sp_core::sr25519::Public = sr25519::Public::from(Sr25519Keyring::Bob).into();
        let _charile: sp_core::sr25519::Public =
            sr25519::Public::from(Sr25519Keyring::Charlie).into();
        let renter_dave: sp_core::sr25519::Public =
            sr25519::Public::from(Sr25519Keyring::Dave).into();

        let one_committee: sp_core::sr25519::Public =
            sr25519::Public::from(Sr25519Keyring::One).into();

        // Controller
        let controller: sp_core::sr25519::Public =
            sr25519::Public::from(Sr25519Keyring::Eve).into();
        // Stash
        let stash: sp_core::sr25519::Public = sr25519::Public::from(Sr25519Keyring::Ferdie).into();
        // Bob pubkey
        let machine_id = "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";
        let machine_id = machine_id.as_bytes().to_vec();

        // dave租用了10天
        assert_ok!(RentMachine::rent_machine(Origin::signed(renter_dave), machine_id.clone(), 10));

        run_to_block(50);

        // dave确认租用成功
        assert_ok!(RentMachine::confirm_rent(Origin::signed(renter_dave), machine_id.clone()));

        // dave续租成功
        assert_ok!(RentMachine::relet_machine(Origin::signed(renter_dave), machine_id.clone(), 10));

        // TODO: 检查机器得分

        // TODO: 检查租金是否正确扣除

        // TODO: 检查机器退租后，状态是否清理

        // TODO: 检查机器没有租用成功，押金正常退回
    })
}
