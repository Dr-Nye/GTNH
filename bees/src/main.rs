fn main() {
    let mut owned = vec![
        "Common",
        "Ender",
        "Majestic",
        "Noble",
        "Cultivated",
        "Modest",
        "Meadows",
        "Water",
        "Wintry",
        "Valiant",
        "Sinister",
        "Imperial",
        "Mystical",
        "Infernal",
        "Attuned",
        "Rocky",
        "Steadfast",
        "Monastic",
        "Embittered",
    ];

    let mut recipees = vec![
        ["Eldritch", "Mystical", "Cultivated"],
        ["Charmed", "Cultivated", "Eldritch"],
        ["Enchanted", "Eldritch", "Charmed"],
        ["Esoteric", "Cultivated", "Eldritch"],
        ["Mysterious", "Eldritch", "Esoteric"],
        ["Supernatural", "Charmed", "Enchanted"],
        ["Arcane", "Esoteric", "Mysterious"],
        ["Ethereal", "Arcane", "Supernatural"],
        ["Vis", "Infernal", "Ethereal"],
        ["Rejuvenating", "Attuned", "Vis"],
        ["Empowering", "Rejuvenating", "Vis"],
        ["Nexus", "Rejuvenating", "Empowering"],
        ["Fiendish", "Sinister", "Cultivated"],
        ["Frugal", "Modest", "Sinister"],
        ["Austere", "Modest", "Frugal"],
        ["Diligent", "Common", "Cultivated"],
        ["Tolerant", "Rocky", "Diligent"],
        ["Unweary", "Diligent", "Cultivated"],
        ["Robust", "Unweary", "Tolerant"],
        ["Secluded", "Monastic", "Austere"],
        ["Demonic", "Fiendish", "Sinister"],
        ["Industrious", "Unweary", "Diligent"],
        ["Clay", "Industrious", "Diligent"],
        ["Tin", "Clay", "Diligent"],
        ["Copper", "Clay", "Majestic"],
        ["Iron", "Copper", "Tin"],
        ["Nickel", "Copper", "Iron"],
        ["Zinc", "Iron", "Tin"],
        ["Resilient", "Industrious", "Robust"],
        ["Redstone", "Industrious", "Demonic"],
        ["Lapis", "Demonic", "Imperial"],
        ["Hermitic", "Monastic", "Secluded"],
        ["Aluminium", "Nickel", "Zinc"],
        ["Titanium", "Redstone", "Aluminium"],
        ["Vindictive", "Monastic", "Demonic"],
        ["Vengeful", "Demonic", "Vindictive"],
        ["Rural", "Meadows", "Diligent"],
        ["Peat", "Rural", "Clay"],
        ["Coal", "Industrious", "Peat"],
        ["Certus", "Hermitic", "Lapis"],
        ["Manganese", "Aluminium", "Titanium"],
        ["Heroic", "Steadfast", "Valiant"],
        ["Avenging", "Vengeful", "Vindictive"],
        ["Diamond", "Certus", "Coal"],
        ["Ruby", "Redstone", "Diamond"],
        ["Olivine", "Certus", "Ender"],
        ["Chrome", "Titanium", "Ruby"],
        ["Platinum", "Diamond", "Chrome"],
        ["Tungsten", "Heroic", "Manganese"],
        ["Uranium", "Avenging", "Platinum"],
        ["Emerald", "Olivine", "Diamond"],
        ["Iridium", "Tungsten", "Platinum"],
        ["Plutonium", "Uranium", "Emerald"],
        ["Naquadah", "Plutonium", "Iridium"],
        ["Cryotheum", "Redstone", "Coolant"],
        ["Coolant", "Icy", "Glacial"],
        ["Icy", "Industrious", "Wintry"],
        ["Glacial", "Icy", "Wintry"],
        ["Pyrotheum", "Redstone", "Energy"],
        ["Energy", "Demonic", "Volcanic"],
        ["Volcanic", "Demonic", "Furious"],
        ["Furious", "Fiendish", "Embittered"],
        ["Chad", "Cultivated", "Celebratory"],
        ["Celebratory", "Austere", "Excited"],
        ["Excited", "Valiant", "Cultivated"],
        ["Osmium", "Tungsten", "Platinum"],
    ];

    //remove recipes for already owned bees
    recipees.retain(|dupes| {
        if owned.contains(&dupes[0]) {
            false
        } else {
            true
        }
    });

    for k in 1..100 {
        if recipees.is_empty() {
            break;
        };
        println!("Tier {k}: ");
        let mut tier: Vec<&str> = vec![];

        //Check layer

        recipees.retain(|recipe| {
            if owned.contains(&recipe[1]) && owned.contains(&recipe[2]) {
                tier.push(recipe[0]);
                false
            } else {
                true
            }
        });

        //Print
        for beee in tier {
            println!("{beee}");
            owned.push(beee);
        }
    }

    println!("Uncraftable:");
    for recipe in recipees {
        println!();

        let p1 = !owned.contains(&recipe[1]);
        let p2 = !owned.contains(&recipe[2]);
        let both = p1 && p2;
        print!("{} is missing: ", { recipe[0] });
        if p1 {
            print!("{}", recipe[1]);
        }
        if both {
            print!(" and ");
        }
        if p2 {
            print!("{}", recipe[2]);
        }
    }
}
