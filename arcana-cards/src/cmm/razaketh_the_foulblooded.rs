//! Razaketh, the Foulblooded — `{5}{B}{B}{B}` 8/8 legendary Demon with
//! Flying, Trample. "Pay 2 life, Sacrifice another creature: Search
//! your library for a card, put that card into your hand, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Razaketh, the Foulblooded");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![arcana_core::effects::KeywordAbility::Flying, arcana_core::effects::KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Pay 2 life, Sacrifice another creature: Search your library for a card, put that card into your hand, then shuffle.".into(),
            cost: ActivationCost {
                life: 2,
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_any,
        }),
    )
}

fn tutor_any(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::default(),
        reveal: false,
    }]
}
