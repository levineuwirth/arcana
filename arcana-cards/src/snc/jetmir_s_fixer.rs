//! Jetmir's Fixer — `{R}{G}` 2/2 red/green Cat Warrior. "{R}{G}: This creature
//! gets +1/+1 until end of turn. If mana from a Treasure was spent to activate
//! this ability, put a +1/+1 counter on this creature instead."
//!
//! GAP: "if mana from a Treasure was spent" conditional check on mana source
//! not expressible. Emitting the +1/+1 pump only.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jetmir's Fixer");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{G}: This creature gets +1/+1 until end of turn. If mana from a Treasure was spent, put a +1/+1 counter instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if mana from a Treasure was spent" conditional not expressible.
    vec![Effect::Pump { target: ctx.source, power: 1, toughness: 1, duration: Duration::EndOfTurn, keywords: vec![] }]
}
