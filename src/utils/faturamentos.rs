use std::collections::HashMap;

use crate::{db::file_manager::FileManager, menus::{ler_opcao_menu, ler_string}, structs::{consulta::Consulta, especialidade::Especialidade, exame::Exame, medico::Medico}};


pub fn faturamento(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    loop {
        println!("\n--- Relatórios de Faturamento ---");
        println!("1. Faturamento por Dia");
        println!("2. Faturamento por Período");
        println!("3. Faturamento por Médico");
        println!("4. Faturamento por Especialidade");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => faturamento_por_dia(consulta_manager, medico_manager, especialidade_manager, exame_manager),
            2 => faturamento_por_periodo(consulta_manager, medico_manager, especialidade_manager, exame_manager),
            3 => faturamento_por_medico(consulta_manager, medico_manager, especialidade_manager, exame_manager),
            4 => faturamento_por_especialidade(consulta_manager, medico_manager, especialidade_manager, exame_manager),
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

fn faturamento_por_dia(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let mut faturamento_dia: HashMap<String, f32> = HashMap::new();
    let consultas = consulta_manager.read_all_records().unwrap();

    for consulta in consultas {
        let valor_total = calcular_valor_consulta_total(
            &consulta,
            medico_manager,
            especialidade_manager,
            exame_manager,
        );
        *faturamento_dia.entry(consulta.data).or_insert(0.0) += valor_total;
    }

    println!("\n--- Faturamento por Dia ---");
    for (dia, valor) in faturamento_dia {
        println!("Dia {}: R$ {:.2}", dia, valor);
    }
}

fn faturamento_por_periodo(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let data_inicial = ler_string("Digite a data inicial (AAAAMMDD): ");
    let data_final = ler_string("Digite a data final (AAAAMMDD): ");
    let mut valor_total_periodo = 0.0;
    let consultas = consulta_manager.read_all_records().unwrap();

    for consulta in consultas {
        if consulta.data >= data_inicial && consulta.data <= data_final {
            valor_total_periodo += calcular_valor_consulta_total(
                &consulta,
                medico_manager,
                especialidade_manager,
                exame_manager,
            );
        }
    }

    println!("\n--- Faturamento de {} a {} ---", data_inicial, data_final);
    println!("Faturamento total no período: R$ {:.2}", valor_total_periodo);
}

fn faturamento_por_medico(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let mut faturamento_medico: HashMap<u32, f32> = HashMap::new();
    let consultas = consulta_manager.read_all_records().unwrap();

    for consulta in consultas {
        let valor_total = calcular_valor_consulta_total(
            &consulta,
            medico_manager,
            especialidade_manager,
            exame_manager,
        );
        *faturamento_medico.entry(consulta.codigo_medico).or_insert(0.0) += valor_total;
    }

    println!("\n--- Faturamento por Médico ---");
    for (codigo_medico, valor) in faturamento_medico {
        let medico = medico_manager.read_record(codigo_medico).unwrap_or(None);
        let nome_medico = medico.map_or("Médico Desconhecido".to_string(), |m| m.nome);
        println!("{}: R$ {:.2}", nome_medico, valor);
    }
}

fn faturamento_por_especialidade(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let mut faturamento_especialidade: HashMap<u32, f32> = HashMap::new();
    let consultas = consulta_manager.read_all_records().unwrap();

    for consulta in consultas {
        let valor_total = calcular_valor_consulta_total(
            &consulta,
            medico_manager,
            especialidade_manager,
            exame_manager,
        );
        
        if let Ok(Some(medico)) = medico_manager.read_record(consulta.codigo_medico) {
            *faturamento_especialidade.entry(medico.codigo_especialidade).or_insert(0.0) += valor_total;
        }
    }

    println!("\n--- Faturamento por Especialidade ---");
    for (codigo_especialidade, valor) in faturamento_especialidade {
        let especialidade = especialidade_manager.read_record(codigo_especialidade).unwrap_or(None);
        let descricao = especialidade.map_or("Especialidade Desconhecida".to_string(), |e| e.descricao);
        println!("{}: R$ {:.2}", descricao, valor);
    }
}

fn calcular_valor_consulta_total(
    consulta: &Consulta,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) -> f32 {
    let valor_consulta = medico_manager
        .read_record(consulta.codigo_medico)
        .ok().flatten()
        .and_then(|m| especialidade_manager.read_record(m.codigo_especialidade).ok().flatten())
        .map_or(0.0, |e| e.valor_consulta);

    let valor_exame = exame_manager
        .read_record(consulta.codigo_exame)
        .ok().flatten()
        .map_or(0.0, |e| e.valor_exame);

    valor_consulta + valor_exame
}