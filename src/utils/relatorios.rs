use std::collections::HashSet;

use crate::{db::entity::FileManager, structs::{cidade::Cidade, consulta::Consulta, diaria::Diaria, especialidade::Especialidade, exame::Exame, medico::Medico, paciente::Paciente}};

fn relatorio_consultas_ordenadas(
    consulta_manager: &FileManager<Consulta>,
    paciente_manager: &FileManager<Paciente>,
    cidade_manager: &FileManager<Cidade>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
) {
    println!("\n--- Relatório de Consultas Ordenadas ---");

    let mut consultas = match consulta_manager.read_all_records() {
        Ok(c) => c,
        Err(_) => {
            println!("Erro ao ler registros de consultas.");
            return;
        }
    };

    consultas.sort_by(|a, b| a.codigo_consulta.cmp(&b.codigo_consulta));

    let mut pacientes_unicos = HashSet::new();
    let mut valor_total_a_pagar = 0.0;

    for consulta in consultas {
        let paciente = paciente_manager.read_record(consulta.codigo_paciente).unwrap_or(None);
        let medico = medico_manager.read_record(consulta.codigo_medico).unwrap_or(None);
        let exame = exame_manager.read_record(consulta.codigo_exame).unwrap_or(None);

        let nome_paciente = paciente.as_ref().map_or("Não encontrado".to_string(), |p| p.nome.clone());
        let nome_medico = medico.as_ref().map_or("Não encontrado".to_string(), |m| m.nome.clone());
        let desc_exame = exame.as_ref().map_or("Não encontrado".to_string(), |e| e.descricao.clone());

        let nome_cidade = if let Some(p) = &paciente {
            cidade_manager.read_record(p.codigo_cidade).unwrap_or(None).map_or("Não encontrada".to_string(), |c| c.descricao)
        } else {
            "Não encontrada".to_string()
        };

        let valor_consulta = medico.as_ref()
            .and_then(|m| especialidade_manager.read_record(m.codigo_especialidade).unwrap_or(None))
            .map_or(0.0, |e| e.valor_consulta);
        let valor_exame = exame.as_ref().map_or(0.0, |e| e.valor_exame);
        let valor_total = valor_consulta + valor_exame;

        valor_total_a_pagar += valor_total;
        pacientes_unicos.insert(consulta.codigo_paciente);

        println!("--------------------------------------------------");
        println!("Código da Consulta: {}", consulta.codigo_consulta);
        println!("Nome do Paciente: {}", nome_paciente);
        println!("Nome da Cidade do Paciente: {}", nome_cidade);
        println!("Nome do Médico: {}", nome_medico);
        println!("Descrição do Exame: {}", desc_exame);
        println!("Valor a ser Pago: R$ {:.2}", valor_total);
    }
    
    println!("--------------------------------------------------");
    println!("--- Resumo do Relatório ---");
    println!("Quantidade Total de Pacientes: {}", pacientes_unicos.len());
    println!("Valor Total a ser Pago: R$ {:.2}", valor_total_a_pagar);
    println!("--------------------------------------------------");
}


