use fuzz_instructions::lending_pool_fuzz_instructions::Deposit;
use fuzz_instructions::lending_pool_fuzz_instructions::Initialize;
use fuzz_instructions::lending_pool_fuzz_instructions::RegisterDepositor;
use fuzz_instructions::lending_pool_fuzz_instructions::UnregisterDepositor;
use lending_pool::entry as entry_lending_pool;
use lending_pool::ID as PROGRAM_ID_LENDING_POOL;
const PROGRAM_NAME_LENDING_POOL: &str = "lending_pool";
use fuzz_instructions::lending_pool_fuzz_instructions::FuzzInstruction as FuzzInstruction_lending_pool;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_lending_pool;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let init_ix = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        let register_depositor =
            FuzzInstruction::RegisterDepositor(RegisterDepositor::arbitrary(u)?);
        Ok(vec![init_ix, register_depositor])
    }
    fn ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let deposit_ix = FuzzInstruction::Deposit(Deposit::arbitrary(u)?);
        Ok(vec![deposit_ix])
    }
    fn post_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let unregister_depositor =
            FuzzInstruction::UnregisterDepositor(UnregisterDepositor::arbitrary(u)?);
        Ok(vec![unregister_depositor])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(PROGRAM_NAME_LENDING_POOL, &PROGRAM_ID_LENDING_POOL,processor!(convert_entry!(entry_lending_pool)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_LENDING_POOL, &mut client);
        });
    }
}
