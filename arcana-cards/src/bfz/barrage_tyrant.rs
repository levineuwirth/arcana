//! Barrage Tyrant — `{4}{R}` 5/3 colorless Eldrazi (Devoid).
//! "{2}{R}, Sacrifice another colorless creature: This creature deals
//!  damage equal to the sacrificed creature's power to any target."
//!
//! Devoid → the card is colorless despite the {R} pip.
//! The cost (mana + sacrifice another colorless creature) and the any-
//! target requirement are expressed; the damage amount is GAP'd — there
//! is no accessor for the sacrificed creature's power at resolution, and
//! the amount is dynamic so a literal would be a materially wrong card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barrage Tyrant");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{R}, Sacrifice another colorless creature: This creature deals damage equal to the sacrificed creature's power to any target.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                sacrifice_other: Some(
                    ObjectFilter::creature().without_colors(
                        ColorSet::white()
                            | ColorSet::blue()
                            | ColorSet::black()
                            | ColorSet::red()
                            | ColorSet::green(),
                    ),
                ),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::any_target()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: barrage,
        }),
    )
}

fn barrage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage = sacrificed creature's power. No accessor exposes the
    // sacrificed-as-cost creature at resolution, so the dynamic amount is
    // unexpressible (a literal would be materially wrong).
    Vec::new()
}
