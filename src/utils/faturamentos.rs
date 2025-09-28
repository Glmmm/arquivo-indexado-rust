use std::collections::HashMap;

use crate::{db::file_manager::FileManager, menus::{ler_opcao_menu, ler_string}, structs::{ consulta::Consulta, especialidade::Especialidade, exame::Exame, medico::Medico}};


pub fn menu_faturamento(
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
        println!("5. Voltar");
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

pub fn faturamento_por_dia(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let dia = ler_string("Digite o dia (AAAAMMDD): ");
    let consultas = consulta_manager.read_all_records().unwrap();
    let consultas_do_dia = consultas.into_iter().filter(|c| c.data == dia).collect::<Vec<_>>();
    
    let mut faturamento_total = 0.0;
    println!("\nFaturamento do dia {}:", dia);
    for consulta in consultas_do_dia {
        let valor = calcular_valor_consulta_total(&consulta, medico_manager, especialidade_manager, exame_manager);
        println!("- Consulta {}: R$ {:.2}", consulta.codigo_consulta, valor);
        faturamento_total += valor;
    }
    println!("Faturamento total do dia: R$ {:.2}", faturamento_total);
}

pub fn faturamento_por_periodo(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let inicio_str = ler_string("Digite a data de início (AAAAMMDD): ");
    let fim_str = ler_string("Digite a data de fim (AAAAMMDD): ");
    
    let inicio = inicio_str.parse::<u32>().unwrap_or(0);
    let fim = fim_str.parse::<u32>().unwrap_or(0);

    let consultas = consulta_manager.read_all_records().unwrap();
    let consultas_do_periodo = consultas.into_iter()
        .filter(|c| {
            let data_consulta = c.data.parse::<u32>().unwrap_or(0);
            data_consulta >= inicio && data_consulta <= fim
        })
        .collect::<Vec<_>>();

    let mut faturamento_total = 0.0;
    println!("\nFaturamento do período de {} a {}:", inicio_str, fim_str);
    for consulta in consultas_do_periodo {
        let valor = calcular_valor_consulta_total(&consulta, medico_manager, especialidade_manager, exame_manager);
        println!("- Consulta {}: R$ {:.2}", consulta.codigo_consulta, valor);
        faturamento_total += valor;
    }
    println!("Faturamento total do período: R$ {:.2}", faturamento_total);
}

pub fn faturamento_por_medico(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let mut faturamento_por_medico = HashMap::new();
    let consultas = consulta_manager.read_all_records().unwrap();
    let medicos = medico_manager.read_all_records().unwrap();
    
    for consulta in consultas {
        let valor = calcular_valor_consulta_total(&consulta, medico_manager, especialidade_manager, exame_manager);
        let medico = medicos.iter().find(|m| m.codigo_medico == consulta.codigo_medico);
        if let Some(medico) = medico {
            *faturamento_por_medico.entry(&medico.nome).or_insert(0.0) += valor;
        }
    }
    
    println!("\n--- Faturamento por Médico ---");
    for (nome_medico, faturamento) in faturamento_por_medico {
        println!("{}: R$ {:.2}", nome_medico, faturamento);
    }
}

pub fn faturamento_por_especialidade(
    consulta_manager: &FileManager<Consulta>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    let mut faturamento_por_especialidade = HashMap::new();
    let consultas = consulta_manager.read_all_records().unwrap();
    let medicos = medico_manager.read_all_records().unwrap();
    let especialidades = especialidade_manager.read_all_records().unwrap();

    for consulta in consultas {
        let valor = calcular_valor_consulta_total(&consulta, medico_manager, especialidade_manager, exame_manager);
        let medico = medicos.iter().find(|m| m.codigo_medico == consulta.codigo_medico);
        if let Some(medico) = medico {
            let especialidade = especialidades.iter().find(|e| e.codigo_especialidade == medico.codigo_especialidade);
            if let Some(especialidade) = especialidade {
                *faturamento_por_especialidade.entry(&especialidade.descricao).or_insert(0.0) += valor;
            }
        }
    }

    println!("\n--- Faturamento por Especialidade ---");
    for (descricao, faturamento) in faturamento_por_especialidade {
        println!("{}: R$ {:.2}", descricao, faturamento);
    }
}

pub fn calcular_valor_consulta_total(
    consulta: &Consulta,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) -> f32 {
    let medico = medico_manager.read_record(consulta.codigo_medico).unwrap().unwrap();
    let especialidade = especialidade_manager.read_record(medico.codigo_especialidade).unwrap().unwrap();
    let exame = exame_manager.read_record(consulta.codigo_exame).unwrap().unwrap();
    
    especialidade.valor_consulta + exame.valor_exame
}
