//! Radioactive Spider — `{G}` 1/1 Spider with Reach and Deathtouch.
//!
//! * Reach, deathtouch (keywords).
//! * Fateful Bite — `{2}, Sacrifice this creature: Search your library
//!   for a Spider Hero card, reveal it, put it into your hand, then
//!   shuffle. Activate only as a sorcery.`

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Radioactive Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice this creature: Search your library for a Spider Hero card, reveal it, put it into your hand, then shuffle. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_spider_hero,
            }),
    )
}

fn tutor_spider_hero(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::creature();
    if let Some(spider) = reg.interner().lookup("Spider") {
        filter = filter.with_subtype_sym(spider);
    }
    if let Some(hero) = reg.interner().lookup("Hero") {
        filter = filter.with_subtype_sym(hero);
    }
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter,
        reveal: true,
    }]
}
