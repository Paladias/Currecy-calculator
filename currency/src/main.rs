use std::{io, convert};

fn euro(){
    let stdin = io::stdin();
     
    println!("What currency do you want to convert to?");
    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

            if crypto == 1{

            println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut ebtc: f32 = amount * 0.000032;
                println!("{}€ are {}BTC", amount, ebtc);
            }else if ctc == 2 {
                let mut eeth: f32 = amount * 0.00043;
                println!("{}€ are {}ETH", amount, eeth);
            }else if ctc == 3 {
                let mut eusdt: f32 = amount * 1.05;
                println!("{}€ are {}USDT", amount, eusdt);
            }else {
                println!("False Input");  
        }
            }else if crypto == 2{
            
            println!("\n 1. Yen ¥ \n 2. Dollar $ \n 3. Rubel ₽ \n 4. Lira ₺ \n 5. Pound £");
             let mut convert = String::new();
                stdin.read_line(&mut convert).expect("Error");
                    let mut convert: u32 = convert.trim().parse().unwrap();

                        println!("Please enter an amount you want to convert");

                let mut money = String::new();
                stdin.read_line(&mut money).expect("Faild to read line");
                let mut money: f32 = money.trim().parse().unwrap();
    
            if convert == 1{
                let mut ey:f32 = money * 137.72;
                println!("{}€ are {}¥", money, ey);
            }else if  convert == 2{
                let mut ed:f32 = money * 1.05;
                println!("{}€ are {}$", money, ed);
            }else if convert == 3{
                let mut er:f32 = money * 70.09;
                println!("{}€ are {}₽", money, er);
            }else if convert == 4{
                let mut el:f32 = money * 15.75;
                println!("{}€ are {}₺", money, el);
            }else if convert == 5{
                let mut ep: f32	= money * 0.85;
                println!("{}€ are {}£", money,ep);
            }
        }  
   }

fn yen(){
    let stdin = io::stdin();
    println!("What currency do you want to convert to?");
    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

    if crypto == 1{
        println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut ybtc: f32 = amount / 4405407.0;
                println!("{}¥ are {}BTC", amount, ybtc);
            }else if ctc == 2 {
                let mut yeth: f32 = amount * 0.0000031;
                println!("{}¥ are {}ETH", amount, yeth);
            }else if ctc == 3 {
                let mut yusdt: f32 = amount * 0.0076;
                println!("{}¥ are {}USDT", amount, yusdt);
            }else {
                println!("False Input");  
        }


    }else if crypto == 2{

    println!("\n 1. Euro € \n 2. Dollar $ \n 3. Rubel ₽ \n 4. Lira ₺ \n 5. Pound £");
    let mut convert = String::new();
        stdin.read_line(&mut convert).expect("Error");
            let mut convert: u32 = convert.trim().parse().unwrap();

    println!("Please enter an amount you want to convert");

    let mut money = String::new();
        stdin.read_line(&mut money).expect("Faild to read line");
            let mut money: f32 = money.trim().parse().unwrap();

            if convert == 1{
                let mut ye:f32 = money * 0.0073;
                println!("{}¥ are {}€", money, ye);
            }else if  convert == 2{
                let mut yd:f32 = money * 0.0077;
                println!("{}¥ are {}$", money, yd);
            }else if convert == 3{
                let mut yr:f32 = money * 0.51;
                println!("{}¥ are {}₽", money, yr);
            }else if convert == 4{
                let mut yl:f32 = money * 0.11;
                println!("{}¥ are {}₺", money, yl);
            }else if convert == 5{
                let mut yp: f32	= money * 0.0062;
                println!("{}¥ are {}£", money,yp);
            }
}
}   
fn dollar(){
    let stdin = io::stdin();

    println!("What currency do you want to convert to?");
    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

    if crypto == 1{
        println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut dbtc: f32 = amount * 0.000030;
                println!("{}$ are {}BTC", amount, dbtc);
            }else if ctc == 2 {
                let mut deth: f32 = amount * 0.00041;
                println!("{}$ are {}ETH", amount, deth);
            }else if ctc == 3 {
                let mut dusdt: f32 = amount * 1.0;
                println!("{}$ are {}USDT", amount, dusdt);
            }else {
                println!("False Input");  
        }
    }
    println!("\n 1. Euro € \n 2. Yen ¥ \n 3. Rubel ₽ \n 4. Lira ₺ \n 5. Pound £");
    let mut convert = String::new();
        stdin.read_line(&mut convert).expect("Error");
            let mut convert: u32 = convert.trim().parse().unwrap();

    println!("Please enter an amount you want to convert");

    let mut money = String::new();
        stdin.read_line(&mut money).expect("Faild to read line");
            let mut money: f32 = money.trim().parse().unwrap();

            if convert == 1{
                let mut de:f32 = money * 0.95;
                println!("{}$ are {}€", money, de);
            }else if  convert == 2{
                let mut dy:f32 = money * 130.57 ;
                println!("{}$ are {}¥", money, dy);
            }else if convert == 3{
                let mut dr:f32 = money * 66.50;
                println!("{}$ are {}₽", money, dr);
            }else if convert == 4{
                let mut dl:f32 = money * 14.95;
                println!("{}$ are {}₺", money, dl);
            }else if convert == 5{
                let mut dp: f32	= money * 0.81;
                println!("{}$ are {}£", money,dp);
            }
}

