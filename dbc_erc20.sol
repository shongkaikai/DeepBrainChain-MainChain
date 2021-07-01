// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.2.0/contracts/token/ERC20/ERC20.sol";

contract DBCToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("Deepbrainchain Token", "DBC") public {
        _mint(msg.sender, initialSupply);
        root = msg.sender;
    }

    address root;
    function set_root(address new_root) public {
        require(msg.sender == root, "DBCERC20: only root");
        root = new_root;
    }

    function decimals() public view virtual override returns (uint8) {
        return 15;
    }

    function mint(address account, uint256 amount) public {
        require(account == root, "DBCERC20: only root");
        _mint(account, amount);
    }

    function burn(address account, uint256 amount) public {
        require(account == root, "DBCERC20: only root");
        _burn(account, amount);
    }
}
