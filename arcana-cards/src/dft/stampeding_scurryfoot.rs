//! Stampeding Scurryfoot — `{G}` 1/1 green Mouse.
//! Exhaust — `{3}{G}`: Put a +1/+1 counter on this creature. Create a 3/3
//! green Elephant creature token. (Activate each exhaust ability only once.)
//!
//! GAP: `Exhaust` is not a keyword in the engine keyword surface; omitted from
//! `keywords`. The "activate only once" restriction is not modeled by the
//! activation cost struct (no `exhaust` field); the ability is emitted as a
//! regular activated ability.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stampeding Scurryfoot");
    let mouse = reg.interner_mut().intern("Mouse");
    let _elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: Put a +1/+1 counter on this creature. Create a 3/3 green Elephant creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_counter_and_elephant,
            }),
    )
}

fn exhaust_counter_and_elephant(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elephant = reg.interner().lookup("Elephant")
        .expect("Elephant interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elephant);
    let token = TokenDefinition {
        name: elephant,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: arcana_core::types::CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
    ]
}
