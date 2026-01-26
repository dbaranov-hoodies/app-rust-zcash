# pylint: disable=C0301

from application_client.zcash_command_sender import ZcashCommandSender
from application_client.zcash_response_unpacker import unpack_trusted_input_response

TRUSTED_INPUT_RESPONSE_HEX_LEN = 112

def test_trusted_input_transparent_v5_two_inputs(backend):
    TX_BYTES = bytes.fromhex("050000800a27a726b4d0d6c200000000a8841e00021111111111111111111111111111111111111111111111111111111111111111000000006b483045022100e35dd2be5e5aeccce0ff7ff892db278047685bc11d34692fd72a9c1914d05f8e0220426dd0a98b39eb6051df9706e4ff9fba4a8be5cd6ef5c3fdd6f2200c709b2bad01210228d06186c26df6afa96076b0ac64cf0d8caf212937f328a52894183cc36e5dd8ffffffff2222222222222222222222222222222222222222222222222222222222222222010000006b483045022100abb1831a7c59bd893420bfe51df0627f239ac2c1524de86958fe84f122c5344d022046ef451e009e500c12516f082a03ffafd3743f522790b866af88ef202fc83a1d0121037e0c5efb047f692c0c89ea9a817f577dc086303aed2f662df4879c89448287c7ffffffff01a0860100000000001976a914b1630abe4ac3749ca5b0ea4c30a7eae5abab19be88ac000000")

    trusted_input_idx = 0

    client = ZcashCommandSender(backend)

    with client.get_trusted_input(TX_BYTES, trusted_input_idx):
        pass

    resp = client.get_async_response().data
    txid, idx, amount, _, _ = unpack_trusted_input_response(resp)

    assert txid.hex() == "754d1a6d0c8e7bfaff9bb1d2f356db3475e60e27d27376f64ba0f21c23adbd80"
    assert idx == trusted_input_idx
    assert amount == 100_000

def test_trusted_input_transparent_v5_two_outputs(backend):
    TX_BYTES = bytes.fromhex("050000800a27a726b4d0d6c200000000a8841e00011111111111111111111111111111111111111111111111111111111111111111000000006a47304402207822747dfbbb31fda5ec92ec908bed2fd9b347d14c5756cf7f81f6548c0eb9170220576933ee4c037cf2c4116a5e4bd374b3b5b518ace808ce740637e1c460ac7cc10121031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078fffffffff0250973100000000001976a914417d4be90d35363267b8f2afafc9531111c41ae488ac50973100000000001976a914417d4be90d35363267b8f2afafc9531111c41ae488ac000000");

    trusted_input_idx = 1

    client = ZcashCommandSender(backend)

    with client.get_trusted_input(TX_BYTES, trusted_input_idx):
        pass

    resp = client.get_async_response().data
    txid, idx, amount, _, _ = unpack_trusted_input_response(resp)

    assert txid.hex() == "f5f79290d3dfe4e768aec837affe8eb9e46fbc82ef9dfdf2c62af1ad0b3878a3"
    assert idx == trusted_input_idx
    assert amount == 3_250_000

def test_trusted_input_transparent_v5_old_1(backend):
    EXPECTED_TRUSTED_INPUT = "a9a27d42321c7ace2884a65a343abb9755f3eba881e53834bdb4a3fed4432a1301000000"

    transport = ZcashCommandSender(backend)

    # with transparent apdus
    sw, _ = transport.exchange_raw("e04200001100000001050000800a27a7265510e7c801")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800025e1360c957489515ddfb5c564962e2c8cb2dc3c651c4a219e25e0b5e569f49d33000000006b")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000324830450221008844cfb8d9983226f74cdd20cb63ee282360374def5de88d093df7f340775d65022072673cea8cd2092484c1")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000321c6e8c35ab765a9501024a96265bdd3b80d0c46f9190012102495e50ff5127b9b74083bad438208c7a39ddd83301cd04e40b")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280000bff5556d3351ab300000000")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280000102")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800022a0860100000000001976a914a96e684ec46cd8a2f98d6ef4b847c0ee88395e9388ac")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800022cedb0e00000000001976a9142495eecd3d7ea979d2066da533f45956a3a6b5c888ac")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800003000000")
    assert sw == 0x9000
    sw, resp = transport.exchange_raw("e042800009000000000400000000")
    assert sw == 0x9000

    resp = resp.hex()
    assert len(resp) == TRUSTED_INPUT_RESPONSE_HEX_LEN
    assert resp[8:8+32*2+8] == EXPECTED_TRUSTED_INPUT

