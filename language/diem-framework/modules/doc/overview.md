
<a name="@Diem_Framework_Modules_0"></a>

# Diem Framework Modules


This is the root document for the Diem framework module documentation. The Diem framework provides a set of Move
modules which define the resources and functions available for the Diem blockchain. Each module is individually
documented here, together with its implementation and
[formal specification](../../script_documentation/spec_documentation.md).

Move modules are not directly called by clients, but instead are used to implement *transaction scripts*.
For documentation of transaction scripts which constitute the client API, see
[../../script_documentation/script_documentation.md](../../script_documentation/script_documentation.md).

The Move modules in the Diem Framework can be bucketed in to a couple categories:


<a name="@Treasury_and_Compliance_1"></a>

### Treasury and Compliance

* <code><a href="AccountFreezing.md#0x1_AccountFreezing">AccountFreezing</a></code>
* <code><a href="AccountLimits.md#0x1_AccountLimits">AccountLimits</a></code>
* <code><a href="DesignatedDealer.md#0x1_DesignatedDealer">DesignatedDealer</a></code>
* <code><a href="DualAttestation.md#0x1_DualAttestation">DualAttestation</a></code>

* <code><a href="XUS.md#0x1_XUS">XUS</a></code>
* <code><a href="XDX.md#0x1_XDX">XDX</a></code>
* <code><a href="Diem.md#0x1_Diem">Diem</a></code>
* <code><a href="RegisteredCurrencies.md#0x1_RegisteredCurrencies">RegisteredCurrencies</a></code>


<a name="@Authentication_2"></a>

### Authentication

* <code><a href="Authenticator.md#0x1_Authenticator">Authenticator</a></code>
* <code><a href="RecoveryAddress.md#0x1_RecoveryAddress">RecoveryAddress</a></code>
* <code><a href="SharedEd25519PublicKey.md#0x1_SharedEd25519PublicKey">SharedEd25519PublicKey</a></code>
* <code><a href="Signature.md#0x1_Signature">Signature</a></code>


<a name="@Accounts_and_Access_Control_3"></a>

### Accounts and Access Control

* <code><a href="DiemAccount.md#0x1_DiemAccount">DiemAccount</a></code>
* <code><a href="Roles.md#0x1_Roles">Roles</a></code>
* <code><a href="VASP.md#0x1_VASP">VASP</a></code>


<a name="@System_Management_4"></a>

### System Management

* <code><a href="ChainId.md#0x1_ChainId">ChainId</a></code>
* <code><a href="DiemBlock.md#0x1_DiemBlock">DiemBlock</a></code>
* <code><a href="DiemConfig.md#0x1_DiemConfig">DiemConfig</a></code>
* <code><a href="DiemTimestamp.md#0x1_DiemTimestamp">DiemTimestamp</a></code>
* <code><a href="DiemTransactionPublishingOption.md#0x1_DiemTransactionPublishingOption">DiemTransactionPublishingOption</a></code>
* <code><a href="DiemVersion.md#0x1_DiemVersion">DiemVersion</a></code>
* <code><a href="DiemVMConfig.md#0x1_DiemVMConfig">DiemVMConfig</a></code>
* <code><a href="TransactionFee.md#0x1_TransactionFee">TransactionFee</a></code>
* <code><a href="DiemSystem.md#0x1_DiemSystem">DiemSystem</a></code>
* <code><a href="ValidatorConfig.md#0x1_ValidatorConfig">ValidatorConfig</a></code>
* <code><a href="ValidatorOperatorConfig.md#0x1_ValidatorOperatorConfig">ValidatorOperatorConfig</a></code>
* <code><a href="Genesis.md#0x1_Genesis">Genesis</a></code> (Note: not published on-chain)


<a name="@Module_Utility_Libraries_5"></a>

### Module Utility Libraries

* <code><a href="../../../../../../move-stdlib/docs/Errors.md#0x1_Errors">Errors</a></code>
* <code><a href="CoreAddresses.md#0x1_CoreAddresses">CoreAddresses</a></code>
* <code><a href="../../../../../../move-stdlib/docs/Event.md#0x1_Event">Event</a></code>
* <code><a href="../../../../../../move-stdlib/docs/FixedPoint32.md#0x1_FixedPoint32">FixedPoint32</a></code>
* <code><a href="../../../../../../move-stdlib/docs/Hash.md#0x1_Hash">Hash</a></code>
* <code><a href="../../../../../../move-stdlib/docs/BCS.md#0x1_BCS">BCS</a></code>
* <code><a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option">Option</a></code>
* <code><a href="SlidingNonce.md#0x1_SlidingNonce">SlidingNonce</a></code>
* <code><a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector">Vector</a></code>
* <code><a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer">Signer</a></code>


