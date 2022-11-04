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
begin
    push.1
    if.true
        dup
    else
        dup
    end
end");
    let inputs_string_frontend = String::from(r#"
{
        "stack_init": []
}
"#);
    
    let assembler = Assembler::new();
                    
    let program: Program = assembler.compile(masm_program_frontend.as_str()).expect("Could not compile source");

    let inputs_str: InputFile = serde_json::from_str(&inputs_string_frontend.as_str())
        .map_err(|err| format!("Failed to deserialize input data - {}", err)).unwrap();
    let input_data = stack_init(inputs_str);
    println!("{:?}", input_data);
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
