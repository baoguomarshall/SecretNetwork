package compute

import (
	"encoding/json"
	compute "github.com/enigmampc/SecretNetwork/x/compute/internal/keeper"
	"testing"

	"github.com/stretchr/testify/require"

	sdk "github.com/enigmampc/cosmos-sdk/types"
)

type contractState struct {
}

//func prepareInitSignedTx(t *testing.T, keeper keeper.Keeper, ctx sdk.Context, creator sdk.AccAddress, privKey crypto.PrivKey, encMsg []byte, codeID uint64, funds sdk.Coins) sdk.Context {
//	creatorAcc, err := auth.GetSignerAcc(ctx, keeper.accountKeeper, creator)
//	require.NoError(t, err)
//
//	tx := authtypes.NewTestTx(ctx, []sdk.Msg{types.MsgInstantiateContract{
//		Sender:    creator,
//		Admin:     nil,
//		Code:      codeID,
//		Label:     "demo contract 1",
//		InitMsg:   encMsg,
//		InitFunds: funds,
//	}}, []crypto.PrivKey{privKey}, []uint64{creatorAcc.GetAccountNumber()}, []uint64{creatorAcc.GetSequence() - 1}, authtypes.StdFee{
//		Amount: nil,
//		Gas:    0,
//	})
//
//	txBytes, err := keeper.cdc.MarshalBinaryLengthPrefixed(tx)
//	require.NoError(t, err)
//
//	return ctx.WithTxBytes(txBytes)
//}

func TestInitGenesis(t *testing.T) {
	data, cleanup := setupTest(t)
	defer cleanup()

	deposit := sdk.NewCoins(sdk.NewInt64Coin("denom", 100000))
	topUp := sdk.NewCoins(sdk.NewInt64Coin("denom", 5000))
	creator, privCreator := createFakeFundedAccount(data.ctx, data.acctKeeper, deposit.Add(deposit...))
	fred, _ := createFakeFundedAccount(data.ctx, data.acctKeeper, topUp)

	h := data.module.NewHandler()
	q := data.module.NewQuerierHandler()

	t.Log("fail with invalid source url")
	msg := MsgStoreCode{
		Sender:       creator,
		WASMByteCode: testContract,
		Source:       "someinvalidurl",
		Builder:      "",
	}

	err := msg.ValidateBasic()
	require.Error(t, err)

	_, err = h(data.ctx, msg)
	require.Error(t, err)

	t.Log("fail with relative source url")
	msg = MsgStoreCode{
		Sender:       creator,
		WASMByteCode: testContract,
		Source:       "./testdata/escrow.wasm",
		Builder:      "",
	}

	err = msg.ValidateBasic()
	require.Error(t, err)

	_, err = h(data.ctx, msg)
	require.Error(t, err)

	t.Log("fail with invalid build tag")
	msg = MsgStoreCode{
		Sender:       creator,
		WASMByteCode: testContract,
		Source:       "",
		Builder:      "somerandombuildtag-0.6.2",
	}

	err = msg.ValidateBasic()
	require.Error(t, err)

	_, err = h(data.ctx, msg)
	require.Error(t, err)

	t.Log("no error with valid source and build tag")
	msg = MsgStoreCode{
		Sender:       creator,
		WASMByteCode: testContract,
		Source:       "https://github.com/enigmampc/SecretNetwork/blob/cosnwasm/x/compute/testdata/escrow.wasm",
		Builder:      "confio/cosmwasm-opt:0.7.0",
	}
	err = msg.ValidateBasic()
	require.NoError(t, err)

	res, err := h(data.ctx, msg)
	require.NoError(t, err)
	require.Equal(t, res.Data, []byte("1"))

	_, _, bob := keyPubAddr()
	initMsg := initMsg{
		Verifier:    fred,
		Beneficiary: bob,
	}
	initMsgBz, err := json.Marshal(initMsg)
	require.NoError(t, err)

	initCmd := MsgInstantiateContract{
		Sender:    creator,
		CodeID:    1,
		InitMsg:   initMsgBz,
		InitFunds: deposit,
	}

	//compute.PrepareInitSignedTx()
	data.ctx = compute.PrepareInitSignedTx(t, data.keeper, data.ctx, creator, privCreator, initMsgBz, 1, deposit)

	res, err = h(data.ctx, initCmd)
	require.NoError(t, err)
	contractAddr := sdk.AccAddress(res.Data)

	execCmd := MsgExecuteContract{
		Sender:    fred,
		Contract:  contractAddr,
		Msg:       []byte(`{"release":{}}`),
		SentFunds: topUp,
	}
	res, err = h(data.ctx, execCmd)
	require.NoError(t, err)

	// ensure all contract state is as after init
	assertCodeList(t, q, data.ctx, 1)
	assertCodeBytes(t, q, data.ctx, 1, testContract)

	assertContractList(t, q, data.ctx, 1, []string{contractAddr.String()})
	assertContractInfo(t, q, data.ctx, contractAddr, 1, creator)
	// assertContractState(t, q, data.ctx, contractAddr, state{
	// 	Verifier:    []byte(fred),
	// 	Beneficiary: []byte(bob),
	// 	Funder:      []byte(creator),
	// })

	// export into genstate
	genState := ExportGenesis(data.ctx, data.keeper)

	// create new app to import genstate into
	newData, newCleanup := setupTest(t)
	defer newCleanup()
	q2 := newData.module.NewQuerierHandler()

	// initialize new app with genstate
	InitGenesis(newData.ctx, newData.keeper, genState)

	// run same checks again on newdata, to make sure it was reinitialized correctly
	assertCodeList(t, q2, newData.ctx, 1)
	assertCodeBytes(t, q2, newData.ctx, 1, testContract)

	assertContractList(t, q2, newData.ctx, 1, []string{contractAddr.String()})
	assertContractInfo(t, q2, newData.ctx, contractAddr, 1, creator)
	// assertContractState(t, q2, newData.ctx, contractAddr, state{
	// 	Verifier:    []byte(fred),
	// 	Beneficiary: []byte(bob),
	// 	Funder:      []byte(creator),
	// })
}
