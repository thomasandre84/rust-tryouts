#[derive(Debug)]
pub enum Media {
     Book{ title: String },
     Movie{ title: String },
     Music{ title: String },
 }

 impl Media {
     pub fn new_book(title: String) -> Self {
         Media::Book{ title }
     }

     pub fn new_movie(title: String) -> Self {
         Media::Movie{ title }
     }

     pub fn new_music(title: String) -> Self {
         Media::Music{ title }
     }

     pub fn title(&self) -> &str {
         match self {
             Media::Book{ title } => title,
             Media::Movie{ title } => title,
             Media::Music{ title } => title,
         }
     }
 }