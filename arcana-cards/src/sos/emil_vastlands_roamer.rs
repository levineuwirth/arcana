//! Emil, Vastlands Roamer — `{2}{G}` 3/3 Legendary Elf Druid.
//! Static: creatures you control with +1/+1 counters have trample. (GAP)
//! {4}{G}, {T}: Create a 0/0 green-and-blue Fractal token, put X +1/+1
//! counters on it (X = differently named lands you control).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emil, Vastlands Roamer");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    // Pre-intern the token subtype.
    let _fractal = reg.interner_mut().intern("Fractal");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Creatures you control with +1/+1 counters on them have
    // trample" is a pure continuous static — not a triggered/activated
    // ability, not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}, {T}: Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it, where X is the number of differently named lands you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_fractal,
        }),
    )
}

fn make_fractal(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fractal = reg.interner().lookup("Fractal").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    // GAP: the X +1/+1 counters (X = number of differently named lands you
    // control) — no script helper counts distinct land names, and a freshly
    // created token has no readable ObjectId to target with AddCounters.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: fractal,
            colors: ColorSet::green() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