def test_trusted_input_transparent_v5_old_2(backend):
    EXPECTED_TRUSTED_INPUT = "58854aa4e2e3b82aa2040c0bc3a6dc9b8ac6acb5e15bf0cfeacd09e77249c18a00000000"

    transport = ZcashCommandSender(backend)

    sw, _ = transport.exchange_raw("e04200001100000000050000800a27a726b4d0d6c201")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280002598cd6cd9559cd98109ad0622f899bc38805f11648e4f985ebe344b8238f87b13010000006b")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280003248304502210095104ae9d53a95105be4ba5a31caddff2ae83ced24b21ab4aec6d735d568fad102206e054b158047529bb736")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800032c810902ea7fc8d92f3f604c1b2a8bb0b92f0e6c016a8012102010a560c7325827df0212bca20f5cf6556b1345991b6b64b46")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280000b9c616e758230a5ffffffff")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280000102")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000221595dd04000000001976a914ca3ba17907dde979bf4e88f5c1be0ddf0847b25d88ac")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800022a245117c140000001976a914c8b56e00740e62449a053c15bdd4809f720b5cb588ac")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800003000000")
    assert sw == 0x9000

    sw, resp = transport.exchange_raw("e0428000090000000004f9081a00")
    assert sw == 0x9000

    resp = resp.hex()
    assert len(resp) == TRUSTED_INPUT_RESPONSE_HEX_LEN
    assert resp[8:8+32*2+8] == EXPECTED_TRUSTED_INPUT

