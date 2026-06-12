//! Cylian Sunsinger — `{1}{G}` 2/2 green Elf Shaman. "{R}{G}{W}: This creature
//! and each other creature with the same name as it get +3/+3 until end of turn."
//!
//! "This creature and each other creature with the same name" is modeled as a
//! name-filtered global pump until end of turn — the source matches its own
//! name filter, so one filtered effect covers both halves. The filter matches
//! PRINTED names (copies named Cylian Sunsinger are caught).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cylian Sunsinger");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{G}{W}: This creature and each other creature with the same name as it get +3/+3 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{G}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self_and_namesakes,
            }),
    )
}

fn pump_self_and_namesakes(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Name-filtered global pump: the source matches its own name filter,
    // so this covers "this creature AND each other creature with the same
    // name" (any controller) in one effect.
    let name = reg
        .interner()
        .lookup("Cylian Sunsinger")
        .expect("name interned during register()");
    let mut namesakes = ObjectFilter::creature();
    namesakes.name = Some(name);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            ctx.source,
            namesakes,
            3,
            3,
            Duration::EndOfTurn,
        ),
    }]
}
