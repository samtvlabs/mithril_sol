// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Test, console2} from "forge-std/Test.sol";
import {Verify} from "../src/Verify.sol";
import "./Msig.sol";

contract VerifyTest is Test {
    Verify public verify;
MsigData public msigData;
    uint256 public constant DECIMALS = 10 ** 18;

    uint256 m = 357;
    uint256 k = 2642;
    // hack  this is 0.2 , and is handled in the revm code.
    uint256 phi_f = 20;
    // todo fetch from rust code via ffis
    bytes ms = abi.encodePacked([0, 1, 2, 3, 4, 5]);

    function setUp() public {
        verify = new Verify();
        msigData = new MsigData();
    }

    // function test_VerificationPasses() public {
    //     bytes memory msig = msigData.msig();
    //     // let result = verify.verify_stm(m, k, phi_f, ms, msig);
    //     assertEq(verify.verify_stm(m, k, phi_f, ms, msig), true);
    
    //     // TODO: Adds assertions
    // }

    function test_BadSignaturesFails() public {
          bytes memory msig = abi.encodePacked([0, 1, 2, 3, 4, 5]);
          assertEq(verify.verify_stm(m, k, phi_f, ms, msig), true);
    }
}
