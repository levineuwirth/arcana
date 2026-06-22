//! Grusilda, Monster Masher — `{3}{B}{R}` 4/4 Legendary Creature — Zombie
//! Villain.
//! Combined, enchanted, and equipped creatures you control have menace.
//! {3}{B}{R}, {T}: Put two target creature cards from graveyards onto the
//! battlefield combined into one creature under your control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grusilda, Monster Masher");
    let zombie = reg.interner_mut().intern("Zombie");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Combined, enchanted, and equipped creatures you control have
    // menace." is a static continuous anthem with no trigger word and no
    // activation cost; not expressible in this shape.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{B}{R}, {T}: Put two target creature cards from graveyards onto the battlefield combined into one creature under your control."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{B}{R}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(2),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: combine_creatures,
        }),
    )
}

fn combine_creatures(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "combined into one creature" (the meld of two graveyard creatures
    // into a single object whose P/T, names, types, and text are the union of
    // both) has no expressible engine primitive. Returning them separately
    // would be a materially different card, so the whole effect is GAP'd.
    Vec::new()
}
