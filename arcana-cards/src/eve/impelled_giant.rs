//! Impelled Giant — `{4}{R}{R}` 3/3 Giant Warrior.
//!
//! Oracle:
//! * Trample  (keyword line)
//! * Tap an untapped red creature you control other than this creature: This
//!   creature gets +X/+0 until end of turn, where X is the power of the
//!   creature tapped this way.  (PARTIAL — the `tap_other` cost is wired (the
//!   engine taps one untapped red creature you control, source excluded), but
//!   X = "the power of the creature tapped this way" is uncomputable:
//!   ActivationContext exposes no accessor for the creature tapped as a cost,
//!   so the +X/+0 effect body is GAP'd rather than hardcoding a literal where
//!   the oracle is dynamic.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Impelled Giant");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let red_creature = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::red());

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped red creature you control other than this creature: This creature gets +X/+0 until end of turn, where X is the power of the creature tapped this way.".into(),
                cost: ActivationCost {
                    tap_other: Some(red_creature),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_by_tapped,
            }),
    )
}

fn pump_by_tapped(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: +X/+0 where X is the power of the creature tapped as the cost — no
    // ActivationContext accessor for the tapped-as-cost creature; the dynamic
    // amount is uncomputable, so the effect body is omitted rather than
    // hardcoded.
    Vec::new()
}