fn rubel(){
    let stdin = io::stdin();

    println!("What currency do you want to convert to?");
    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

    if crypto == 1{
        println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut rbtc: f32 = amount / 2358521.93;
                println!("{}₽ are {}BTC", amount, rbtc);
            }else if ctc == 2 {
                let mut reth: f32 = amount / 172382.51;
                println!("{}₽ are {}ETH", amount, reth);
            }else if ctc == 3 {
                let mut rusdt: f32 = amount * 0.014;
                println!("{}₽ are {}USDT", amount, rusdt);
            }else {
                println!("False Input");  
        }
    }

    println!("\n 1. Euro € \n 2. Yen ¥ \n 3. Dollar $ \n 4. Lira ₺ \n 5. Pound £");
    let mut convert = String::new();
        stdin.read_line(&mut convert).expect("Error");
            let mut convert: u32 = convert.trim().parse().unwrap();

    println!("Please enter an amount you want to convert");
    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

    if crypto == 1{
        println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut rbtc: f32 = amount / 2358521.93;
                println!("{}₽ are {}BTC", amount, rbtc);
            }else if ctc == 2 {
                let mut reth: f32 = amount / 172382.51;
                println!("{}₽ are {}ETH", amount, reth);
            }else if ctc == 3 {
                let mut rusdt: f32 = amount * 0.014;
                println!("{}₽ are {}USDT", amount, rusdt);
            }else {
                println!("False Input");  
        }
    }

    let mut money = String::new();
        stdin.read_line(&mut money).expect("Faild to read line");
            let mut money: f32 = money.trim().parse().unwrap();

            if convert == 1{
                let mut re:f32 = money * 0.014;
                println!("{}₽ are {}€", money, re);
            }else if  convert == 2{
                let mut ry:f32 = money * 1.96;
                println!("{}₽ are {}¥", money, ry);
            }else if convert == 3{
                let mut rd:f32 = money * 0.015;
                println!("{}₽ are {}$", money, rd);
            }else if convert == 4{
                let mut rl:f32 = money * 0.22;
                println!("{}₽ are {}₺", money, rl);
            }else if convert == 5{
                let mut rp: f32	= money * 0.012;
                println!("{}₽ are {}£", money,rp);
            }
}

