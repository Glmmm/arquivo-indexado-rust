use crate::db::file_manager::{FileManager};
use crate::structs::{
    cidade::Cidade, consulta::Consulta, diaria::Diaria,
    especialidade::Especialidade, exame::Exame, medico::Medico,
    paciente::Paciente,
};
use std::io::{self, Write};


pub fn ler_opcao_menu() -> u32 {
    print!("Digite sua opção: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

pub fn ler_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn ler_u32(prompt: &str) -> u32 {
    loop {
        let input = ler_string(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Por favor, digite um número."),
        }
    }
}

pub fn ler_f32(prompt: &str) -> f32 {
    loop {
        let input = ler_string(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Por favor, digite um número decimal."),
        }
    }
}


pub fn exibir_menu_principal() {
    println!("\n--- Menu Principal ---");
    println!("1. Gerenciar Pacientes");
    println!("2. Gerenciar Médicos");
    println!("3. Gerenciar Especialidades");
    println!("4. Gerenciar Cidades");
    println!("5. Gerenciar Exames");
    println!("6. Gerenciar Consultas");
    println!("7. Gerenciar Diárias");
    println!("8. Relatórios de Faturamento");
    println!("9. Relatório de Consultas");
    println!("10. Sair");
}

pub fn menu_pacientes(manager: &mut FileManager<Paciente>, cidade_manager: &FileManager<Cidade>) {
    loop {
        println!("\n--- Gerenciamento de Pacientes ---");
        println!("1. Inserir novo paciente");
        println!("2. Consultar paciente por código");
        println!("3. Excluir paciente por código");
        println!("4. Listar todos os pacientes");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código do Paciente: ");
                let nome = ler_string("Nome: ");
                let data_nascimento = ler_string("Data de Nascimento: ");
                let endereco = ler_string("Endereço: ");
                let telefone = ler_string("Telefone: ");
                let codigo_cidade = ler_u32("Código da Cidade: ");
                let peso = ler_f32("Peso (kg): ");
                let altura = ler_f32("Altura (m): ");

                let novo_paciente = Paciente { codigo_paciente: codigo, nome, data_nascimento, endereco, telefone, codigo_cidade, peso, altura };
                if let Err(e) = manager.create_record(&novo_paciente, codigo) {
                    eprintln!("Erro ao inserir paciente: {}", e);
                } else {
                    println!("Paciente inserido com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código do paciente para consulta: ");
                if let Ok(Some(paciente)) = manager.read_record(codigo) {
                    println!("\n--- Dados do Paciente ---");
                    println!("Código: {}", paciente.codigo_paciente);
                    println!("Nome: {}", paciente.nome);
                    println!("Data de Nascimento: {}", paciente.data_nascimento);
                    println!("Endereço: {}", paciente.endereco);
                    println!("Telefone: {}", paciente.telefone);

                    if let Ok(Some(cidade)) = cidade_manager.read_record(paciente.codigo_cidade) {
                        println!("Cidade: {}, Estado: {}", cidade.descricao, cidade.estado);
                    } else {
                        println!("Cidade: Não encontrada");
                    }
                    
                    let imc = paciente.peso / (paciente.altura * paciente.altura);
                    let diagnostico = match imc {
                        _ if imc < 18.5 => "Abaixo do peso",
                        _ if imc < 25.0 => "Peso normal",
                        _ if imc < 30.0 => "Sobrepeso",
                        _ => "Obesidade",
                    };
                    println!("Peso: {:.2} kg", paciente.peso);
                    println!("Altura: {:.2} m", paciente.altura);
                    println!("IMC: {:.2}", imc);
                    println!("Diagnóstico: {}", diagnostico);

                } else {
                    println!("Paciente não encontrado.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código do paciente para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Paciente excluído (logicamente) com sucesso!");
                } else {
                    println!("Paciente não encontrado ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(pacientes) = manager.read_all_records() {
                    println!("--- Lista de Todos os Pacientes ---");
                    for p in pacientes {
                        println!("{:?}", p);
                    }
                } else {
                    println!("Erro ao listar pacientes.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}
pub fn menu_medicos(manager: &mut FileManager<Medico>, cidade_manager: &FileManager<Cidade>, especialidade_manager: &FileManager<Especialidade>) {
    loop {
        println!("\n--- Gerenciamento de Médicos ---");
        println!("1. Inserir novo médico");
        println!("2. Consultar médico por código");
        println!("3. Excluir médico por código");
        println!("4. Listar todos os médicos");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código do Médico: ");
                let nome = ler_string("Nome: ");
                let endereco = ler_string("Endereço: ");
                let telefone = ler_string("Telefone: ");
                let codigo_cidade = ler_u32("Código da Cidade: ");
                let codigo_especialidade = ler_u32("Código da Especialidade: ");

                let novo_medico = Medico { codigo_medico: codigo, nome, endereco, telefone, codigo_cidade, codigo_especialidade };
                if let Err(e) = manager.create_record(&novo_medico, codigo) {
                    eprintln!("Erro ao inserir médico: {}", e);
                } else {
                    println!("Médico inserido com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código do médico para consulta: ");
                if let Ok(Some(medico)) = manager.read_record(codigo) {
                    println!("\n--- Dados do Médico ---");
                    println!("Código: {}", medico.codigo_medico);
                    println!("Nome: {}", medico.nome);
                    println!("Endereço: {}", medico.endereco);
                    println!("Telefone: {}", medico.telefone);

                    if let Ok(Some(cidade)) = cidade_manager.read_record(medico.codigo_cidade) {
                        println!("Cidade: {}, Estado: {}", cidade.descricao, cidade.estado);
                    } else {
                        println!("Cidade: Não encontrada");
                    }
                    
                    if let Ok(Some(especialidade)) = especialidade_manager.read_record(medico.codigo_especialidade) {
                        println!("Especialidade: {}", especialidade.descricao);
                        println!("Valor da Consulta: R$ {:.2}", especialidade.valor_consulta);
                        println!("Limite Diário de Consultas: {}", especialidade.limite_diario);
                    } else {
                        println!("Especialidade: Não encontrada");
                    }

                } else {
                    println!("Médico não encontrado.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código do médico para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Médico excluído (logicamente) com sucesso!");
                } else {
                    println!("Médico não encontrado ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(medicos) = manager.read_all_records() {
                    println!("--- Lista de Todos os Médicos ---");
                    for m in medicos {
                        println!("{:?}", m);
                    }
                } else {
                    println!("Erro ao listar médicos.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}
pub fn menu_especialidades(manager: &mut FileManager<Especialidade>) {
    loop {
        println!("\n--- Gerenciar Especialidades ---");
        println!("1. Incluir Especialidade");
        println!("2. Consultar Especialidade por código");
        println!("3. Excluir Especialidade");
        println!("4. Exibir todas as Especialidades");
        println!("5. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo = ler_u32("Código da Especialidade: ");
                let descricao = ler_string("Descrição: ");
                let valor_consulta = ler_f32("Valor da Consulta: ");
                let limite_diario = ler_u32("Limite Diário de Consultas: ");
                let especialidade = Especialidade {
                    codigo_especialidade: codigo,
                    descricao,
                    valor_consulta,
                    limite_diario,
                };
                if let Err(e) = manager.create_record(&especialidade, codigo) {
                    println!("Erro ao incluir Especialidade: {}", e);
                } else {
                    println!("Especialidade incluída com sucesso!");
                }
            }
            2 => {
                let codigo = ler_u32("Digite o código da Especialidade: ");
                if let Ok(Some(especialidade)) = manager.read_record(codigo) {
                    println!("{:?}", especialidade);
                } else {
                    println!("Especialidade não encontrada.");
                }
            }
            3 => {
                let codigo = ler_u32("Digite o código da Especialidade para excluir: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Especialidade excluída com sucesso!");
                } else {
                    println!("Especialidade não encontrada.");
                }
            }
            4 => {
                if let Ok(especialidades) = manager.read_all_records() {
                    for especialidade in especialidades {
                        println!("{:?}", especialidade);
                    }
                }
            }
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_cidades(manager: &mut FileManager<Cidade>) {
    loop {
        println!("\n--- Gerenciar Cidades ---");
        println!("1. Incluir Cidade");
        println!("2. Consultar Cidade por código");
        println!("3. Excluir Cidade");
        println!("4. Exibir todas as Cidades");
        println!("5. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo = ler_u32("Código da Cidade: ");
                let descricao = ler_string("Descrição: ");
                let estado = ler_string("Estado: ");
                let cidade = Cidade { codigo_cidade: codigo, descricao, estado };
                if let Err(e) = manager.create_record(&cidade, codigo) {
                    println!("Erro ao incluir Cidade: {}", e);
                } else {
                    println!("Cidade incluída com sucesso!");
                }
            }
            2 => {
                let codigo = ler_u32("Digite o código da Cidade: ");
                if let Ok(Some(cidade)) = manager.read_record(codigo) {
                    println!("{:?}", cidade);
                } else {
                    println!("Cidade não encontrada.");
                }
            }
            3 => {
                let codigo = ler_u32("Digite o código da Cidade para excluir: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Cidade excluída com sucesso!");
                } else {
                    println!("Cidade não encontrada.");
                }
            }
            4 => {
                if let Ok(cidades) = manager.read_all_records() {
                    for cidade in cidades {
                        println!("{:?}", cidade);
                    }
                }
            }
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_exames(manager: &mut FileManager<Exame>, especialidade_manager: &FileManager<Especialidade>) {
    loop {
        println!("\n--- Gerenciamento de Exames ---");
        println!("1. Inserir novo exame");
        println!("2. Consultar exame por código");
        println!("3. Excluir exame por código");
        println!("4. Listar todos os exames");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código do Exame: ");
                let descricao = ler_string("Descrição: ");
                let codigo_especialidade = ler_u32("Código da Especialidade: ");
                let valor = ler_f32("Valor do Exame: ");
                
                if let Ok(Some(especialidade)) = especialidade_manager.read_record(codigo_especialidade) {
                    println!("Especialidade selecionada: {}", especialidade.descricao);
                } else {
                    println!("Atenção: A especialidade com o código {} não foi encontrada.", codigo_especialidade);
                }

                let novo_exame = Exame { codigo_exame: codigo, descricao, codigo_especialidade, valor_exame: valor };
                if let Err(e) = manager.create_record(&novo_exame, codigo) {
                    eprintln!("Erro ao inserir exame: {}", e);
                } else {
                    println!("Exame inserido com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código do exame para consulta: ");
                if let Ok(Some(exame)) = manager.read_record(codigo) {
                    println!("\n--- Dados do Exame ---");
                    println!("Código: {}", exame.codigo_exame);
                    println!("Descrição: {}", exame.descricao);
                    println!("Valor do Exame: R$ {:.2}", exame.valor_exame);

                    if let Ok(Some(especialidade)) = especialidade_manager.read_record(exame.codigo_especialidade) {
                        println!("Especialidade: {}", especialidade.descricao);
                    } else {
                        println!("Especialidade: Não encontrada");
                    }
                } else {
                    println!("Exame não encontrado.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código do exame para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Exame excluído (logicamente) com sucesso!");
                } else {
                    println!("Exame não encontrado ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(exames) = manager.read_all_records() {
                    println!("--- Lista de Todos os Exames ---");
                    for e in exames {
                        println!("{:?}", e);
                    }
                } else {
                    println!("Erro ao listar exames.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}


pub fn menu_consultas(
    manager: &mut FileManager<Consulta>,
    paciente_manager: &FileManager<Paciente>,
    medico_manager: &FileManager<Medico>,
    cidade_manager: &FileManager<Cidade>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
    diaria_manager: &mut FileManager<Diaria>,
) {
    loop {
        println!("\n--- Gerenciamento de Consultas ---");
        println!("1. Inserir nova consulta");
        println!("2. Consultar consulta por código");
        println!("3. Excluir consulta por código");
        println!("4. Listar todas as consultas");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código da Consulta: ");
                let codigo_paciente = ler_u32("Código do Paciente: ");
                let codigo_medico = ler_u32("Código do Médico: ");
                let codigo_exame = ler_u32("Código do Exame: ");
                let data = ler_string("Data (AAAAMMDD): ");

                let medico = medico_manager.read_record(codigo_medico).unwrap_or(None);
                let especialidade_medico = if let Some(m) = &medico {
                    especialidade_manager.read_record(m.codigo_especialidade).unwrap_or(None)
                } else {
                    None
                };

                if let Some(esp) = &especialidade_medico {
                    let diaria = diaria_manager.read_record(data.parse().unwrap()).unwrap_or(None);
                    let consultas_do_dia = if let Some(d) = &diaria { d.quantidade_consultas } else { 0 };

                    if consultas_do_dia >= esp.limite_diario {
                        println!("ATENÇÃO: Não há mais vagas para esta especialidade hoje. Limite diário de {} atingido.", esp.limite_diario);
                        continue;
                    }
                } else {
                    println!("Atenção: Médico ou especialidade não encontrados. Não é possível verificar o limite diário.");
                }

                let paciente = paciente_manager.read_record(codigo_paciente).unwrap_or(None);
                let exame = exame_manager.read_record(codigo_exame).unwrap_or(None);

                let valor_consulta = especialidade_medico.as_ref().map_or(0.0, |e| e.valor_consulta);
                let valor_exame = exame.as_ref().map_or(0.0, |e| e.valor_exame);
                let valor_total = valor_consulta + valor_exame;
                
                let nome_paciente = paciente.as_ref().map_or("Não encontrado".to_string(), |p| p.nome.clone());
                let nome_medico = medico.as_ref().map_or("Não encontrado".to_string(), |m| m.nome.clone());
                let desc_exame = exame.as_ref().map_or("Não encontrado".to_string(), |e| e.descricao.clone());

                println!("\n--- Resumo da Consulta ---");
                println!("Paciente: {}", nome_paciente);
                println!("Médico: {}", nome_medico);
                println!("Exame: {}", desc_exame);
                println!("Valor Total a Pagar: R$ {:.2}", valor_total);
                println!("--------------------------");

                let nova_consulta = Consulta { codigo_consulta: codigo, codigo_paciente, codigo_medico, codigo_exame, data, hora: ler_string("Hora (HH:MM): ") };
                if let Err(e) = manager.create_record(&nova_consulta, codigo) {
                    eprintln!("Erro ao inserir consulta: {}", e);
                } else {
                    println!("Consulta inserida com sucesso!");
                    if let Some(esp) = especialidade_medico {
                        atualizar_diaria(diaria_manager, nova_consulta.data.parse().unwrap(), esp.codigo_especialidade, 1);
                    }
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código da consulta para consulta: ");
                if let Ok(Some(consulta)) = manager.read_record(codigo) {
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

                    println!("\n--- Detalhes da Consulta ---");
                    println!("Código: {}", consulta.codigo_consulta);
                    println!("Paciente: {}", nome_paciente);
                    println!("Cidade do Paciente: {}", nome_cidade);
                    println!("Médico: {}", nome_medico);
                    println!("Exame: {}", desc_exame);
                    println!("Data: {}", consulta.data);
                    println!("Hora: {}", consulta.hora);
                    println!("Valor Total a Pagar: R$ {:.2}", valor_total);
                    
                } else {
                    println!("Consulta não encontrada.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código da consulta para exclusão: ");
                if let Ok(Some(consulta)) = manager.read_record(codigo) {
                    if let Ok(true) = manager.delete_record(codigo) {
                        println!("Consulta excluída (logicamente) com sucesso!");
                        if let Some(medico) = medico_manager.read_record(consulta.codigo_medico).unwrap_or(None) {
                            if let Some(especialidade) = especialidade_manager.read_record(medico.codigo_especialidade).unwrap_or(None) {
                                atualizar_diaria(diaria_manager, consulta.data.parse().unwrap(), especialidade.codigo_especialidade, -1);
                            }
                        }
                    } else {
                        println!("Consulta não encontrada ou erro na exclusão.");
                    }
                } else {
                    println!("Consulta não encontrada.");
                }
            },
            4 => {
                if let Ok(consultas) = manager.read_all_records() {
                    println!("--- Lista de Todas as Consultas ---");
                    for c in consultas {
                        println!("{:?}", c);
                    }
                } else {
                    println!("Erro ao listar consultas.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_diarias(manager: &mut FileManager<Diaria>) {
    loop {
        println!("\n--- Gerenciar Diárias ---");
        println!("1. Consultar Diária por código (AAAAMMDD)");
        println!("2. Exibir todas as Diárias");
        println!("3. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo = ler_u32("Digite o código (AAAAMMDD) da Diária: ");
                if let Ok(Some(diaria)) = manager.read_record(codigo) {
                    println!("{:?}", diaria);
                } else {
                    println!("Diária não encontrada.");
                }
            }
            2 => {
                if let Ok(diarias) = manager.read_all_records() {
                    for diaria in diarias {
                        println!("{:?}", diaria);
                    }
                }
            }
            3 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn atualizar_diaria(diaria_manager: &mut FileManager<Diaria>, codigo_dia: u32, codigo_especialidade: u32, incremento: i32) {
    if let Ok(Some(mut diaria)) = diaria_manager.read_record(codigo_dia) {
        diaria.quantidade_consultas = (diaria.quantidade_consultas as i32 + incremento) as u32;
        if let Err(e) = diaria_manager.create_record(&diaria, codigo_dia) {
            eprintln!("Erro ao atualizar diária: {}", e);
        }
    } else if incremento > 0 {
        let nova_diaria = Diaria {
            codigo_dia,
            codigo_especialidade,
            quantidade_consultas: incremento as u32,
        };
        if let Err(e) = diaria_manager.create_record(&nova_diaria, codigo_dia) {
            eprintln!("Erro ao criar nova diária: {}", e);
        }
    }
}