// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.2.0/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";
// import "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";

contract DBCERC20 is ERC20PresetMinterPauser {
    bytes32 public constant BURN_ROLE = keccak256("BURN_ROLE");

    constructor() ERC20PresetMinterPauser("DeepBrainChain Token", "DBC") {
        _setupRole(BURN_ROLE, _msgSender());
    }

    function decimals() public view virtual override returns (uint8) {
        return 15;
    }

    function sudo_burn(address account, uint256 amount) public {
        require(hasRole(BURN_ROLE, _msgSender()), "DBCERC20: must have burn role to burn");
        _burn(account, amount);
    }
}
