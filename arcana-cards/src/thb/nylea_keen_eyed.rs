//! Nylea, Keen-Eyed — `{3}{G}` 5/6 Legendary Enchantment Creature — God
//! (green) with Indestructible.
//!
//! Oracle:
//! * Indestructible — keyword.
//! * "As long as your devotion to green is less than five, Nylea isn't a
//!   creature." — a STATIC characteristic-defining/continuous ability (no
//!   trigger, no cost). GAP'd: there is no expressible self-conditional
//!   "isn't a creature" continuous effect on this card class.
//! * "Creature spells you cast cost {1} less to cast." — a STATIC cost
//!   reduction. GAP'd: no cost-reduction effect/field is exposed here.
//! * "{2}{G}: Reveal the top card of your library. If it's a creature card,
//!   put it into your hand. Otherwise, you may put it into your graveyard."
//!   — modeled as `DigTopN` looking at the top 1 card, optionally taking a
//!   creature card to hand, with the remainder routed to the graveyard.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nylea, Keen-Eyed");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Reveal the top card of your library. If it's a creature card, put it into your hand. Otherwise, you may put it into your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: dig_for_creature,
            }),
    )
}

fn dig_for_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::Graveyard,
    }]
}
