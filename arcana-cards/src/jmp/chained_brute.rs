//! Chained Brute — `{1}{R}` 4/3 Devil.
//! "This creature doesn't untap during your untap step." (static)
//! "{1}, Sacrifice another creature: Untap this creature. Activate only during
//! your turn."
//!
//! Decomposition:
//! * "Doesn't untap during your untap step" — a pure static untap restriction
//!   with no trigger/cost; GAP'd.
//! * {1}, Sacrifice another creature → untap this — mana + `sacrifice_other`
//!   cost, untaps the source. ("Activate only during your turn" timing
//!   restriction is a fidelity gap; not modeled in this shape.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chained Brute");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "doesn't untap during your untap step" — static untap restriction.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, Sacrifice another creature: Untap this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
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
            effect: untap_self,
        }),
    )
}

fn untap_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
