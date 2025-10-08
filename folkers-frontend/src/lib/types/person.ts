interface SurrealThing {
  tb: string;
  id: {
    String: string;
  };
}

export interface PersonRecord {
  id: SurrealThing;
  name: string;
  surname: string;
  patronymic: string;

  birthday: string;
  city: string;
  intented_address: string;

  summary: string;
  past: string;
  traits_good: string;
  traits_bad: string;

  avatar: string | null;
  media: string[];

  author: SurrealThing;
}
