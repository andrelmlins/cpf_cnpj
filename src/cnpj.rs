use rand::Rng;

/// Converte um caractere para seu valor numérico para cálculo do DV.
/// - Dígitos 0-9: retornam 0-9
/// - Letras A-Z: retornam 17-42
///
/// Referência: https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/documentos-tecnicos/cnpj/manual-dv-cnpj.pdf
///
/// Letras minúsculas são convertidas para maiúsculas automaticamente.
fn char_to_value(c: char) -> Option<usize> {
    const ASCII_ZERO: usize = '0' as usize;
    let upper = c.to_ascii_uppercase();
    match upper {
        '0'..='9' => Some((upper as usize) - ASCII_ZERO),
        'A'..='Z' => Some((upper as usize) - ASCII_ZERO),
        _ => None,
    }
}

/// Converte um dígito (0-9) para seu caractere correspondente.
/// Usa conversão direta via ASCII, evitando unwrap.
fn digit_to_char(d: usize) -> char {
    (b'0' + d as u8) as char
}

pub fn validate(valor: &str) -> bool {
    let numbers: Vec<usize> = match valor
        .chars()
        .filter(|c| !"./-".contains(*c))
        .map(char_to_value)
        .collect::<Option<Vec<_>>>()
    {
        Some(nums) => nums,
        None => return false,
    };

    if numbers.len() != 14 || equal_digits(&numbers) {
        return false;
    }

    if numbers[12] > 9 || numbers[13] > 9 {
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[12] {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[13] {
        return false;
    }

    true
}

/// Gera um CNPJ numérico válido (formato tradicional antes julho/2026)
pub fn generate() -> String {
    let mut rng = rand::rng();

    let mut vec: Vec<usize> = (0..8).map(|_| rng.random_range(0..10)).collect();

    vec.extend(vec![0, 0, 0, 1]);
    vec.push(validate_first_digit(&vec));
    vec.push(validate_second_digit(&vec));

    vec.into_iter().map(digit_to_char).collect()
}

/// Gera um CNPJ alfanumérico válido (novo formato a partir de julho/2026)
/// Usa caracteres permitidos: 0-9, A-Z (exceto I, O, Q, F para evitar confusão visual)
/// Estamos evitando usar o I,O,Q para não confundir as semelhanças entre I e 1 ou L, O, 0 e Q.
/// Porém na função de validate consideramos todos
pub fn generate_alphanumeric() -> String {
    let mut rng = rand::rng();
    const CHARSET: &[u8] = b"0123456789ABCDEGHJKLMNPRSTUVWXYZ";
    let mut chars: Vec<char> = (0..8)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect();

    chars.extend((0..4).map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char));
    let values: Vec<usize> = chars.iter().filter_map(|&c| char_to_value(c)).collect();

    let dv1 = validate_first_digit(&values);
    let dv2 = {
        let mut values_with_dv1 = values.clone();
        values_with_dv1.push(dv1);
        validate_second_digit(&values_with_dv1)
    };

    chars.push(digit_to_char(dv1));
    chars.push(digit_to_char(dv2));

    chars.into_iter().collect()
}

fn validate_first_digit(numbers: &[usize]) -> usize {
    const WEIGHTS: [usize; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: usize = numbers
        .iter()
        .take(12)
        .zip(WEIGHTS.iter())
        .map(|(n, w)| n * w)
        .sum();

    let result = sum % 11;
    if result < 2 {
        0
    } else {
        11 - result
    }
}

fn validate_second_digit(numbers: &[usize]) -> usize {
    const WEIGHTS: [usize; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: usize = numbers
        .iter()
        .take(13)
        .zip(WEIGHTS.iter())
        .map(|(n, w)| n * w)
        .sum();

    let result = sum % 11;
    if result < 2 {
        0
    } else {
        11 - result
    }
}

fn equal_digits(numbers: &[usize]) -> bool {
    numbers.windows(2).all(|w| w[0] == w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_valid_cnpj() {
        assert!(validate("11222333000181"));
    }

    #[test]
    fn should_validate_valid_cnpj_with_formatting() {
        assert!(validate("11.222.333/0001-81"));
    }

    #[test]
    fn should_reject_invalid_cnpj() {
        assert!(!validate("11222333000182"));
    }

    #[test]
    fn should_reject_cnpj_with_all_same_digits() {
        assert!(!validate("11111111111111"));
    }

    #[test]
    fn should_reject_masked_cnpj_without_panic() {
        assert!(!validate("**.222.333/0001-**"));
    }

    #[test]
    fn should_reject_cnpj_with_letters_in_dv() {
        assert!(!validate("11222333000AB1"));
    }

    #[test]
    fn should_reject_cnpj_with_special_characters() {
        assert!(!validate("11222333@00181"));
    }

    #[test]
    fn should_generate_valid_cnpj() {
        let cnpj = generate();
        assert!(validate(&cnpj));
    }

    #[test]
    fn should_validate_alphanumeric_cnpj() {
        assert!(validate("12ABC34501DE35"));
    }

    #[test]
    fn should_validate_alphanumeric_cnpj_with_formatting() {
        assert!(validate("12.ABC.345/01DE-35"));
    }

    #[test]
    fn should_reject_alphanumeric_cnpj_with_invalid_dv() {
        assert!(!validate("12ABC34501DE99"));
    }

    #[test]
    fn should_reject_cnpj_with_letter_in_dv_positions() {
        assert!(!validate("12ABC34501DEAB"));
    }

    #[test]
    fn should_validate_cnpj_with_lowercase_letters() {
        assert!(validate("12abc34501de35"));
    }

    #[test]
    fn should_generate_valid_alphanumeric_cnpj() {
        let cnpj = generate_alphanumeric();
        assert!(validate(&cnpj));
    }

    #[test]
    fn should_maintain_backward_compatibility_with_numeric_cnpj() {
        assert!(validate("11222333000181"));
        assert!(validate("11.222.333/0001-81"));
    }

    #[test]
    fn should_validate_cnpjs_generated_by_receita_federal() {
        // CNPJs alfanuméricos gerados diretamente pela Receita Federal em Jan/2026
        const CNPJS_RECEITA_FEDERAL: &str = "SY.99M.4WL/0001-22;SY.99M.4WL/1G7A-66;4B.7BJ.E7D/0001-45;4B.7BJ.E7D/AVWA-36;PW.6EE.Z91/0001-70;PW.6EE.Z91/6D9K-57;YA.THW.A3M/0001-98;YA.THW.A3M/HGCA-57;9M.05N.MN2/0001-51;9M.05N.MN2/TZR5-92;9M.05N.MN2/182A-75;9M.05N.MN2/CJ0W-56;9M.05N.MN2/XYKH-99;9M.05N.MN2/TABH-92;9M.05N.MN2/XSJN-26;9M.05N.MN2/65YW-96;9M.05N.MN2/30AR-53;9M.05N.MN2/P6EK-36;9M.05N.MN2/G9PP-46;9M.05N.MN2/1LJ7-74;9M.05N.MN2/6DHB-97;9M.05N.MN2/R3WW-49;9M.05N.MN2/MNZP-33;9M.05N.MN2/P5BG-24;9M.05N.MN2/LBPE-99;9M.05N.MN2/WGSG-90;9M.05N.MN2/1MKC-90;9M.05N.MN2/PWXN-63;LM.7BN.GKB/0001-13;4P.NEB.EYM/0001-58;MY.4WX.JS1/0001-14;59.TX1.BNP/0001-84;3R.M97.VNX/0001-35;3E.ZWJ.C2W/0001-79;6X.SYK.X29/0001-22;HJ.1WX.RG3/0001-44;N3.8XW.PX5/0001-31;17.D9M.KZN/0001-38;42.9WX.XY3/0001-27;1R.L6W.1HX/0001-60;AN.BVP.3X3/0001-07;2W.W2Y.VZN/0001-51;SZ.CCG.WNJ/0001-60;RR.3CC.775/0001-00;LE.C2R.M40/0001-58;SY.4TG.MK4/0001-15;LC.KS8.ZAM/0001-84;ES.NCY.KA6/0001-73;8T.BRJ.J9V/0001-48;TK.1ZJ.W18/0001-14;47.GMA.4TC/0001-99;EX.56H.P9V/0001-04;DG.JGC.2JX/0001-88;WN.J9L.PPB/0001-18;1E.1J2.8TX/0001-66;LA.1W8.7V5/0001-08;9R.Y75.YSL/0001-50;HS.D3J.2GL/0001-67;4S.31A.VZR/0001-50;H3.KZ9.AV4/0001-89;90.ST3.7RW/0001-21;B0.945.S91/0001-76;D8.42T.R72/0001-72;9D.SH4.ZRB/0001-37;67.PGX.VB3/0001-27;8Y.LS1.BPZ/0001-10;KT.MZE.R5K/0001-10;N7.8NN.HX0/0001-69;7X.CAC.48Y/0001-06;SX.1GZ.251/0001-46;T6.ENJ.7V0/0001-64;MK.DTS.4W0/0001-57;2K.SSL.DSV/0001-86;4B.VH2.8AN/0001-15;MC.ZK6.PAM/0001-65;RP.G2L.NJC/0001-52;EH.YH6.8MT/0001-20;8D.3E7.45V/0001-87;L0.EPA.WDB/0001-87;J2.X8H.TLM/0001-54;KR.BS9.CS4/0001-38;Y8.J31.J63/0001-59;KR.E0S.D3H/0001-32;4S.EW8.BBG/0001-99;KD.7B9.TP3/0001-61;K2.05A.0WM/0001-41;0A.6EZ.ADM/0001-06;PL.EY0.TAX/0001-13;RD.JN8.THT/0001-02;A5.5AP.EV3/0001-61;D6.P4Y.8V4/0001-45;JK.170.LA5/0001-81;KL.SYE.084/0001-02;MB.Z8K.R7H/0001-96;XV.LYT.NBT/0001-45;60.B8B.M4T/0001-85;G4.YL5.8VE/0001-12;W6.HME.HZ5/0001-00;AN.R3L.N7P/0001-81;VN.43E.BZ0/0001-00;7R.1SG.4GX/0001-50;7L.MSX.TEJ/0001-81;0N.DED.EST/0001-18;7C.WZV.099/0001-80;4N.581.JAX/0001-07;26.CX7.736/0001-36;52.0KC.1CS/0001-55;TY.EMK.CA3/0001-29;K4.TBB.1W5/0001-06;XX.CL2.1E9/0001-27;YP.GSM.N01/0001-99;1R.WC5.X0H/0001-72;6M.BW8.5W9/0001-72;AH.VBV.YP3/0001-96;6B.39J.19X/0001-61;ZK.3L3.9RP/0001-09;8S.CT6.WXS/0001-04;H0.CW2.0E1/0001-17;KA.533.XVJ/0001-61;LH.2T2.KNH/0001-99;HC.PPR.JP5/0001-75;41.WZY.0TX/0001-12;66.WRD.JD2/0001-06;1K.JS6.0GL/0001-38;YA.GLX.BMH/0001-38;3M.8NR.PPE/0001-41;BE.Z1E.54Y/0001-77;KE.ZDH.MKH/0001-91;C6.KAG.PNX/0001-65;E6.54M.ME1/0001-32;WG.YRP.LYY/0001-45;0V.WY1.1B7/0001-14;BS.7RV.0TX/0001-07;SK.17R.LL7/0001-88;09.7XM.GGY/0001-03;AK.78E.D7T/0001-11;T1.992.X21/0001-59;K0.D5Z.25C/0001-98;7B.7BY.NRG/0001-42;4M.EL4.5VH/0001-55;SH.ES5.DM4/0001-90;R9.CRY.7L4/0001-17;D3.06A.D6D/0001-46;VA.EZ4.SGS/0001-68;LA.ZMB.M50/0001-26;E0.8WW.JXJ/0001-67;XB.80M.3GW/0001-14;36.VDL.ZYR/0001-29;HX.NDC.E4X/0001-44;4Y.VVY.MRT/0001-00;SE.YWW.5DS/0001-73;HK.YEV.2L8/0001-40;XB.CWB.PTK/0001-20;SC.HCS.K7C/0001-05;K8.48W.2GK/0001-44;2D.ZEB.4YP/0001-25;0Z.STM.G7D/0001-70;KG.42J.NHZ/0001-52;NS.GNJ.6WV/0001-67;NC.GK1.CLM/0001-99;GC.VYB.RZM/0001-72;1W.M0A.B4B/0001-08;JE.8WW.31A/0001-12;N1.ZYL.3ZA/0001-90;RN.WYY.4BJ/0001-33;1D.9SC.YTW/0001-89;G1.P0T.RN9/0001-10;XE.6WM.903/0001-43;3B.WVD.BD5/0001-54;5J.NY1.1A8/0001-82;B5.D4G.WA5/0001-07;47.77Y.0T2/0001-86;ZX.8ZT.4SD/0001-97;B1.HGZ.6WP/0001-61;GY.RV8.AH4/0001-05;PH.YZS.VL1/0001-14;GP.RWA.0N5/0001-73;7P.ZSJ.AD4/0001-76;KZ.3NY.ST2/0001-01;6H.X95.3TS/0001-67;Y3.N5H.GKT/0001-94;6G.NG6.2B2/0001-59;JN.0PE.57L/0001-80;CR.LD7.SEJ/0001-90;2W.3JG.WCS/0001-90;42.M3Z.LX3/0001-08;HZ.LSW.01S/0001-08;ZK.Y4N.D15/0001-68;WW.JSK.MSP/0001-62;AZ.ZEY.2GE/0001-53;M6.VDM.G9P/0001-93;P9.KGB.SBB/0001-07;GD.ZYR.DWZ/0001-19;30.3KN.ECE/0001-41;X1.A23.SS0/0001-10;81.38E.3TZ/0001-48;X5.ZDG.GL4/0001-27;PG.6VC.PK3/0001-87;YZ.W7S.HL0/0001-17;22.7P7.MCP/0001-11;81.EV7.BNS/0001-25;R7.MEV.JV7/0001-13;X4.YT4.61C/0001-93;9J.T4P.RV9/0001-84;YH.BSH.TBA/0001-74;33.LNP.0P7/0001-09;6A.WCY.7BP/0001-32;9A.X77.YYC/0001-42;5E.0NW.W5A/0001-84;6K.ETW.N1L/0001-49;D5.53R.ZZV/0001-23;38.Y0V.AH5/0001-01;H1.CMX.3W0/0001-46;ER.CCZ.5B5/0001-16;6Z.AXE.APT/0001-47;4X.0CP.DLT/0001-91;SR.X8X.1JJ/0001-00;0G.YMB.HD8/0001-85;VE.NG8.7Y9/0001-06;S4.AYG.0CR/0001-09;87.GLA.N5A/0001-10;S7.TWB.WMV/0001-20;8V.TA4.CP6/0001-86;WM.LD8.ZKW/0001-80;Y2.7AW.3E9/0001-16;C2.T56.07B/0001-63;E8.N74.L46/0001-56;G9.WBB.69T/0001-80;6R.R6N.S5H/0001-19;ER.174.NCD/0001-32;A8.M7C.CLM/0001-06;WB.26J.XVN/0001-08;RX.TRP.GG2/0001-27;9X.ZDR.41V/0001-06;N2.TED.RJA/0001-05;7A.27V.6ZW/0001-20;3V.5AG.XGN/0001-31;ZB.T7C.LV9/0001-18;93.V5R.Z2P/0001-34;V2.A1B.6PD/0001-72;9N.2C7.V7A/0001-86;SX.V54.1WW/0001-06;N8.C4B.86G/0001-50;60.5CP.7K9/0001-06;Z1.4VH.ZB8/0001-40;69.XXP.HV3/0001-26;JS.DMY.M75/0001-86;11.LNX.WSK/0001-90;0P.E4P.688/0001-84;YT.P3R.YA8/0001-04;BV.M3V.WT3/0001-47;G5.YS4.ZX2/0001-78;M2.514.8KJ/0001-05;WG.A3P.9E9/0001-68;52.CD4.N3M/0001-25;AX.TND.RA5/0001-78;EB.SZW.N6T/0001-43;82.5PL.JWT/0001-03;TY.02S.K9W/0001-01;0Y.X38.CT9/0001-33;KJ.77H.M2K/0001-27;1J.39N.KZE/0001-12;8A.ZX2.W0C/0001-01;53.DKZ.4SG/0001-00;9J.4BD.20W/0001-90;DS.797.3T9/0001-89;Y5.C8P.3W8/0001-20;TR.X9R.7Y3/0001-00;V4.D4R.L53/0001-02;G4.VDJ.K87/0001-19;XW.AVT.YTD/0001-90;K0.GG7.01D/0001-90;WL.K5L.0DZ/0001-10;JR.JL9.9BN/0001-84;G4.HGJ.N4R/0001-06;32.X5P.16M/0001-41;X6.5WM.1S8/0001-01;5R.2L7.VTA/0001-09;89.XH2.SRJ/0001-85;LT.6AT.44T/0001-82;V4.HLM.YWS/0001-04;JV.J08.ZDA/0001-07;CV.3NE.X4P/0001-94;92.660.CM5/0001-38;R6.RE9.C61/0001-61;A9.5K8.TLH/0001-07;SN.434.D1D/0001-02;KP.P13.304/0001-27;MH.9W8.E6D/0001-98;66.83M.P1W/0001-35;03.6HY.8XC/0001-64;97.D8V.W22/0001-58;82.E92.15S/0001-54;16.Y81.9W3/0001-81;M3.CWE.K92/0001-70;1T.4YC.ZX4/0001-08;GD.TMJ.JK4/0001-50;0M.GKP.D6A/0001-30;L5.VPK.BNT/0001-20;8W.G14.MNL/0001-93;93.BVJ.KBW/0001-25;WJ.N2B.7SB/0001-20;7X.4R8.G23/0001-71;A3.A35.KSX/0001-02;K5.H2S.23H/0001-86;XZ.X5P.BJK/0001-03;L4.KNL.SAG/0001-18;Y4.VB6.1EZ/0001-26;P0.KSS.7J7/0001-56;28.2RY.YA8/0001-81;PH.NZG.XBZ/0001-54;4G.K4N.1B6/0001-60;4Z.5RN.ZP2/0001-78;XE.C1M.ZKX/0001-48;6E.3YG.5P4/0001-22;LP.0ZV.ZR3/0001-06;12.LZ5.JLN/0001-27;WA.MBZ.YG0/0001-04;53.J7L.SX9/0001-40;DR.8DM.8X8/0001-27;76.WX4.D27/0001-79;PY.KHB.0H1/0001-18;DA.7TJ.34T/0001-59;05.J73.VZ8/0001-50;4W.323.M9Z/0001-58;N4.AZ4.EPL/0001-34;KE.H9H.RN5/0001-60;CC.WX1.456/0001-24;72.5SP.BCL/0001-64;1T.XAR.CTN/0001-40;PY.LJC.0VZ/0001-49;3V.ARC.AX8/0001-17;9D.CTR.AX5/0001-40;PN.GNY.PJ6/0001-08;5D.ZC1.WRK/0001-99;J9.K8C.1HH/0001-30;22.L4B.GJE/0001-44;T9.DX0.B2G/0001-60;NB.T7N.YX1/0001-21;J9.V5P.C9M/0001-00;81.RPC.A7M/0001-69;P1.ECB.Z5Y/0001-07;5Z.A8M.28M/0001-12;JY.2C7.1KW/0001-21;N5.2GX.K8S/0001-15;7X.EX6.PK2/0001-78;MN.HZ5.DML/0001-26;SX.ALM.S4Z/0001-01;ME.0VX.051/0001-72;JR.BDP.JH5/0001-04;YY.PSP.D5C/0001-07;M3.ABB.GX1/0001-23;KB.PMR.LGP/0001-87;GY.1BX.5V9/0001-57;A8.838.22B/0001-44;28.0LC.2V9/0001-93;Y8.MSN.LH5/0001-15;DR.3KM.E8D/0001-77;MZ.AJ1.S4P/0001-80;CE.E3J.1Z6/0001-30;XP.803.HCD/0001-08;CR.4AL.ZCX/0001-36;13.DVH.P3E/0001-79;4Y.M5M.KA8/0001-88;GX.LY4.KX3/0001-84;2L.P89.HD5/0001-36;LK.BMA.W1V/0001-50;94.R98.NT0/0001-19;G4.557.P0A/0001-42;RK.NM6.0R6/0001-30;EK.ER4.YHN/0001-97;ZE.TDP.K56/0001-47;J1.CG9.EZN/0001-64;TH.PM5.K69/0001-30;XY.V8N.N0K/0001-95;SR.2ZD.Z7D/0001-10;X9.HPN.815/0001-30;H8.69M.ELZ/0001-09;HH.2E3.R5V/0001-87;TS.D61.158/0001-14;PW.P9C.L3A/0001-79;Y8.C0P.953/0001-60;C9.BWM.EEG/0001-08;AD.HTH.KTT/0001-03;C1.KM3.2C3/0001-62;BK.J7N.G2S/0001-97;S0.T6T.HG1/0001-04;9R.PML.S3T/0001-51;WA.XTX.TBN/0001-80;3R.WVZ.BGJ/0001-30;B4.VAY.6EB/0001-73;TL.T13.AEZ/0001-22;4M.24E.X9R/0001-76;3G.WKH.AKB/0001-67;2S.C0Z.R11/0001-86;1M.EBC.8X2/0001-01;7G.E42.3KW/0001-64;Z9.ZBY.ZV6/0001-20;YX.VR3.PKN/0001-90;ZB.V8C.ER2/0001-94;52.S4Z.84K/0001-19;11.6TG.C79/0001-96;4A.WX4.ZVL/0001-70;JY.LY5.6M2/0001-04;JM.RPT.2JB/0001-16;G4.9HM.YCC/0001-17;T4.S58.5C7/0001-55;RD.XCM.4AK/0001-62;NY.4PG.SP3/0001-14;SD.88T.XHW/0001-00;HP.Z6V.6ZC/0001-24;JX.8G0.AX1/0001-81;3B.4HN.GVK/0001-33;EZ.0C8.52P/0001-74;03.K8A.HBR/0001-69;PZ.PM1.4YL/0001-06;PX.K3P.1V0/0001-88;WT.P8H.PCP/0001-49;ZA.PP1.YG7/0001-09;H7.TZB.CKV/0001-54;T3.7DD.PR5/0001-94;9D.Z77.BJ7/0001-61;K6.C9G.H1S/0001-60;H6.W39.RH3/0001-60;SX.CDX.GAX/0001-53;KN.R63.GL2/0001-20;XT.AKX.AS4/0001-50;0W.AH9.BW5/0001-71;DH.7BK.1JW/0001-27;6A.W0A.DN7/0001-09;E0.VZ3.HCZ/0001-46;GM.MBX.BY3/0001-17;L3.WR9.R0D/0001-21;GL.TJ4.LNE/0001-56;Z0.N30.Y06/0001-08;5W.5VW.45V/0001-42;JB.YR9.JA0/0001-00;G9.T41.VZE/0001-08;L8.YBJ.0DN/0001-40;LM.TYL.GM3/0001-60;NW.G1X.JKP/0001-30;YB.T7N.5RT/0001-19;ZE.HJ9.6GX/0001-75;B5.J18.WP9/0001-49;VR.RAZ.BGH/0001-70;1R.AM9.GY9/0001-07;G1.KMH.ALA/0001-30;DD.TBP.KAX/0001-00;0G.HKB.LMA/0001-28;MC.BDR.JY5/0001-76;B6.TLZ.3W4/0001-05;AM.67H.6YZ/0001-71;4X.70K.YVH/0001-48;V1.LG5.YC0/0001-41;18.WVK.BZ0/0001-09;NN.AET.RPC/0001-01;TD.AB5.413/0001-01;N1.1BH.46B/0001-10;96.YCA.SMC/0001-87;7H.15V.WBE/0001-43;PP.KZ7.T9M/0001-24;TT.ABL.M35/0001-99;BE.VE6.BK1/0001-16;RM.LGP.G52/0001-94;VY.17A.6ZL/0001-91;5K.XS8.NZY/0001-00;7Z.3C9.008/0001-41;WD.B42.N8H/0001-67;TS.9H6.S0X/0001-88;A6.1XT.KR8/0001-35;CS.YRA.8M7/0001-10;EX.081.YCC/0001-20;8Z.K8N.D59/0001-01;B1.LK1.59B/0001-82;2V.GJ9.8EN/0001-20;LC.N8P.80Z/0001-74;7A.L76.DYV/0001-31;MM.632.5X5/0001-51;MK.1BT.R4N/0001-27;8C.PNZ.KXB/0001-90;D6.A85.61M/0001-67;MM.6C2.JRD/0001-03;W1.SLT.TY5/0001-74;M9.PLD.28H/0001-61;TE.YBH.CZK/0001-52;VC.S87.MRN/0001-90;TG.6N9.KM2/0001-53;V0.V29.7SN/0001-83;9G.1A5.H2E/0001-86;5C.GMC.LAD/0001-58;PW.Y3G.JHK/0001-75;8G.YN6.1PW/0001-09;6H.NHR.C3Y/0001-01;LE.VNH.CTR/0001-88;2P.YSP.8CL/0001-40;T4.EMM.ZMB/0001-57;V1.YD8.Z79/0001-06;3L.LL2.TCG/0001-11;9S.0WN.VXJ/0001-10;9M.96J.AYC/0001-61;4G.Z98.C4Y/0001-39;WV.2CY.ZCL/0001-92;LG.SH1.BCG/0001-39;XD.LYP.961/0001-20;7H.4Z8.W2R/0001-58;CZ.N9S.8Y6/0001-39;WC.PR5.TH9/0001-98;LR.EAP.LD1/0001-17;R7.SW6.XA9/0001-59;BB.X5T.LV0/0001-32;T2.JGE.0WA/0001-21;M2.G2D.96P/0001-12;BE.1XM.Z5V/0001-31;AP.LVA.MPY/0001-29;ET.ANZ.BR4/0001-93;TH.7Y2.WXY/0001-60;95.C16.2EW/0001-65;HM.W4N.1TR/0001-36;RJ.T75.R1E/0001-06;48.SYE.RDK/0001-82;6J.RLW.CMZ/0001-29;6V.YH9.NHA/0001-03;K8.TLH.2R0/0001-00";

        let mut failed = Vec::new();
        for cnpj in CNPJS_RECEITA_FEDERAL.split(';') {
            if !validate(cnpj) {
                failed.push(cnpj);
            }
        }

        assert!(
            failed.is_empty(),
            "Os seguintes CNPJs da Receita Federal falharam na validação: {:?}",
            failed
        );
    }
}
