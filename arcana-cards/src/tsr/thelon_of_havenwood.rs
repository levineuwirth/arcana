//! Thelon of Havenwood — `{G}{G}` 2/2 Legendary Elf Druid.
//! "Each Fungus creature gets +1/+1 for each spore counter on it."
//! "{B}{G}, Exile a Fungus card from a graveyard: Put a spore counter on each
//! Fungus on the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thelon of Havenwood");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    // pre-intern the spore counter name and the Fungus subtype
    let _spore = reg.interner_mut().intern("spore");
    let _fungus = reg.interner_mut().intern("Fungus");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP (cost): "Exile a Fungus card from a graveyard" is an exile-from-
            // graveyard cost with no ActivationCost field; only the {B}{G} mana
            // component is modeled.
            text: "{B}{G}, Exile a Fungus card from a graveyard: Put a spore counter on each Fungus on the battlefield."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: spore_each_fungus,
        }),
    )
    // GAP: static "Each Fungus creature gets +1/+1 for each spore counter on it"
    // is a dynamic continuous anthem static ability, not a triggered/activated
    // ability — not expressible with the demonstrated MultiAbilityCreature API.
}

fn spore_each_fungus(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let spore = match reg.interner().lookup("spore").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    let filter = script::subtype_filter(reg, "Fungus");
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: spore,
            count: 1,
        })
        .collect()
}
