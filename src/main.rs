use miden::{Assembler, Program, ProgramInputs, ProofOptions, HashFunction, FieldExtension};

#[derive(serde::Deserialize, serde::Serialize)]

pub struct InputFile {
    pub stack_init: Vec<String>,
}

fn get_program_inputs(stack_init: &[u64]) -> ProgramInputs {
    ProgramInputs::from_stack_inputs(&stack_init).unwrap()
}

/// Parse stack_init vector of strings to a vector of u64
fn stack_init(inputs_data: InputFile) -> Vec<u64> {
    inputs_data.stack_init
        .iter()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let masm_program_frontend = String::from("    
proc.storecellsn.16
    loc_store.0 drop
    loc_store.1 drop
    loc_store.2 drop
    loc_store.3 drop
    loc_store.4 drop
    loc_store.5 drop
    loc_store.6 drop
    loc_store.7 drop
    loc_store.8 drop
    loc_store.9 drop
    loc_store.10 drop
    loc_store.11 drop
    loc_store.12 drop
    loc_store.13 drop
    loc_store.14 drop
    loc_store.15 drop
 end

# We clear the stack by setting all to 0

proc.clearstack
    repeat.16
        drop
    end
end

proc.cell_0_transition.32
   
   # We assume a cell is dead (0) in the next round until proven otherwise
    push.0
    loc_store.16 drop
    
   # We load all the cell's neighbours to check their status
    loc_load.1
    loc_load.4
    loc_load.5

   # We load the cell itself to check its status
    loc_load.0

   # Cell is alice (1) and might stay alive 
    if.true
        dup
    else
        dup
    end
end

begin
    repeat.2
       exec.storecellsn
       exec.cell_0_transition
   end
end");
    let inputs_string_frontend = String::from(r#"
{
        "stack_init": ["0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "1", "1", "0", "0", "1", "0"]
}
"#);
    
    let assembler = Assembler::new();
                    
    let program: Program = assembler.compile(masm_program_frontend.as_str()).expect("Could not compile source");

    let inputs_str: InputFile = serde_json::from_str(&inputs_string_frontend.as_str())
        .map_err(|err| format!("Failed to deserialize input data - {}", err)).unwrap();
    let input_data = stack_init(inputs_str);
    let input = get_program_inputs(&input_data); 
    
    let proof_options = ProofOptions::new(
        27,
        8,
        16,
        HashFunction::Blake3_192,
        FieldExtension::Quadratic,
        8,
        256,
    );

    let (outputs, proof) = miden::prove(&program, &input, &proof_options).unwrap();
}
