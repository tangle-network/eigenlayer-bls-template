// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.13;

import "eigenlayer-contracts/src/contracts/libraries/BytesLib.sol";
import "contracts/src/IHelloTaskManager.sol";
//import "@eigenlayer-middleware/src/ServiceManagerBase.sol";
import "eigenlayer-middleware/src/ServiceManagerBase.sol";

/**
 * @title Primary entrypoint for procuring services from Hello.
 * @author Layr Labs, Inc.
 */
contract HelloServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IHelloTaskManager
        public immutable helloTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyHelloTaskManager() {
        require(
            msg.sender == address(helloTaskManager),
            "onlyHelloTaskManager: not from credible squaring task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        IRewardsCoordinator _rewardsCoordinator,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        IHelloTaskManager _helloTaskManager
    )
        ServiceManagerBase(
            _avsDirectory,
            _rewardsCoordinator,
            _registryCoordinator,
            _stakeRegistry
        )
    {
        helloTaskManager = _helloTaskManager;
    }

    /// @notice Called in the event of challenge resolution, in order to forward a call to the Slasher, which 'freezes' the `operator`.
    /// @dev The Slasher contract is under active development and its interface expected to change.
    ///      We recommend writing slashing logic without integrating with the Slasher at this point in time.
    function freezeOperator(
        address operatorAddr
    ) external onlyHelloTaskManager {
        // slasher.freezeOperator(operatorAddr);
    }
}