<a name="@Index_6"></a>

## Index


-  [`0x1::AccountAdministrationScripts`](AccountAdministrationScripts.md#0x1_AccountAdministrationScripts)
-  [`0x1::AccountCreationScripts`](AccountCreationScripts.md#0x1_AccountCreationScripts)
-  [`0x1::AccountFreezing`](AccountFreezing.md#0x1_AccountFreezing)
-  [`0x1::AccountLimits`](AccountLimits.md#0x1_AccountLimits)
-  [`0x1::AccountScripts`](ol_account.md#0x1_AccountScripts)
-  [`0x1::Ancestry`](Ancestry.md#0x1_Ancestry)
-  [`0x1::Audit`](Audit.md#0x1_Audit)
-  [`0x1::Authenticator`](Authenticator.md#0x1_Authenticator)
-  [`0x1::AutoPay`](AutoPay.md#0x1_AutoPay)
-  [`0x1::AutoPayScripts`](ol_autopay.md#0x1_AutoPayScripts)
-  [`0x1::BCS`](../../../../../../move-stdlib/docs/BCS.md#0x1_BCS)
-  [`0x1::Bonding`](Demo_Bonding.md#0x1_Bonding)
-  [`0x1::Burn`](Burn.md#0x1_Burn)
-  [`0x1::BurnScript`](ol_burn_pref.md#0x1_BurnScript)
-  [`0x1::Cases`](Cases.md#0x1_Cases)
-  [`0x1::ChainId`](ChainId.md#0x1_ChainId)
-  [`0x1::CoreAddresses`](CoreAddresses.md#0x1_CoreAddresses)
-  [`0x1::Debug`](Debug.md#0x1_Debug)
-  [`0x1::Decimal`](Decimal.md#0x1_Decimal)
-  [`0x1::DemoScripts`](ol_demo_e2e.md#0x1_DemoScripts)
-  [`0x1::DesignatedDealer`](DesignatedDealer.md#0x1_DesignatedDealer)
-  [`0x1::Diem`](Diem.md#0x1_Diem)
-  [`0x1::DiemAccount`](DiemAccount.md#0x1_DiemAccount)
-  [`0x1::DiemBlock`](DiemBlock.md#0x1_DiemBlock)
-  [`0x1::DiemConfig`](DiemConfig.md#0x1_DiemConfig)
-  [`0x1::DiemConsensusConfig`](DiemConsensusConfig.md#0x1_DiemConsensusConfig)
-  [`0x1::DiemId`](DiemId.md#0x1_DiemId)
-  [`0x1::DiemSystem`](DiemSystem.md#0x1_DiemSystem)
-  [`0x1::DiemTimestamp`](DiemTimestamp.md#0x1_DiemTimestamp)
-  [`0x1::DiemTransactionPublishingOption`](DiemTransactionPublishingOption.md#0x1_DiemTransactionPublishingOption)
-  [`0x1::DiemVMConfig`](DiemVMConfig.md#0x1_DiemVMConfig)
-  [`0x1::DiemVersion`](DiemVersion.md#0x1_DiemVersion)
-  [`0x1::DualAttestation`](DualAttestation.md#0x1_DualAttestation)
-  [`0x1::Epoch`](Epoch.md#0x1_Epoch)
-  [`0x1::EpochBoundary`](EpochBoundary.md#0x1_EpochBoundary)
-  [`0x1::Errors`](../../../../../../move-stdlib/docs/Errors.md#0x1_Errors)
-  [`0x1::EthSignature`](EthSignature.md#0x1_EthSignature)
-  [`0x1::Event`](../../../../../../move-stdlib/docs/Event.md#0x1_Event)
-  [`0x1::FIFO`](FIFO.md#0x1_FIFO)
-  [`0x1::FixedPoint32`](../../../../../../move-stdlib/docs/FixedPoint32.md#0x1_FixedPoint32)
-  [`0x1::FullnodeSubsidy`](FullnodeSubsidy.md#0x1_FullnodeSubsidy)
-  [`0x1::GAS`](GAS.md#0x1_GAS)
-  [`0x1::Genesis`](Genesis.md#0x1_Genesis)
-  [`0x1::Globals`](Globals.md#0x1_Globals)
-  [`0x1::Hash`](../../../../../../move-stdlib/docs/Hash.md#0x1_Hash)
-  [`0x1::MakeWhole`](MakeWhole.md#0x1_MakeWhole)
-  [`0x1::MigrateAutoPayBal`](Migrations.md#0x1_MigrateAutoPayBal)
-  [`0x1::MigrateTowerCounter`](Migrations.md#0x1_MigrateTowerCounter)
-  [`0x1::MigrateVouch`](Migrations.md#0x1_MigrateVouch)
-  [`0x1::Migrations`](Migrations.md#0x1_Migrations)
-  [`0x1::Mock`](Mock.md#0x1_Mock)
-  [`0x1::NodeWeight`](NodeWeight.md#0x1_NodeWeight)
-  [`0x1::Option`](../../../../../../move-stdlib/docs/Option.md#0x1_Option)
-  [`0x1::Oracle`](Oracle.md#0x1_Oracle)
-  [`0x1::OracleScripts`](ol_oracle.md#0x1_OracleScripts)
-  [`0x1::PaymentScripts`](PaymentScripts.md#0x1_PaymentScripts)
-  [`0x1::PersistenceDemo`](Demos.md#0x1_PersistenceDemo)
-  [`0x1::Receipts`](Receipts.md#0x1_Receipts)
-  [`0x1::RecoveryAddress`](RecoveryAddress.md#0x1_RecoveryAddress)
-  [`0x1::RecoveryMode`](RecoveryMode.md#0x1_RecoveryMode)
-  [`0x1::RegisteredCurrencies`](RegisteredCurrencies.md#0x1_RegisteredCurrencies)
-  [`0x1::Roles`](Roles.md#0x1_Roles)
-  [`0x1::SharedEd25519PublicKey`](SharedEd25519PublicKey.md#0x1_SharedEd25519PublicKey)
-  [`0x1::Signature`](Signature.md#0x1_Signature)
-  [`0x1::Signer`](../../../../../../move-stdlib/docs/Signer.md#0x1_Signer)
-  [`0x1::SlidingNonce`](SlidingNonce.md#0x1_SlidingNonce)
-  [`0x1::StagingNet`](Testnet.md#0x1_StagingNet)
-  [`0x1::Stats`](Stats.md#0x1_Stats)
-  [`0x1::Subsidy`](Subsidy.md#0x1_Subsidy)
-  [`0x1::SystemAdministrationScripts`](SystemAdministrationScripts.md#0x1_SystemAdministrationScripts)
-  [`0x1::TestFixtures`](TestFixtures.md#0x1_TestFixtures)
-  [`0x1::Testnet`](Testnet.md#0x1_Testnet)
-  [`0x1::TowerState`](TowerState.md#0x1_TowerState)
-  [`0x1::TowerStateScripts`](ol_miner_state.md#0x1_TowerStateScripts)
-  [`0x1::TransactionFee`](TransactionFee.md#0x1_TransactionFee)
-  [`0x1::TransferScripts`](ol_transfer.md#0x1_TransferScripts)
-  [`0x1::TreasuryComplianceScripts`](TreasuryComplianceScripts.md#0x1_TreasuryComplianceScripts)
-  [`0x1::Upgrade`](Upgrade.md#0x1_Upgrade)
-  [`0x1::VASP`](VASP.md#0x1_VASP)
-  [`0x1::VDF`](VDF.md#0x1_VDF)
-  [`0x1::ValidatorAdministrationScripts`](ValidatorAdministrationScripts.md#0x1_ValidatorAdministrationScripts)
-  [`0x1::ValidatorConfig`](ValidatorConfig.md#0x1_ValidatorConfig)
-  [`0x1::ValidatorOperatorConfig`](ValidatorOperatorConfig.md#0x1_ValidatorOperatorConfig)
-  [`0x1::ValidatorScripts`](ol_validator.md#0x1_ValidatorScripts)
-  [`0x1::ValidatorUniverse`](ValidatorUniverse.md#0x1_ValidatorUniverse)
-  [`0x1::Vector`](../../../../../../move-stdlib/docs/Vector.md#0x1_Vector)
-  [`0x1::Vouch`](Vouch.md#0x1_Vouch)
-  [`0x1::VouchScripts`](ol_vouch_for.md#0x1_VouchScripts)
-  [`0x1::Wallet`](Wallet.md#0x1_Wallet)
-  [`0x1::WalletScripts`](ol_wallet.md#0x1_WalletScripts)
-  [`0x1::XDX`](XDX.md#0x1_XDX)
-  [`0x1::XHash`](xhash.md#0x1_XHash)
-  [`0x1::XUS`](XUS.md#0x1_XUS)


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/diem/dip/blob/main/dips/dip-2.md
[ROLE]: https://github.com/diem/dip/blob/main/dips/dip-2.md#roles
[PERMISSION]: https://github.com/diem/dip/blob/main/dips/dip-2.md#permissions