fn lira(){
    let stdin = io::stdin();

    println!("What currency do you want to convert to?");

    println!("\n 1. Cryptocurrency \n 2. traditional currency");
    let mut crypto = String::new();
        stdin.read_line(&mut crypto).expect("Failed to read line");
            let mut crypto: u32 = crypto.trim().parse().unwrap();

    if crypto == 1{
        println!("\n 1. Bitcoin BTC \n 2. Etherium ETH \n 3. Tether USDT");

            let mut ctc = String::new();
                stdin.read_line(&mut ctc).expect("Failed to read line");
                    let mut ctc: u32 = ctc.trim().parse().unwrap();

            println!("Please enter the amount you want to convert");    
                let mut amount = String::new();
                    stdin.read_line(&mut amount).expect("Failed to read line");
                        let mut amount: f32 = amount.trim().parse().unwrap();       

            if ctc == 1 {
                let mut rbtc: f32 = amount / 2358521.93;
                println!("{}₽ are {}BTC", amount, rbtc);
            }else if ctc == 2 {
                let mut reth: f32 = amount / 172382.51;
                println!("{}₽ are {}ETH", amount, reth);
            }else if ctc == 3 {
                let mut rusdt: f32 = amount * 0.014;
                println!("{}₽ are {}USDT", amount, rusdt);
            }else {
                println!("False Input");  
        }
    }

    println!("\n 1. Euro € \n 2. Yen ¥ \n 3. Dollar $ \n 4. Rubel ₽ \n 5. Pound £");
    let mut convert = String::new();
        stdin.read_line(&mut convert).expect("Error");
            let mut convert: u32 = convert.trim().parse().unwrap();

    println!("Please enter an amount you want to convert");

    let mut money = String::new();
        stdin.read_line(&mut money).expect("Faild to read line");
            let mut money: f32 = money.trim().parse().unwrap();

            if convert == 1{
                let mut le:f32 = money * 0.063;
                println!("{}₺ are {}€", money, le);
            }else if  convert == 2{
                let mut ly:f32 = money * 8.74;
                println!("{}₺ are {}¥", money, ly);
            }else if convert == 3{
                let mut ld:f32 = money * 0.067;
                println!("{}₺ are {}$", money, ld);
            }else if convert == 4{
                let mut lr:f32 = money * 4.48;
                println!("{}₺ are {}₽", money, lr);
            }else if convert == 5{
                let mut lp: f32	= money * 0.054;
                println!("{}₺ are {}£", money,lp);
            }
}

fn pound(){
    let stdin = io::stdin();

    println!("What currency do you want to convert to?");

    println!("\n 1. Euro € \n 2. Yen ¥ \n 3. Dollar $ \n 4. Rubel ₽ \n 5. Lira ₺");
    let mut convert = String::new();
        stdin.read_line(&mut convert).expect("Error");
            let mut convert: u32 = convert.trim().parse().unwrap();

    println!("Please enter an amount you want to convert");

    let mut money = String::new();
        stdin.read_line(&mut money).expect("Faild to read line");
            let mut money: f32 = money.trim().parse().unwrap();

            if convert == 1{
                let mut pe:f32 = money * 1.17;
                println!("{}£ are {}€", money, pe);
            }else if  convert == 2{
                let mut py:f32 = money * 160.99;
                println!("{}£ are {}¥", money, py);
            }else if convert == 3{
                let mut pd:f32 = money * 1.23;
                println!("{}£ are {}$", money, pd);
            }else if convert == 4{
                let mut pr:f32 = money * 82.06;
                println!("{}£ are {}₽", money, pr);
            }else if convert == 5{
                let mut pl: f32	= money * 18.44;
                println!("{}£ are {}₺", money,pl);
            }
}   

fn main() {
   
    let stdin = io::stdin();

    loop {
        
       
        println!("Currency calculator!");

        println!("Pick a currency!");
        println!(" \n 1. Euro € \n 2. Yen ¥ \n 3. Dollar $ \n 4. Rubel ₽ \n 5. Lira ₺ \n 6. Pound £");

        let mut currency = String::new();
        stdin.read_line(&mut currency) .expect("Error");
            let mut currency: u32 = currency.trim().parse().unwrap();

            if currency == 1 {
                euro();
                    break;
            }else if currency == 2 {
                yen();
                    break;
            }else if currency == 3{
                dollar();
                    break;
            }else if currency == 4 {
                rubel();
                    break;
            }else if currency == 5 {
                lira();
                    break;
            }else if currency == 6 {
                pound();
                    break;
            }else {
                println!("Invalid Input");
            }
    
   
    }
}
