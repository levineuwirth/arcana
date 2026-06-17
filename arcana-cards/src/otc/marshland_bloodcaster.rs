//! Marshland Bloodcaster — `{4}{B}` 3/5 Vampire Warlock.
//! Flying.
//! "{1}{B}, {T}: Rather than pay the mana cost of the next spell you cast
//!  this turn, you may pay life equal to that spell's mana value."
//!
//! Flying is a base keyword. The cost-substitution activated ability has
//! no expressible effect primitive — the activation cost is modeled but
//! the payoff is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marshland Bloodcaster");
    let vampire = reg.interner_mut().intern("Vampire");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}, {T}: Rather than pay the mana cost of the next spell \
                   you cast this turn, you may pay life equal to that spell's \
                   mana value."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cost_substitution_gap,
        }),
    )
}

fn cost_substitution_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Rather than pay the mana cost of the next spell you cast this
    // turn, you may pay life equal to that spell's mana value." — a
    // next-spell cost-substitution (alternative life payment) effect is
    // not expressible with the available primitives.
    Vec::new()
}
