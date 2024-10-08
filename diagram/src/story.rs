use crate::{Action, Character, Event, Location, Movement, Scene};

pub fn get_story() -> Vec<Scene<'static>> {
    let rufus_red = Character { name: "Rufus Red" };
    let dianna_robinson = Character {
        name: "Dianna Robinson",
    };
    let judy_woolridge = Character {
        name: "Judy Woolridge",
    };
    let duncan_moss = Character {
        name: "Duncan Moss",
    };
    let rebecca_red = Character {
        name: "Rebecca Red",
    };

    let greyham_red = Character {
        name: "Greyham Red",
    };

    let greenfield = Location { name: "Greenfield" };
    let boxon = Location { name: "Boxon" };

    let rebeccas_appartment = Location {
        name: "Rebecca's Appartment",
    };
    let restaurant = Location { name: "Restaurant" };
    let hotel = Location { name: "Hotel" };
    let other = Location { name: "Other" };

    let yard = Location { name: "Yard" };
    let living_room = Location {
        name: "Living Room",
    };
    let garage = Location { name: "Garage" };
    let basement = Location { name: "Basement" };
    let attic = Location { name: "Attic" };

    return vec![
        Scene {
            locations: vec![boxon, greenfield],
            name: "Past Years",
            characters: vec![
                rebecca_red,
                dianna_robinson,
                rufus_red,
                judy_woolridge,
                duncan_moss,
                greyham_red,
            ],
            events: vec![
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rufus_red, judy_woolridge, rebecca_red],
                        to: Some(boxon),
                    }],
                    action: Some(Action {
                        characters: vec![rufus_red, rebecca_red, judy_woolridge],
                        name: "Go to school in Boxon",
                    }),
                },
                Event {
                    time: Some("2018"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![judy_woolridge, rebecca_red],
                        name: "Graduates High School",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red],
                        to: Some(greenfield),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red],
                        name: "Rebeca goes to the university in Greenfield",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red, dianna_robinson],
                        to: Some(greenfield),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, dianna_robinson],
                        name: "Rebeca and Dianna Meet for the first time",
                    }),
                },
                Event {
                    time: Some("2020"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rufus_red],
                        name: "Rufus graduates starts working at Red Bakery",
                    }),
                },
                Event {
                    time: Some("2023"),
                    movement: vec![Movement {
                        characters: vec![greyham_red],
                        to: Some(boxon),
                    }],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy starts working at Red Bakery",
                    }),
                },
                Event {
                    time: Some("Aug 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, dianna_robinson],
                        name: "Rebeca and Dianna develop the advanced material science degree",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rufus_red, judy_woolridge],
                        name: "Judy and Rufus start dating",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(greenfield),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan start dating",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rufus_red, judy_woolridge],
                        name: "Judy and Rufus break up",
                    }),
                },
                Event {
                    time: Some("Nov 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red],
                        name: "Investors contact Rebecca about funding the research",
                    }),
                },
                Event {
                    time: Some("Fri, Dec 22 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![dianna_robinson],
                        name: "Dianna gets expelled for plagarism",
                    }),
                },
                Event {
                    time: Some("Thu, 4 Jan 2024"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![greyham_red],
                        name: "Greyham Red Dies",
                    }),
                },
                Event {
                    time: Some("Mon Jan 2024"),
                    movement: vec![
                        Movement {
                            characters: vec![rebecca_red, duncan_moss],
                            to: Some(boxon),
                        },
                        Movement {
                            characters: vec![greyham_red],
                            to: None,
                        },
                    ],
                    action: Some(Action {
                        characters: vec![rebecca_red, rufus_red, judy_woolridge, duncan_moss],
                        name: "Greyham Red's Funeral",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red, duncan_moss],
                        to: Some(greenfield),
                    }],
                    action: None,
                },
                Event {
                    time: Some("Fri, 12 Jan 2024"),
                    movement: vec![Movement {
                        characters: vec![rufus_red, judy_woolridge],
                        to: Some(greenfield),
                    }],
                    action: None,
                },
            ],
        },
        Scene {
            characters: vec![
                rebecca_red,
                dianna_robinson,
                rufus_red,
                judy_woolridge,
                duncan_moss,
            ],
            locations: vec![rebeccas_appartment, restaurant, hotel, other],
            name: "Past Days",
            events: vec![
                Event {
                    time: Some("Fri, 12 Jan 2024 15:00"),
                    movement: vec![Movement {
                        characters: vec![
                            rebecca_red,
                            judy_woolridge,
                            duncan_moss,
                            rufus_red,
                            dianna_robinson,
                        ],
                        to: Some(rebeccas_appartment),
                    }],
                    action: None,
                },
                Event {
                    time: Some("Fri, 12 Jan 2024, 22:00"),
                    movement: vec![
                        Movement {
                            characters: vec![rufus_red],
                            to: Some(hotel),
                        },
                        Movement {
                            characters: vec![dianna_robinson],
                            to: Some(other),
                        },
                    ],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![
                            rebecca_red,
                            dianna_robinson,
                            rufus_red,
                            judy_woolridge,
                            duncan_moss,
                        ],
                        name: "Sleeping arangement friday night",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 11:00"),
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(other),
                    }],
                    action: Some(Action {
                        characters: vec![duncan_moss],
                        name: "Duncan Moss Goes to Work",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:00"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red, judy_woolridge, rufus_red, dianna_robinson],
                        to: Some(restaurant),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, rufus_red],
                        name: "Rufus and Rebecca start arguing",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy joins the argument",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:30"),
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(restaurant),
                    }],
                    action: Some(Action {
                        characters: vec![duncan_moss],
                        name: "Duncan Moss arrives",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:45"),
                    movement: vec![Movement {
                        characters: vec![judy_woolridge, rufus_red],
                        to: Some(hotel),
                    }],
                    action: Some(Action {
                        characters: vec![judy_woolridge, rufus_red],
                        name: "Judy drags Rufus away",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![dianna_robinson],
                        to: Some(other),
                    }],
                    action: Some(Action {
                        characters: vec![dianna_robinson],
                        name: "Dianna also leaves",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan eat dinner",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 19:30"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red, duncan_moss],
                        to: Some(rebeccas_appartment),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan go home",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: None,
                },
            ],
        },
        Scene {
            locations: vec![yard, living_room, garage, basement, attic],
            characters: vec![
                rebecca_red,
                dianna_robinson,
                rufus_red,
                judy_woolridge,
                duncan_moss,
            ],

            name: "Past Hours",
            events: vec![
                Event {
                    time: Some("Sun, 14 Jan 2024, 10:30"),
                    movement: vec![Movement {
                        characters: vec![dianna_robinson],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![duncan_moss, rebecca_red],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![judy_woolridge, rufus_red],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: Some("Sun, 14 Jan 2024, 12:45"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![
                            rebecca_red,
                            dianna_robinson,
                            rufus_red,
                            judy_woolridge,
                            duncan_moss,
                        ],
                        name: "Finish unloading truck",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy brings the truck back to the rental",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![
                        Movement {
                            characters: vec![judy_woolridge],
                            to: None,
                        },
                        Movement {
                            characters: vec![rufus_red],
                            to: Some(basement),
                        },
                        Movement {
                            characters: vec![dianna_robinson],
                            to: Some(garage),
                        },
                        Movement {
                            characters: vec![rebecca_red],
                            to: Some(attic),
                        },
                    ],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![duncan_moss],
                        name: "Duncan goes to the store to buy toilet paper",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![
                        Movement {
                            characters: vec![judy_woolridge],
                            to: Some(yard),
                        },
                        Movement {
                            characters: vec![duncan_moss],
                            to: None,
                        },
                    ],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy comes back",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![judy_woolridge],
                        to: Some(living_room),
                    }],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy starts unpacking in the living room",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rufus_red],
                        to: Some(living_room),
                    }],
                    action: Some(Action {
                        characters: vec![rufus_red, judy_woolridge],
                        name: "Rufus and Judy talk",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rufus_red],
                        to: Some(basement),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![dianna_robinson],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![judy_woolridge],
                        to: Some(attic),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![judy_woolridge, rebecca_red],
                        name: "Judy kills Rebecca",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red],
                        to: None,
                    }],
                    action: Some(Action {
                        characters: vec![dianna_robinson],
                        name: "Dianna finds the body",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rufus_red, judy_woolridge],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(yard),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![duncan_moss],
                        name: "Duncan calls an ambulance",
                    }),
                },
            ],
        },
    ];
}
