//! Deity of Scars — `{B/G}{B/G}{B/G}{B/G}{B/G}` 7/7 Spirit Avatar.
//! Trample. Enters with two -1/-1 counters.
//! `{B/G}, Remove a -1/-1 counter from this creature: Regenerate this
//! creature.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deity of Scars");
    let spirit = reg.interner_mut().intern("Spirit");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/G}{B/G}{B/G}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::MinusOneMinusOne,
                count: 2,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B/G}, Remove a -1/-1 counter from this creature: Regenerate this creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B/G}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::MinusOneMinusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_self,
            }),
    )
}

fn regenerate_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
