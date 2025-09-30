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

fn menu_pacientes(manager: &mut FileManager<Paciente>, cidade_manager: &FileManager<Cidade>) {
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

                    // Requisito 2: Buscar e exibir a cidade e o estado
                    if let Ok(Some(cidade)) = cidade_manager.read_record(paciente.codigo_cidade) {
                        println!("Cidade: {}, Estado: {}", cidade.descricao, cidade.estado);
                    } else {
                        println!("Cidade: Não encontrada");
                    }
                    
                    // Requisito 2.1: Calcular e exibir o IMC e o diagnóstico
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
        println!("\n--- Gerenciar Médicos ---");
        println!("1. Incluir Médico");
        println!("2. Consultar Médico por código");
        println!("3. Excluir Médico");
        println!("4. Exibir todos os Médicos");
        println!("5. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo = ler_u32("Código do Médico: ");
                let nome = ler_string("Nome: ");
                let endereco = ler_string("Endereço: ");
                let telefone = ler_string("Telefone: ");
                let codigo_cidade = ler_u32("Código da Cidade: ");
                if cidade_manager.read_record(codigo_cidade).unwrap_or(None).is_none() {
                    println!("Erro: Cidade com código {} não encontrada.", codigo_cidade);
                    continue;
                }
                let codigo_especialidade = ler_u32("Código da Especialidade: ");
                if especialidade_manager.read_record(codigo_especialidade).unwrap_or(None).is_none() {
                    println!("Erro: Especialidade com código {} não encontrada.", codigo_especialidade);
                    continue;
                }
                let medico = Medico {
                    codigo_medico: codigo,
                    nome,
                    endereco,
                    telefone,
                    codigo_cidade,
                    codigo_especialidade,
                };
                if let Err(e) = manager.create_record(&medico, codigo) {
                    println!("Erro ao incluir Médico: {}", e);
                } else {
                    println!("Médico incluído com sucesso!");
                }
            }
            2 => {
                let codigo = ler_u32("Digite o código do Médico: ");
                if let Ok(Some(medico)) = manager.read_record(codigo) {
                    println!("{:?}", medico);
                } else {
                    println!("Médico não encontrado.");
                }
            }
            3 => {
                let codigo = ler_u32("Digite o código do Médico para excluir: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Médico excluído com sucesso!");
                } else {
                    println!("Médico não encontrado.");
                }
            }
            4 => {
                if let Ok(medicos) = manager.read_all_records() {
                    for medico in medicos {
                        println!("{:?}", medico);
                    }
                }
            }
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
        println!("\n--- Gerenciar Exames ---");
        println!("1. Incluir Exame");
        println!("2. Consultar Exame por código");
        println!("3. Excluir Exame");
        println!("4. Exibir todos os Exames");
        println!("5. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo = ler_u32("Código do Exame: ");
                let descricao = ler_string("Descrição: ");
                let codigo_especialidade = ler_u32("Código da Especialidade: ");
                if especialidade_manager.read_record(codigo_especialidade).unwrap_or(None).is_none() {
                    println!("Erro: Especialidade com código {} não encontrada.", codigo_especialidade);
                    continue;
                }
                let valor_exame = ler_f32("Valor do Exame: ");
                let exame = Exame {
                    codigo_exame: codigo,
                    descricao,
                    codigo_especialidade,
                    valor_exame,
                };
                if let Err(e) = manager.create_record(&exame, codigo) {
                    println!("Erro ao incluir Exame: {}", e);
                } else {
                    println!("Exame incluído com sucesso!");
                }
            }
            2 => {
                let codigo = ler_u32("Digite o código do Exame: ");
                if let Ok(Some(exame)) = manager.read_record(codigo) {
                    println!("{:?}", exame);
                } else {
                    println!("Exame não encontrado.");
                }
            }
            3 => {
                let codigo = ler_u32("Digite o código do Exame para excluir: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Exame excluído com sucesso!");
                } else {
                    println!("Exame não encontrado.");
                }
            }
            4 => {
                if let Ok(exames) = manager.read_all_records() {
                    for exame in exames {
                        println!("{:?}", exame);
                    }
                }
            }
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_consultas(
    manager: &mut FileManager<Consulta>,
    paciente_manager: &FileManager<Paciente>,
    medico_manager: &FileManager<Medico>,
    especialidade_manager: &FileManager<Especialidade>,
    exame_manager: &FileManager<Exame>,
    diaria_manager: &mut FileManager<Diaria>,
) {
    loop {
        println!("\n--- Gerenciar Consultas ---");
        println!("1. Incluir Consulta");
        println!("2. Consultar Consulta por código");
        println!("3. Excluir Consulta");
        println!("4. Exibir todas as Consultas");
        println!("5. Voltar");
        let choice = ler_opcao_menu();

        match choice {
            1 => {
                let codigo_consulta = ler_u32("Código da Consulta: ");
                
                let codigo_paciente = ler_u32("Código do Paciente: ");
                if paciente_manager.read_record(codigo_paciente).unwrap_or(None).is_none() {
                    println!("Erro: Paciente não encontrado.");
                    continue;
                }
                
                let codigo_medico = ler_u32("Código do Médico: ");
                if medico_manager.read_record(codigo_medico).unwrap_or(None).is_none() {
                    println!("Erro: Médico não encontrado.");
                    continue;
                }
                
                let codigo_exame = ler_u32("Código do Exame: ");
                if exame_manager.read_record(codigo_exame).unwrap_or(None).is_none() {
                    println!("Erro: Exame não encontrado.");
                    continue;
                }
                
                let data = ler_string("Data da Consulta (AAAAMMDD): ");
                let hora = ler_string("Hora da Consulta (HHMM): ");

                let medico = medico_manager.read_record(codigo_medico).unwrap().unwrap();
                let especialidade = especialidade_manager.read_record(medico.codigo_especialidade).unwrap().unwrap();
                let codigo_dia = data.parse::<u32>().unwrap_or(0);
                
                let diaria_atual = diaria_manager.read_record(codigo_dia).unwrap_or(None).unwrap_or(Diaria {
                    codigo_dia,
                    codigo_especialidade: especialidade.codigo_especialidade,
                    quantidade_consultas: 0,
                });
                if diaria_atual.quantidade_consultas >= especialidade.limite_diario {
                    println!("Erro: Limite de consultas diárias para a especialidade '{}' atingido.", especialidade.descricao);
                    continue;
                }

                let consulta = Consulta {
                    codigo_consulta,
                    codigo_paciente,
                    codigo_medico,
                    codigo_exame,
                    data: data.clone(),
                    hora,
                };
                if let Err(e) = manager.create_record(&consulta, codigo_consulta) {
                    println!("Erro ao incluir Consulta: {}", e);
                } else {
                    println!("Consulta incluída com sucesso!");
                    atualizar_diaria(diaria_manager, codigo_dia, especialidade.codigo_especialidade, 1);
                }
            }
            2 => {
                let codigo = ler_u32("Digite o código da Consulta: ");
                if let Ok(Some(consulta)) = manager.read_record(codigo) {
                    println!("{:#?}", consulta);
                } else {
                    println!("Consulta não encontrada.");
                }
            }
            3 => {
                let codigo = ler_u32("Digite o código da Consulta para excluir: ");
                if let Ok(Some(consulta)) = manager.read_record(codigo) {
                    if let Ok(true) = manager.delete_record(codigo) {
                        println!("Consulta excluída com sucesso!");
                        let medico = medico_manager.read_record(consulta.codigo_medico).unwrap().unwrap();
                        let codigo_dia = consulta.data.parse::<u32>().unwrap_or(0);
                        atualizar_diaria(diaria_manager, codigo_dia, medico.codigo_especialidade, -1);
                    } else {
                        println!("Consulta não encontrada.");
                    }
                } else {
                    println!("Consulta não encontrada.");
                }
            }
            4 => {
                if let Ok(consultas) = manager.read_all_records() {
                    for consulta in consultas {
                        println!("{:#?}", consulta);
                    }
                }
            }
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
    let mut diaria = diaria_manager.read_record(codigo_dia).unwrap_or(None).unwrap_or(Diaria {
        codigo_dia,
        codigo_especialidade,
        quantidade_consultas: 0,
    });
    
    if incremento > 0 {
        diaria.quantidade_consultas += 1;
    } else if incremento < 0 && diaria.quantidade_consultas > 0 {
        diaria.quantidade_consultas -= 1;
    }

    if let Err(e) = diaria_manager.delete_record(codigo_dia) {
        println!("Erro ao atualizar diária (deleção): {}", e);
    }
    
    if let Err(e) = diaria_manager.create_record(&diaria, codigo_dia) {
        println!("Erro ao atualizar diária (inclusão): {}", e);
    }
}