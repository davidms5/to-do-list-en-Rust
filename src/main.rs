use std::collections::HashMap;


fn main() {
    let action = std::env::args().nth(1).expect("especifique una accion");
    let item = std::env::args().nth(2).expect("especifique un item");

    println!("{:?}, {:?}", action, item);

    let mut todo = Hacer::new().expect("inicializacion de db falló");

    if action == "add" || action == "agregar" {
        todo.insertar(item);
        match todo.guardar(){
            Ok(_) => println!("guardado"),
            Err(why) => println!("ocurrio un error: {}", why),
        }
    } else if action == "completo" || action == "complete"{
        match todo.completo(&item){
            None => println!("'{}' no esta presente en la lista", item),
            Some(_) => match todo.guardar(){
                Ok(_) => println!("tarea guardada"),
                Err(why) => println!("ocurrio un error: {}", why),
            },
        }
    }
}

struct Hacer {
    map: HashMap<String, bool>,
}

impl Hacer {
    fn new() -> Result<Hacer, std::io::Error>{
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        match serde_json::from_reader(f){
            Ok(map) => Ok(Hacer{map}),
            Err(e) if e.is_eof()=> Ok(Hacer {
                map: HashMap::new(),
            }),
            Err(e) => panic!("ocurrio un error: {}", e),
        }
      //  let mut contenido = String::new();
        //f.read_to_string(&mut contenido)?;
        //let map: HashMap<String, bool> = contenido
          //  .lines()
          //  .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
          //  .map(|v| (v[0], v[1]))
          //  .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
          //  .collect();
      //  Ok(Hacer{ map })
    }


    fn insertar(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
//en la funcion de abajo, una vez invocado, no se podra volver a
//usar, debido al borrowing de "self"
    fn guardar(self) -> Result<(), Box<dyn std::error::Error>>{
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    fn completo(&mut self, key: &String) -> Option<()>{
        match self.map.get_mut(key){
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