def test_trusted_input_sapling_single_old(backend):
    # TXID == 339df469e5b5e0259e214a1c653cdcb28c2c2e9664c5b5df5d518974950c36e1
    TXID_LEN = 112
    EXPECTED_TRUSTED_INPUT = "e1360c957489515ddfb5c564962e2c8cb2dc3c651c4a219e25e0b5e569f49d3300000000"

    transport = ZcashCommandSender(backend)

    # with sapling apdus
    sw, _ = transport.exchange_raw("e04200001100000000050000800a27a7265510e7c800")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280000101")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280002220d61300000000001976a914040ac822bd91e60f709174ac94bee1fb1aaadf2a88ac")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800003010200")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800028b810140000000000ec191fd9bc3a13624b396dcdef5d13559ced5bcc2058b98f55b2250b9b98b658")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000602b88b763bbed1f015f89aca3786acd2702a5ec69482404fc90cb4e6f597827a1884a2be02987a69f1461b3d479661407c8f31f5f26140c964c7e28315e429115c534b33f896e0320f630a4c4418b25db0c2ce8412a386f29be29a138d496a260")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800074bfd33cf9757e6da7374d12598bdfee4a821f915254a1fed7189a0acecf52aa4087d61a8797497ba66dffeec0d3451b01c16b4dcbcc27f6896cdf59f07d4d535fb3b9f3b7a013d2b69aee28b9d4bdee0eaab836b776e6419324d5690dc1050b0b10063bead8c37ea1ebf03e390be41f313d370e70")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800074ef91cb7052c730740ca3fdb5082ca9316fd7fbe4ce63ad66fc32bdfe7106bb34f6cf09d89739dff6205051189a6d1bd7aa4f7518e2cf1872272f9a90baa053c932622ebd195de4ce00c7c3fdbdde46c11fd616fee703cf770caa03e9f6ffa1f32b77e21fffee6fa03badeb99dadaafdfd20ad05a")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000809953f91cfa0923a220f031b58e536e503cc161a046d7eefe1194f7fe3d879417b3ad8ed4e04620ecf771cf503939eb441fd2586d2711a1d1228292e66bc09986d06e332b697153228efa383094ecb27eca5288771f6a721185ec59bfeb7695b66692de9207a638386304b2fef517c1002158b7a340230d79be282d56764e78c7")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000807a4c09bb70874e0888a9c4b54df76e92621ec8ce3981e3413e0ce4576caa99a2fbeee2cbaa9d4448fa22c27fa588455bdb6ea9cf2515c8edb475a6da460a5e0072150f5d71568bf875a6cc0be52944aaa360dd425722ef70ff6e785263b45d0ce6053ea2a48360ad66432b84550e1634d6eb40a99e8c44e229d9de18306325d6")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000804f1595c928df3bece71461c281ed948fb54ad4c059cd78e0bd50f39a38cbeeb68977582b90ec18cc799ae9c63fbb028d884d4c32067cd6e73809b42b473c1c5031de16e524fe242c2840d8cb2d554544525e4d2e48655ed17c5d8f98a5f0b30d72a4be22fe250ea5d4b28e51421ecfa3d6c146edafdc90c6950f8f86c1c5db73")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800080b53e8323a1c59347de9f6a420aab8a1ea3245f0eac5c420e8d2333c25846289515ccfe15288670d870c2de8ff6dc737b5bb15b6d8c59366d59a15b3361475f7de91fabf0972e323356ecd6ee3bbe8129eac8c8a55dbd4945ad6c02b774dd986503ea7dbd1ef4d0fe00e88efbf09cc2ce2754963e085076cbda51a43fe04ffbc5")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280008098c04020e8a852c105a2b82a543e8e1e3b24646e196fbf1d75fe4bd77a82990a4cf54935d02b33b2853504567b9f8b3c2d056c489d991b16543cfbaf6e69a42cc356b799705085b7b58347272a5ff0c454dc5ba733dc6127d573844e4f7461f57de2ec04eb97bd94dc29f9bd700cd4f7e0f0c5144e1e24c83528735e17ce585d")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000804ca2ca7318d44f5fe00b069cc27806c557c5dba69f3e8ab910ba3509f25ec05af5a0f0e7e86eaec4ec1264124c02d7c35bb90ef500cc813d1f1f37ead8eba59e3c7c346042e4bf29055a823c820bb47000d8e0aa07707f33f0ba4254538532066f0e59ac60a64547acf69ba15b4cd61bc6b7e7d4fa111fcbe7eed9cd125f5735")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000808c74d1a0db663487ea4f88ae079460c09199e93315defab6ace9df089954beb09dde24549b5ec3e6e0c6d95580ea1a25516016f87b52ecb902c9bc87f9386e668867f920a6c0da05ff9da0718fb6dff84d3f381f1c40da6e1bc75b6045a54296b759f99071e145d99b9772616280d04c278bff7c3688070fc7ac4ff6e0bdb329")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e0428000804224af07e1efe10cf57f6898a39b63ddd9792c4effd933414a1cbc23b20ccf1f3624a187b4faab8687ad58b18a4dc5753343ddff7477370f8e766d74e7c5db381910c768270cb62096b110dd9268ad97149fc13c5513024dfafb4093fd6d31c942d9bfb0630f98957d9782dac478f39c6d74affcdf031a56e63e090437d1670a")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e042800080d50845284903cd9e2f3da8b5d716217e82aa6021acf7e402a5918dd39602710ca57e7862ccbfa5b7ebf4f999c2450108589143fbcd257fade54537bd6b808f0f59a020033cc09166790f7eae8eb2c236bbcfa0515846939b10f8d675a0c39d5dadf8f8df6039d7c56965255de8b22f43bf40749c65ff1c6b9c939d394e165be0")
    assert sw == 0x9000
    sw, _ = transport.exchange_raw("e04280008066bc1edf06b465f42ffaabddc059f238e02879ade2a08041314665b38ab2e13bfc2a1e75cbd359667fd002f83c70aa35685314e8c9a8e00d3e653a9d90ccff47b02cd6621ee5ddd94e3e3a822c52bbbfce64b8a55bfe6d3da9bfb1b1f8948c3a7fdfee92bf369b43bb432954273d98f251d344aa710d812c8b84b8ab4f694080")
    assert sw == 0x9000
    sw, sig = transport.exchange_raw("e042800009000000000494e02c00")
    assert sw == 0x9000

    sig = sig.hex()
    assert len(sig) == TXID_LEN
    assert sig[8:8+32*2+8] == EXPECTED_TRUSTED_INPUT
