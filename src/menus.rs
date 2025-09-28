use std::io::{self, Write};

use crate::{db::entity::FileManager, structs::{cidade::Cidade, consulta::Consulta, diaria::Diaria, especialidade::Especialidade, exame::Exame, medico::Medico, paciente::Paciente}};

pub fn exibir_menu_principal() {
    println!("\n--- Menu Principal ---");
    println!("1. Gerenciar Pacientes");
    println!("2. Gerenciar Médicos");
    println!("3. Gerenciar Especialidades");
    println!("4. Gerenciar Cidades");
    println!("5. Gerenciar Exames");
    println!("6. Gerenciar Consultas");
    println!("7. Gerenciar Diárias");
    println!("8. Sair");
}

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

                    // Requisito 3: Buscar e exibir a cidade e o estado
                    if let Ok(Some(cidade)) = cidade_manager.read_record(medico.codigo_cidade) {
                        println!("Cidade: {}, Estado: {}", cidade.descricao, cidade.estado);
                    } else {
                        println!("Cidade: Não encontrada");
                    }
                    
                    // Requisito 3.1: Buscar e exibir os dados da especialidade
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
        println!("\n--- Gerenciamento de Especialidades ---");
        println!("1. Inserir nova especialidade");
        println!("2. Consultar especialidade por código");
        println!("3. Excluir especialidade por código");
        println!("4. Listar todas as especialidades");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código da Especialidade: ");
                let descricao = ler_string("Descrição: ");
                let valor = ler_f32("Valor da Consulta: ");
                let limite = ler_u32("Limite Diário: ");

                let nova_especialidade = Especialidade { codigo_especialidade: codigo, descricao, valor_consulta: valor, limite_diario: limite };
                if let Err(e) = manager.create_record(&nova_especialidade, codigo) {
                    eprintln!("Erro ao inserir especialidade: {}", e);
                } else {
                    println!("Especialidade inserida com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código da especialidade para consulta: ");
                if let Ok(Some(especialidade)) = manager.read_record(codigo) {
                    println!("Registro encontrado: {:?}", especialidade);
                } else {
                    println!("Especialidade não encontrada.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código da especialidade para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Especialidade excluída (logicamente) com sucesso!");
                } else {
                    println!("Especialidade não encontrada ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(especialidades) = manager.read_all_records() {
                    println!("--- Lista de Todas as Especialidades ---");
                    for e in especialidades {
                        println!("{:?}", e);
                    }
                } else {
                    println!("Erro ao listar especialidades.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_cidades(manager: &mut FileManager<Cidade>) {
    loop {
        println!("\n--- Gerenciamento de Cidades ---");
        println!("1. Inserir nova cidade");
        println!("2. Consultar cidade por código");
        println!("3. Excluir cidade por código");
        println!("4. Listar todas as cidades");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo = ler_u32("Código da Cidade: ");
                let descricao = ler_string("Descrição: ");
                let estado = ler_string("Estado: ");

                let nova_cidade = Cidade { codigo_cidade: codigo, descricao, estado };
                if let Err(e) = manager.create_record(&nova_cidade, codigo) {
                    eprintln!("Erro ao inserir cidade: {}", e);
                } else {
                    println!("Cidade inserida com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código da cidade para consulta: ");
                if let Ok(Some(cidade)) = manager.read_record(codigo) {
                    println!("Registro encontrado: {:?}", cidade);
                } else {
                    println!("Cidade não encontrada.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código da cidade para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Cidade excluída (logicamente) com sucesso!");
                } else {
                    println!("Cidade não encontrada ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(cidades) = manager.read_all_records() {
                    println!("--- Lista de Todas as Cidades ---");
                    for c in cidades {
                        println!("{:?}", c);
                    }
                } else {
                    println!("Erro ao listar cidades.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

pub fn menu_exames(manager: &mut FileManager<Exame>) {
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
                    println!("Registro encontrado: {:?}", exame);
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

pub fn menu_consultas(manager: &mut FileManager<Consulta>) {
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
                let data = ler_string("Data (AAAA-MM-DD): ");
                let hora = ler_string("Hora (HH:MM): ");

                let nova_consulta = Consulta { codigo_consulta: codigo, codigo_paciente, codigo_medico, codigo_exame, data, hora };
                if let Err(e) = manager.create_record(&nova_consulta, codigo) {
                    eprintln!("Erro ao inserir consulta: {}", e);
                } else {
                    println!("Consulta inserida com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código da consulta para consulta: ");
                if let Ok(Some(consulta)) = manager.read_record(codigo) {
                    println!("Registro encontrado: {:?}", consulta);
                } else {
                    println!("Consulta não encontrada.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código da consulta para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Consulta excluída (logicamente) com sucesso!");
                } else {
                    println!("Consulta não encontrada ou erro na exclusão.");
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
        println!("\n--- Gerenciamento de Diárias ---");
        println!("1. Inserir nova diária");
        println!("2. Consultar diária por código");
        println!("3. Excluir diária por código");
        println!("4. Listar todas as diárias");
        println!("5. Voltar ao menu principal");

        let choice = ler_opcao_menu();
        match choice {
            1 => {
                let codigo_dia = ler_u32("Código do Dia (AAAAMMDD): ");
                let codigo_especialidade = ler_u32("Código da Especialidade: ");
                let quantidade = ler_u32("Quantidade de Consultas: ");

                let nova_diaria = Diaria { codigo_dia, codigo_especialidade, quantidade_consultas: quantidade };
                if let Err(e) = manager.create_record(&nova_diaria, codigo_dia) {
                    eprintln!("Erro ao inserir diária: {}", e);
                } else {
                    println!("Diária inserida com sucesso!");
                }
            },
            2 => {
                let codigo = ler_u32("Digite o código da diária para consulta: ");
                if let Ok(Some(diaria)) = manager.read_record(codigo) {
                    println!("Registro encontrado: {:?}", diaria);
                } else {
                    println!("Diária não encontrada.");
                }
            },
            3 => {
                let codigo = ler_u32("Digite o código da diária para exclusão: ");
                if let Ok(true) = manager.delete_record(codigo) {
                    println!("Diária excluída (logicamente) com sucesso!");
                } else {
                    println!("Diária não encontrada ou erro na exclusão.");
                }
            },
            4 => {
                if let Ok(diarias) = manager.read_all_records() {
                    println!("--- Lista de Todas as Diárias ---");
                    for d in diarias {
                        println!("{:?}", d);
                    }
                } else {
                    println!("Erro ao listar diárias.");
                }
            },
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}