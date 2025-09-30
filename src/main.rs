use crate::{db::file_manager::FileManager, structs::{cidade::Cidade, consulta::Consulta, diaria::Diaria, especialidade::Especialidade, exame::Exame, medico::Medico, paciente::Paciente}, utils::{faturamentos, relatorios}};
mod structs;
mod menus;
mod db;
mod utils;

fn main() {
    let mut paciente_manager = FileManager::<Paciente>::new("pacientes.dat").unwrap();
    let mut medico_manager = FileManager::<Medico>::new("medicos.dat").unwrap();
    let mut cidade_manager = FileManager::<Cidade>::new("cidades.dat").unwrap();
    let mut especialidade_manager = FileManager::<Especialidade>::new("especialidades.dat").unwrap();
    let mut exame_manager = FileManager::<Exame>::new("exames.dat").unwrap();
    let mut consulta_manager = FileManager::<Consulta>::new("consultas.dat").unwrap();
    let mut diaria_manager = FileManager::<Diaria>::new("diarias.dat").unwrap();

    loop {
        menus::exibir_menu_principal(); 
        let choice = menus::ler_opcao_menu();

        match choice {
            1 => menus::menu_pacientes(&mut paciente_manager, &cidade_manager),
            2 => menus::menu_medicos(&mut medico_manager, &cidade_manager, &especialidade_manager),
            3 => menus::menu_especialidades(&mut especialidade_manager),
            4 => menus::menu_cidades(&mut cidade_manager),
            5 => menus::menu_exames(&mut exame_manager, &especialidade_manager),
            6 => menus::menu_consultas(
                &mut consulta_manager,
                &paciente_manager,
                &medico_manager,
                &cidade_manager,
                &especialidade_manager,
                &exame_manager,
                &mut diaria_manager,
            ),
            7 => menus::menu_diarias(&mut diaria_manager),
            8 => faturamentos::menu_faturamento(
                &consulta_manager,
                &medico_manager,
                &especialidade_manager,
                &exame_manager,
            ),
            9 => relatorios::relatorio_consultas_ordenadas(
                &consulta_manager,
                &paciente_manager,
                &cidade_manager,
                &medico_manager,
                &especialidade_manager,
                &exame_manager,
            ),
            10 => {
                println!("Até mais!");
                break;
            },
            _ => println!("Opção inválida. Por favor, tente novamente."),
        }
    }
}
